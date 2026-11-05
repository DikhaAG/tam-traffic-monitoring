use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct LogTailer {
    file_path: PathBuf,
    max_lines: usize,
}

impl LogTailer {
    pub fn new(file_path: PathBuf, max_lines: usize) -> Self {
        Self {
            file_path,
            max_lines,
        }
    }

    pub fn read_latest_lines(&self) -> Result<Vec<String>, std::io::Error> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        let mut ring_buffer = std::collections::VecDeque::with_capacity(self.max_lines);

        for line in reader.lines() {
            let l = line?;
            if ring_buffer.len() == self.max_lines {
                ring_buffer.pop_front();
            }
            ring_buffer.push_back(l);
        }

        Ok(ring_buffer.into_iter().collect())
    }
}

