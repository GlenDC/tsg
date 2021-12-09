use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;

use super::{File, FileFormat};
use super::data::{Value, ValueIter};
use super::path::{PathIter, PathComponent};

use anyhow::{anyhow, Result};

pub struct Workspace {
    pages: FileEntry,
    layouts: FileEntry,
    includes: FileEntry,
}

impl Workspace {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Workspace> {
        let path = path.as_ref();

        let pages = load_files(
            path.join("pages"),
            &|file| match file.info().format() {
                FileFormat::Html | FileFormat::Markdown | FileFormat::Rhai => true,
                _ => false,
            })?;

        let layouts = load_files(
            path.join("layouts"),
            &|file| match file.info().format() {
                FileFormat::Html => true,
                _ => false,
            })?;

        let includes = load_files(
            path.join("includes"),
            &|_| true,
        )?;

        Ok(Workspace::new(pages, layouts, includes))
    }

    pub fn new(pages: FileEntry, layouts: FileEntry, includes: FileEntry) -> Workspace {
        return Workspace {
            pages,
            layouts,
            includes,
        };
    }
}

pub enum FileEntry {
    File(File),
    Dir(HashMap<String, FileEntry>),
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
    path: Vec<PathComponent<'b>>,
    entry_ref: &'a FileEntry,
    path_index: usize,
    recursive: bool,
}

impl FileEntry {
    pub fn file_entry_or_value<'a, 'b, T>(&'a self, t: T) -> Option<FileOrValue<'a>>
    where T: Into<PathIter<'b>>
    {
        self.file_entry_or_value_iter(t).next()
    }

    pub fn file_entry_or_value_iter<'a, 'b, T>(&'a self, t: T) -> FileOrValueIter<'a, 'b>
    where T: Into<PathIter<'b>>
    {
        FileOrValueIter::new(self, t)
    }
}

fn load_files<P: AsRef<Path>>(dir: P, filter: &dyn Fn(&File) -> bool) -> Result<FileEntry> {
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
                Some(dir_name) => files.insert(String::from(dir_name), dir),
                None => return Err(anyhow!("failed to get dirname for dir entry")),
            };
        } else {
            let file = File::new(path)?;
            if filter(&file) {
                // TODO: is this clone really needed?
                files.insert(String::from(file.info().name()), FileEntry::File(file));
            }
        }
    }

    Ok(FileEntry::Dir(files))
}

pub struct FileOrValueIter<'a, 'b> {
    stack: VecDeque<FileOrValueIterInner<'a, 'b>>,
}

struct FileOrValueIterInner<'a, 'b> {
    state: FileEntryOrValueInnerState<'a, 'b>,
}

impl<'a, 'b> FileOrValueIter<'a, 'b> {
    pub fn new<T>(entry: &'a FileEntry, path_iter: T) -> FileOrValueIter<'a, 'b>
    where
        T: Into<PathIter<'b>>,
    {
        let root_value_iter = FileOrValueIterInner::new(FileEntryOrValueInnerState::FileEntry(FileEntryState{
            path: path_iter.into().collect(),
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
        FileOrValueIterInner {
            state,
        }
    }

    fn next_value(&mut self, stack: &mut VecDeque<FileOrValueIterInner<'a, 'b>>) -> Option<FileOrValue<'a>> {
        None
    }
}
