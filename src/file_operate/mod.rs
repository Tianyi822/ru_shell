use std::{fs::File, io::BufWriter};

struct FileOperator {
    // The buffer writer for file
    writer: BufWriter<File>,

    // The status of file: open or close
    // If the file is closed, the writer will be invalid
    // If the file is open, the writer will be valid
    is_open: bool,

    // The flag to indicate whether the file need to be compressed
    need_compress: bool,

    // The max size of file
    max_size: u32,

    // The path of file
    path: String,
}

// The basic configuration for file operator
pub struct FileConfig {
    path: String,
    need_compress: bool,
    max_size: u32,
}

impl From<FileConfig> for FileOperator {
    fn from(config: FileConfig) -> Self {
        // Create file
        let file = match File::create(config.path.clone()) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create file: {}", e),
        };
        // Create buffer writer
        let writer = BufWriter::new(file);

        FileOperator {
            writer,
            is_open: true,
            need_compress: config.need_compress,
            max_size: config.max_size,
            path: config.path,
        }
    }
}
