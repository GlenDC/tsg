use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use super::path::{PathComponent, PathIter};
use super::{File, FileFormat, FileInfo};
use super::{Value, ValueIter};

use anyhow::{anyhow, Result};

pub struct Workspace {
    root: PathBuf,
    assets: Vec<PathBuf>,
    pages: FileEntry,
    layouts: FileEntry,
    includes: FileEntry,
}

impl Workspace {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Workspace> {
        let path = path.as_ref();

        let assets = list_files(path.join("assets"))?;

        let pages = load_files(path.join("pages"), &|file_info| match file_info.format() {
            FileFormat::Html | FileFormat::Markdown | FileFormat::Rhai => true,
            _ => false,
        })?;

        let layouts = load_files(path.join("layouts"), &|file_info| match file_info.format() {
            FileFormat::Html => true,
            _ => false,
        })?;

        let includes = load_files(path.join("includes"), &|_| true)?;

        Ok(Workspace {
            root: PathBuf::from(path),
            assets,
            pages,
            layouts,
            includes,
        })
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    pub fn assets(&self) -> &[PathBuf] {
        &self.assets[..]
    }

    pub fn page_or_value<'a, 'b, T>(&'a mut self, t: T) -> Option<FileOrValue<'a>>
    where
        T: Into<PathIter<'b>>,
    {
        self.page_or_value_iter(t).next()
    }

    pub fn page_or_value_iter<'a, 'b, T>(&'a mut self, t: T) -> FileOrValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        FileOrValueIter::new(&mut self.pages, t)
    }

    pub fn layout_or_value<'a, 'b, T>(&'a mut self, t: T) -> Option<FileOrValue<'a>>
    where
        T: Into<PathIter<'b>>,
    {
        self.layout_or_value_iter(t).next()
    }

    pub fn layout_or_value_iter<'a, 'b, T>(&'a mut self, t: T) -> FileOrValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        FileOrValueIter::new(&mut self.layouts, t)
    }

    pub fn include_or_value<'a, 'b, T>(&'a mut self, t: T) -> Option<FileOrValue<'a>>
    where
        T: Into<PathIter<'b>>,
    {
        self.include_or_value_iter(t).next()
    }

    pub fn include_or_value_iter<'a, 'b, T>(&'a mut self, t: T) -> FileOrValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        FileOrValueIter::new(&mut self.includes, t)
    }
}

enum FileEntry {
    File(LazyFile),
    Dir(HashMap<String, FileEntry>),
}

enum LazyFile {
    FileInfo(FileInfo),
    File(File),
}

impl LazyFile {
    pub fn read_or_get_file(&mut self) -> Result<&File> {
        match self {
            LazyFile::File(file) => Ok(file),
            LazyFile::FileInfo(info) => {
                let file = info.clone().try_into()?;
                *self = LazyFile::File(file);
                self.read_or_get_file()
            },
        }
    }
}

pub enum FileOrValue<'a> {
    File(&'a File),
    Value(&'a Value),
}

enum FileEntryOrValueInnerState<'a, 'b> {
    None,
    FileEntry(FileEntryState<'a, 'b>),
    ValueIter(ValueIter<'a, 'b>),
}

struct FileEntryState<'a, 'b> {
    pub path: Vec<PathComponent<'b>>,
    pub entry_ref: &'a mut FileEntry,
    pub path_index: usize,
    pub recursive: bool,
}

fn load_files<P: AsRef<Path>>(dir: P, filter: &dyn Fn(&FileInfo) -> bool) -> Result<FileEntry> {
    let mut files = HashMap::new();

    let dir = dir.as_ref();
    if !dir.exists() {
        return Ok(FileEntry::Dir(files));
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir = load_files(&path, filter)?;
            match path.file_name().and_then(|n| n.to_str()) {
                Some(dir_name) => files.insert(dir_name.to_lowercase(), dir),
                None => return Err(anyhow!("failed to get dirname for dir entry")),
            };
        } else {
            let file_info: FileInfo = (&path).try_into()?;
            if filter(&file_info) {
                files.insert(file_info.name().to_lowercase(), FileEntry::File(LazyFile::FileInfo(file_info)));
            }
        }
    }

    Ok(FileEntry::Dir(files))
}

fn list_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    let root = dir.as_ref();
    if !root.exists() {
        return Ok(file_paths);
    }

    let mut dirs_to_read = vec![PathBuf::from(root)];
    loop {
        match dirs_to_read.pop() {
            None => break,
            Some(dir) => { 
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_read.push(PathBuf::from(path));
                    } else {
                        let path = path.strip_prefix(root)?;
                        file_paths.push(PathBuf::from(path));
                    }
                }
            }
        }
    }

    Ok(file_paths)
}

pub struct FileOrValueIter<'a, 'b> {
    stack: VecDeque<FileOrValueIterInner<'a, 'b>>,
}

struct FileOrValueIterInner<'a, 'b> {
    state: FileEntryOrValueInnerState<'a, 'b>,
}

impl<'a, 'b> FileOrValueIter<'a, 'b> {
    fn new<T>(entry: &'a mut FileEntry, t: T) -> FileOrValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        let root_value_iter =
            FileOrValueIterInner::new(FileEntryOrValueInnerState::FileEntry(FileEntryState {
                path: t.into().collect(),
                path_index: 0,
                entry_ref: entry,
                recursive: false,
            }));
        let mut stack = VecDeque::with_capacity(1);
        stack.push_front(root_value_iter);
        FileOrValueIter { stack }
    }
}

impl<'a, 'b> Iterator for FileOrValueIter<'a, 'b> {
    type Item = FileOrValue<'a>;

    fn next(&mut self) -> Option<FileOrValue<'a>> {
        let mut inner_stack = VecDeque::new();
        loop {
            if self.stack.is_empty() {
                return None;
            }
            let result = self.stack[0].next_value(&mut inner_stack);
            if inner_stack.len() > 0 {
                self.stack.append(&mut inner_stack);
            }
            match result {
                None => {
                    self.stack.pop_front();
                    continue;
                }
                Some(value) => return Some(value),
            }
        }
    }
}

impl<'a, 'b> FileOrValueIterInner<'a, 'b> {
    pub fn new(state: FileEntryOrValueInnerState<'a, 'b>) -> FileOrValueIterInner<'a, 'b> {
        FileOrValueIterInner { state }
    }

    fn next_value(
        &mut self,
        stack: &mut VecDeque<FileOrValueIterInner<'a, 'b>>,
    ) -> Option<FileOrValue<'a>> {
        let state = std::mem::replace(&mut self.state, FileEntryOrValueInnerState::None);
        match state {
            FileEntryOrValueInnerState::None => None,
            FileEntryOrValueInnerState::ValueIter(mut it) => match it.next() {
                None => {
                    self.state = FileEntryOrValueInnerState::None;
                    None
                }
                Some(value) => {
                    self.state = FileEntryOrValueInnerState::ValueIter(it);
                    return Some(FileOrValue::Value(value));
                }
            },
            FileEntryOrValueInnerState::FileEntry(mut state) => {
                while state.path_index >= state.path.len() {
                    match state.path[state.path_index] {
                        PathComponent::Name(name) => match state.entry_ref {
                            FileEntry::File(file) => match file.read_or_get_file().ok().and_then(|file| file.meta()) {
                                None => return None,
                                Some(meta) => {
                                    let mut path = Vec::new();
                                    if state.recursive {
                                        path.push(PathComponent::AnyRecursive);
                                    }
                                    path.extend(state.path.into_iter().skip(state.path_index));
                                    let value_it =
                                        meta.value_iter(PathIter::wrap(path.into_iter()));
                                    stack.push_back(FileOrValueIterInner::new(
                                        FileEntryOrValueInnerState::ValueIter(value_it),
                                    ));
                                    return None;
                                }
                            },
                            FileEntry::Dir(map) => {
                                let name = name.to_lowercase();
                                if !state.recursive {
                                    match map.get_mut(&name) {
                                        None => return None,
                                        Some(entry) => {
                                            state.entry_ref = entry;
                                            state.path_index += 1;
                                            state.recursive = false;
                                        }
                                    }
                                } else {
                                    for (entry_name, entry) in map.iter_mut() {
                                        if entry_name == &name {
                                            stack.push_front(FileOrValueIterInner::new(
                                                FileEntryOrValueInnerState::FileEntry(
                                                    FileEntryState {
                                                        path: state.path[state.path_index + 1..]
                                                            .to_vec(),
                                                        entry_ref: entry,
                                                        path_index: 0,
                                                        recursive: false,
                                                    },
                                                ),
                                            ));
                                        } else {
                                            stack.push_back(FileOrValueIterInner::new(
                                                FileEntryOrValueInnerState::FileEntry(
                                                    FileEntryState {
                                                        path: state.path[state.path_index..]
                                                            .to_vec(),
                                                        entry_ref: entry,
                                                        path_index: 0,
                                                        recursive: true,
                                                    },
                                                ),
                                            ));
                                        }
                                    }
                                    return None;
                                }
                            }
                        },
                        PathComponent::Any => match state.entry_ref {
                            FileEntry::File(file) => match file.read_or_get_file().ok().and_then(|file| file.meta()) {
                                None => {
                                    self.state = FileEntryOrValueInnerState::None;
                                    return None;
                                }
                                Some(meta) => {
                                    let it = state.path.into_iter().skip(state.path_index);
                                    let value_it = meta.value_iter(PathIter::wrap(it));
                                    stack.push_back(FileOrValueIterInner::new(
                                        FileEntryOrValueInnerState::ValueIter(value_it),
                                    ));
                                    return None;
                                }
                            },
                            FileEntry::Dir(map) => {
                                for entry in map.values_mut() {
                                    stack.push_back(FileOrValueIterInner::new(
                                        FileEntryOrValueInnerState::FileEntry(FileEntryState {
                                            path: state.path[state.path_index + 1..].to_vec(),
                                            entry_ref: entry,
                                            path_index: 0,
                                            recursive: false,
                                        }),
                                    ));
                                }
                                return None;
                            }
                        },
                        PathComponent::AnyRecursive => match state.entry_ref {
                            FileEntry::File(file) => match file.read_or_get_file().ok().and_then(|file| file.meta()) {
                                None => {
                                    return None;
                                }
                                Some(meta) => {
                                    let it = state.path.into_iter().skip(state.path_index);
                                    let value_it = meta.value_iter(PathIter::wrap(it));
                                    stack.push_back(FileOrValueIterInner::new(
                                        FileEntryOrValueInnerState::ValueIter(value_it),
                                    ));
                                    return None;
                                }
                            },
                            FileEntry::Dir(map) => {
                                for entry in map.values_mut() {
                                    stack.push_back(FileOrValueIterInner::new(
                                        FileEntryOrValueInnerState::FileEntry(FileEntryState {
                                            path: state.path[state.path_index + 1..].to_vec(),
                                            entry_ref: entry,
                                            path_index: 0,
                                            recursive: true,
                                        }),
                                    ));
                                }
                                return None;
                            }
                        },
                    }
                }
                None
            }
        }
    }
}
