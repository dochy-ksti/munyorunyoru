use std::path::PathBuf;

pub struct Data<T> {
    pub path: PathBuf,
    pub items: Vec<T>,
}

impl<T> Data<T> {
    pub fn new(path: PathBuf, items: Vec<T>) -> Self { Self { path, items } }
}
