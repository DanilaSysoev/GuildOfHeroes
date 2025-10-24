pub mod map_gen;

use std::fs;
use std::io;
use std::path::Path;

pub fn ensure_parent_dirs<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    if let Some(parent) = file_path.as_ref().parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
