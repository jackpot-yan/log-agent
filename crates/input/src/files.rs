use common::error::Result;

pub struct FileSource {
    pub path: String,
    pub position: u64,
}
impl FileSource {
    pub fn new(path: String, position: u64) -> Result<Self> {
        Ok(FileSource { path, position })
    }
}
