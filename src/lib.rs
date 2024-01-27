use pdfium_render::prelude::*;
use std::{
    env,
    ffi::OsStr,
    fmt, fs, io,
    path::{Path, PathBuf},
};

const TMP_DIR: &str = "label-fixer";

#[derive(thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("{0}")]
    Other(&'static str),

    #[error(transparent)]
    Pdfium(#[from] PdfiumError),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub fn fix_label(label_path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let label_path = label_path.as_ref();
    if label_path.extension() != Some(OsStr::new("pdf")) {
        return Err(Error::Other("must be a file ending in .pdf"));
    }

    let out_path = {
        let mut out_path = env::temp_dir();
        out_path.push(TMP_DIR);
        fs::create_dir_all(&out_path)?;
        out_path.push(
            label_path
                .file_stem()
                .ok_or(Error::Other("could not parse file name"))?,
        );
        out_path.set_extension("png");
        out_path
    };

    let pdfium = Pdfium::default();
    let first_page = pdfium
        .load_pdf_from_file(label_path, None)?
        .pages()
        .first()?;
    let label = first_page
        .objects()
        .first()?
        .as_image_object()
        .ok_or(Error::Other("failed to find label"))?
        .get_raw_image()?;
    label
        .save(&out_path)
        .map_err(|_| Error::Other("failed to save label"))?;

    Ok(out_path)
}
