use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;

use common::error::Result;
use common::event::Event;

pub struct FileSource {
    pub path: PathBuf,
    reader: BufReader<File>,
}

impl FileSource {
    pub fn new(path: PathBuf, position: Option<u64>) -> Result<Self> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        if let Some(pos) = position {
            reader.seek(SeekFrom::Start(pos))?;
        }
        Ok(FileSource { path, reader })
    }

    pub fn read_line(&mut self) -> Result<Option<Event>> {
        let mut line = String::new();
        let len = self.reader.read_line(&mut line)?;
        if len == 0 {
            return Ok(None);
        }
        let event = Event::new(self.path.to_string_lossy().to_string(), line.into_bytes())?;
        Ok(Some(event))
    }
}

impl Iterator for FileSource {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_line() {
            Ok(Some(event)) => Some(Ok(event)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
