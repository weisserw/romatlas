//! Protect the source image when writing analysis files.
use anyhow::{Result, ensure};
use std::{io, path::Path};

pub fn distinct_from_input(input: &Path, output: &Path) -> Result<()> {
    let original = match std::fs::metadata(input) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    let target = match std::fs::metadata(output) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    ensure!(
        std::fs::canonicalize(input)? != std::fs::canonicalize(output)?,
        "output aliases the input ROM"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ensure!(
            original.dev() != target.dev() || original.ino() != target.ino(),
            "output aliases the input ROM"
        );
    }
    #[cfg(not(unix))]
    let _ = (original, target);
    Ok(())
}
