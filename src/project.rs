use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Book {
    pub path: PathBuf,
    pub file_name: String,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectState {
    pub path: PathBuf,
    pub books: HashMap<String, Book>,
}

impl ProjectState {
    pub fn new(path: PathBuf) -> Self {
        let search_path = path.join("books");
        let mut books = HashMap::new();
        let book_walker = WalkDir::new(search_path.clone()).into_iter();
        for entry in book_walker.flatten() {
            if entry.metadata().is_ok_and(|md| md.is_dir()) {
                continue;
            }

            let path = entry.path();
            let extension = path.extension().and_then(OsStr::to_str);
            if extension.is_some_and(|ext| ext.to_lowercase() == "str") {
                let language = file_name_string(path.parent().unwrap()).to_lowercase();
                let file_name = file_name_string(path).to_lowercase();
                books
                    .entry(file_name.clone())
                    .and_modify(|b: &mut Book| b.languages.push(language.clone()))
                    .or_insert(Book {
                        path: path.to_owned(),
                        file_name,
                        languages: vec![language],
                    });
            }
        }

        Self { path, books }
    }
}

fn file_name_string(path: &Path) -> String {
    let file_name = path.file_name();
    file_name.map(|s| s.to_str().unwrap().to_owned()).unwrap()
}
