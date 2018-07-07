use juniper::FieldResult;

use model::{Context, EntryData, FileData, FolderData, Mutation, Query};
use traits::{Entry, File, Folder};
use util::from_virtual_path;

impl juniper::Context for Context {}

graphql_object!(FolderData: Context as "Folder" |&self| {
    interfaces: [EntryData]

    field name() -> Option<&str> as "Name of the folder" {
        self.name()
    }

    field is_folder() -> bool {
        !self.path().is_file()
    }

    field is_file() -> bool {
        self.path().is_file()
    }

    field entries() -> FieldResult<Vec<EntryData>> {
        Ok(self.entries()?)
    }

    field absolute_path(&executor) -> Option<String> {
        self.virtual_absolute_path(&executor.context().root_path)
    }
});

graphql_object!(FileData: Context as "File" |&self| {
    interfaces: [EntryData]

    field name() -> Option<&str> as "Name of the file" {
        self.name()
    }

    field is_folder() -> bool {
        !self.path().is_file()
    }

    field is_file() -> bool {
        self.path().is_file()
    }

    field absolute_path(&executor) -> Option<String> {
        self.virtual_absolute_path(&executor.context().root_path)
    }

    field content() -> FieldResult<String> as "Complete content of the file" {
        Ok(self.content()?)
    }
});

graphql_interface!(EntryData: Context as "Entry" |&self| {
    field name() -> Option<&str> as "Name of the entry" {
        self.name()
    }

    field absolute_path(&executor) -> Option<String> {
        self.virtual_absolute_path(&executor.context().root_path)
    }

    field is_folder() -> bool {
        !self.path().is_file()
    }

    field is_file() -> bool {
        self.path().is_file()
    }

    instance_resolvers: |_| {
        FileData => {
            if self.path().is_file() {
                Some(FileData {
                    path: self.path.clone()
                })
            } else {
                None
            }
        },
        FolderData => {
            if self.path().is_dir() {
                Some(FolderData {
                    path: self.path.clone()
                })
            } else {
                None
            }
        }
    }
});

graphql_object!(Query: Context |&self| {
    field api_version() -> &str as "Current version of the API" {
        "1.0"
    }

    field root(&executor) -> FolderData as "Root folder" {
        FolderData {
            path: executor.context().root_path.clone()
        }
    }

    field file(&executor, path: String) -> Option<FileData> {
        let abs_path = from_virtual_path(&executor.context().root_path, &path)?;
        if abs_path.is_file() {
            Some(FileData {
                path: abs_path
            })
        } else {
            None
        }
    }

    field folder(&executor, path: String) -> Option<FolderData> {
        let abs_path = from_virtual_path(&executor.context().root_path, &path)?;
        if !abs_path.is_file() {
            Some(FolderData {
                path: abs_path
            })
        } else {
            None
        }
    }
});

graphql_object!(Mutation: Context | &self | {});
