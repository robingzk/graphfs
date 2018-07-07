use std::path::PathBuf;

use traits::*;

pub struct Query {}

pub struct Mutation {}

pub struct Context {
    pub root_path: PathBuf,
}

pub struct EntryData {
    pub path: PathBuf,
}

pub struct FileData {
    pub path: PathBuf,
}

pub struct FolderData {
    pub path: PathBuf,
}

impl Entry for EntryData {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Entry for FolderData {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Entry for FileData {
    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Folder for FolderData {}
impl File for FileData {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
