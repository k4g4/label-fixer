use std::{
    env::temp_dir,
    ffi::OsStr,
    fmt::{self, Debug},
    fs::{copy, create_dir_all},
    io,
    path::{Path, PathBuf},
};

const TMP_DIR: &str = "label-fixer";

pub enum Error {
    Io(io::Error),
    Other(&'static str),
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Other(e) => f.write_str(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub fn fix_label(label_path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let label_path = label_path.as_ref();
    if label_path.extension() != Some(OsStr::new("pdf")) {
        return Err(Error::Other("must be a file ending in .pdf"));
    }
    let mut out_path = temp_dir();
    create_dir_all(TMP_DIR)?;
    out_path.push(TMP_DIR);
    out_path.push(
        label_path
            .file_name()
            .ok_or(Error::Other("could not parse file name"))?,
    );
    copy(label_path, &out_path)?;
    Ok(out_path)
}
