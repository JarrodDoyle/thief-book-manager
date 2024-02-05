use std::{ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Book {
    pub path: PathBuf,
    pub file_name: String,
}

#[derive(Debug, Clone)]
pub struct ProjectState {
    pub path: PathBuf,
    pub books: Vec<Book>,
}

impl ProjectState {
    pub fn new(path: PathBuf) -> Self {
        let mut books = vec![];
        let book_walker = WalkDir::new(path.join("books")).into_iter();
        for entry in book_walker.flatten() {
            if entry.metadata().is_ok_and(|md| md.is_dir()) {
                continue;
            }

            let extension = entry.path().extension().and_then(OsStr::to_str);
            if extension.is_some_and(|ext| ext.to_lowercase() == "str") {
                books.push(Book {
                    path: entry.path().to_owned(),
                    file_name: entry.file_name().to_str().unwrap().to_owned(),
                });
            }
        }

        Self { path, books }
    }
}
