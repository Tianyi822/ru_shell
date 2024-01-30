mod file_test {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[test]
    fn test_read_file() {
        let file = File::open("Cargo.toml").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        println!("{}", contents);
    }

    #[test]
    fn test_file_read_line() {
        let file = File::open("Cargo.toml").unwrap();
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
