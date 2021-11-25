use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::{File, FileFormat};

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

pub enum FileEntryGetResult<'a> {
    File(&'a File),
    Dir(&'a HashMap<String, FileEntry>),
    Value,
}

impl FileEntry {
    pub fn get(&self, path: &str) -> Result<FileEntryGetResult> {
        let mut it = path.split(".").into_iter();
        let mut entry = self;
        loop {
            match it.next() {
                None => return Ok(match entry {
                    FileEntry::File(file) => FileEntryGetResult::File(&file),
                    FileEntry::Dir(files) => FileEntryGetResult::Dir(&files),
                }),
                Some(component) => {
                    match entry {
                        FileEntry::File(_file) => {
                            // TODO: given we'll anyway split again to get meta value,
                            // can we perhaps do without having to go to a string again first?!
                            let _path = it.fold(String::new(), |acc, s| format!("{}.{}", acc, s));
                            return Ok(FileEntryGetResult::Value);  // TODO: actually get the metadata
                        },
                        FileEntry::Dir(files) => match files.get(component) {
                            Some(found_entry) => entry = found_entry,
                            None => return Err(anyhow!("not found")), // TODO: return a typed error
                        }
                    };
                },
            }
        }
        
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
