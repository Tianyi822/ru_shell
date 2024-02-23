use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    rc::Rc,
};

use dirs_next::home_dir;

use crate::{executor, file_operator::FileOperator, stream::{console_stream::ConsoleStream, Stream}};

pub fn run() {
    // Create history file
    let user_home_path = match home_dir() {
        Some(path) => path.to_str().unwrap().to_string(),
        None => {
            println!("Unable to get home directory");
            return;
        }
    };
    let mut history_file = FileOperator::new(
        &format!("{}/.rusty_shell_history", user_home_path),
        false,
        1024,
    );

    // Print the logo
    let file = File::open("asset/logo.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut logo = String::new();
    buf_reader.read_to_string(&mut logo).unwrap();
    println!("{}", logo);
    println!("Welcome to the Rusty Shell!");

    // Create a stream for the console
    let console_stream = Rc::new(ConsoleStream::new());

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Write the input to history file
        match history_file.write(&input) {
            Ok(_) => {}
            Err(e) => {
                println!("Unable to write to history file: {}", e);
            }
        }

        if input.trim() == "exit" {
            break;
        }

        executor::execute(&input, console_stream.clone());

        console_stream.output();
    }
}
