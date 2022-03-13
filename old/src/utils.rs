use std::{fs, io, path::Path};

// todo: write unit tests

// https://stackoverflow.com/a/65192210
pub(crate) fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub(crate) fn is_using_gitpod() -> bool {
    Path::new("/ide/bin/gp-code").exists() || std::env::var("GITPOD_REPO_ROOT").is_ok()
}
