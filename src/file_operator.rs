use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
};

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

        let file = if !self.overwrite {
            // Attempt to open the file in append mode; if the file does not exist, create it.
            OpenOptions::new()
                .append(true)
                .create(true) // Create the file if it does not exist
                .open(&self.path)
                .expect("Unable to open or create file")
        } else {
            // If overwriting is needed, directly create the file. This will clear the file if it already exists.
            File::create(&self.path).unwrap()
        };

        // Create buffer writer
        self.writer = Some(BufWriter::new(file));
        self.is_open = true;
    }

    // Write string data to file
    pub fn write(&mut self, data: &str) -> io::Result<()> {
        self.write_byte(data.as_bytes())
    }

    // Write byte data to file
    // This is the real write function to write data to file
    fn write_byte(&mut self, data: &[u8]) -> io::Result<()> {
        if !self.is_open {
            self.ready();
        }

        match &mut self.writer {
            Some(writer) => writer.write_all(data),
            None => {
                self.close();
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The file is not ready to write",
                ))
            }
        }
    }
}
