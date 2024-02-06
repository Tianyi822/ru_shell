use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use crate::executor::*;

pub fn run() {
    // Print the logo
    let file = File::open("asset/logo.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut logo = String::new();
    buf_reader.read_to_string(&mut logo).unwrap();
    println!("{}", logo);
    println!("");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        executor::execute(&input)
    }
}
