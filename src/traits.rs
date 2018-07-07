use std::fs::read_to_string;
use std::io::Result as IoResult;
use std::path::PathBuf;

use model::*;
use util::to_virtual_path;

///
/// Represents an entry: either a file or a folder.
///
/// To implement this trait, objects only need to implement the function
/// `path(&self)` which returns the path to the entry.
///
pub trait Entry {
    // The actual path in the filesystem to this entry.
    fn path(&self) -> &PathBuf;

    /// Return the name of the entry, or None if it is not defined.
    fn name(&self) -> Option<&str> {
        self.path().file_name()?.to_str()
    }

    /// Return the path to this entry relative to the root,
    /// formatted as if it were absolute.
    fn virtual_absolute_path(&self, root: &PathBuf) -> Option<String> {
        to_virtual_path(root, self.path())
    }
}

///
/// A folder represents an entry that contains nested folders or files.
///
pub trait Folder: Entry {
    /// Return a vector over the entries in the folder.
    ///
    /// # Error
    /// Could fail if the path does not refer to an existing directory or due
    /// to a filesystem error.
    fn entries(&self) -> IoResult<Vec<EntryData>> {
        self.path()
            .read_dir()?
            .map(|entry| {
                Ok(EntryData {
                    path: entry?.path(),
                })
            })
            .collect()
    }
}

///
/// A file reprsents an entry with a content that can be read and modified.
///
pub trait File: Entry {
    fn content(&self) -> IoResult<String> {
        read_to_string(&self.path())
    }
}
