
/*
use std::fs::File;
use std::io::Write;

fn create_and_write_to_file() {
    let mut file = File::create("example.txt").unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}

fn main() {
    create_and_write_to_file();
    println!("File created and written successfully.");
} */


/*
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
}
*/



use std::io::{self, Read, Write};

#[derive(Debug)
struct Student {
    name: String,
    major: String,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What's your major? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let student = buffer.trim().parse().unwrap();

    let student = Student { name, major };
    println!("Hi {}, your classification is: {} !", student.name, student.major);
}

fn main(){
    reading_from_console();
} 

use std::fs::File;
use std::io::prelude::*;

struct Student {
    name: String,
    major: String,
   
}

impl Student {
    fn from_file(path: &str) -> Student {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let major = lines.next().unwrap().to_string();

        Student { name, major }
    }
}

fn reading_from_file() {
    let student = Student::from_file("config.txt");
    // Print data
    println!({:?})

    
}

fn main() {
    reading_from_console();
    reading_from_file();
    gytv
} 