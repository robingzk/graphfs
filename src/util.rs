use relative_path::RelativePath;
use std::fs::canonicalize;
use std::path::PathBuf;

/// Resolve the path of a file given its location relative to a root path.
///
/// # Example
/// ```
/// let root = std::path::PathBuf::from("/home");
/// let result = graphfs::util::from_virtual_path(&root, "/hello.txt").unwrap();
/// assert_eq!(result.to_str(), Some("/home/hello.txt"));
/// ```
///
pub fn from_virtual_path(root: &PathBuf, rel_path: &str) -> Option<PathBuf> {
    Some(RelativePath::new(rel_path).to_path(root))
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
            (None, None) => break,
            (None, Some(_)) => break,
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
