use crate::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Paths {
    pub paths: Vec<Path>,
}

impl Paths {
    pub fn new(paths: Vec<Path>) -> Self {
        Self { paths }
    }
}

impl From<Vec<Path>> for Paths {
    fn from(paths: Vec<Path>) -> Self {
        Self { paths }
    }
}
