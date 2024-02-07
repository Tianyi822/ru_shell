use std::{fs::File, io::BufWriter};
use std::io::Write;

pub struct FileOperator {
    // The buffer writer for file
    writer: Option<BufWriter<File>>,

    // The status of file: open or close
    // If the file is closed, the writer will be invalid
    // If the file is open, the writer will be valid
    pub is_open: bool,

    // If file exists, the flag to indicate whether the file need to be covered
    pub overwrite: bool,

    // The flag to indicate whether the file need to be compressed
    pub need_compress: bool,

    // The max size of file
    pub max_size: u32,

    // The path of file
    pub path: String,
}

impl FileOperator {
    pub fn new(path: &str, overwrite: bool, need_compress: bool, max_size: u32) -> FileOperator {
        FileOperator {
            writer: None,
            is_open: false,
            overwrite,
            need_compress,
            max_size,
            path: path.to_string(),
        }
    }

    // Because the file may be not used immediately, a field to indicate whether the file is ready.
    // This is done to reduce memory usage when the file is not used.
    pub fn ready(&mut self) {
        if self.is_open {
            return;
        }

        // Open file
        let file = match File::open(&self.path) {
            Ok(f) => {
                if !self.overwrite {
                    f
                } else {
                    File::create(&self.path).unwrap()
                }
            }
            Err(_) => File::create(&self.path).unwrap(),
        };

        // Create buffer writer
        self.writer = Some(BufWriter::new(file));
        self.is_open = true;
    }


    // Write data to file
    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // Flush and drop the writer
        if self.is_open {
            self.writer.as_mut().unwrap().flush()?;
            self.writer = None;
        }

        // Set the status of file to close
        self.is_open = false;

        Ok(())
    }
}
