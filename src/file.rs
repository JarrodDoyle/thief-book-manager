use std::{path::PathBuf, sync::Arc};

use crate::Error;

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;
    Ok((path, content))
}

pub async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(file_handle.path().to_owned()).await
}

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}
