use relative_path::RelativePath;
use std::fs::canonicalize;
use std::path::PathBuf;

/// Resolve the path of a file given its location relative to a root path, and
/// verify that the path is contained within the root directory.
///
pub fn from_virtual_path(root: &PathBuf, rel_path: &str) -> Option<PathBuf> {
    let path = RelativePath::new(rel_path).to_path(root);
    if is_path_contained(&root, &path) {
        Some(path)
    } else {
        None
    }
}

/// Check if `root` contains the path `other`, potentially into one of its
/// sub-folders.
///
/// # Example
/// ```
/// let root = std::path::PathBuf::from("/home");
/// let other = std::path::PathBuf::from("/lol");
/// assert_eq!(graphfs::util::is_path_contained(&root, &other), false);
/// ```
///
///
pub fn is_path_contained(root: &PathBuf, other: &PathBuf) -> bool {
    match (canonicalize(root), canonicalize(other)) {
        (Ok(root), Ok(path)) => {
            let mut root_components = root.components();
            let mut path_components = path.components();
            loop {
                match (root_components.next(), path_components.next()) {
                    (None, None) | (None, Some(_)) => break,
                    (Some(x), Some(y)) => {
                        if x != y {
                            return false;
                        }
                    },
                    (Some(_), None) => return false
                }
            }
            true
        },
        _ => false
    }
}

/// Resolve the virtual location of a path relative to a root folder.
/// Opposite operation of `from_virtual_path`.
///
/// This function will return `None` if the path does not exist, or is not
/// contained in the root.
///
/// # Example
/// Note that this example actually require the folder `/usr/bin` to actually exist.
/// ```
/// let mut root = std::path::PathBuf::from("/usr");
/// let mut path = std::path::PathBuf::from("/usr/bin");
/// assert_eq!(graphfs::util::to_virtual_path(&root, &path), Some("/bin".to_string()));
/// ```
///
pub fn to_virtual_path(root: &PathBuf, path: &PathBuf) -> Option<String> {
    let canonicalized_root = canonicalize(root).ok()?;
    let canonicalized_path = canonicalize(path).ok()?;
    let mut root_components = canonicalized_root.components();
    let mut path_components = canonicalized_path.components().peekable();
    // Remove the root path from `path_components`
    loop {
        match (root_components.next(), path_components.peek()) {
            (None, None) | (None, Some(_)) => break,
            (Some(x), Some(y)) => {
                if x != *y {
                    return None;
                }
            }
            (Some(_), None) => return None,
        }
        path_components.next();
    }

    // Collect the remaining components from `path_components`
    let mut new_path = PathBuf::new();
    while let Some(comp) = path_components.next() {
        new_path.push(comp);
    }
    Some(format!("/{}", new_path.to_str()?))
}
