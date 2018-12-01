use std::fs::File;
use std::io::prelude::*;
use std::string::String;

extern crate day1;

fn main() {
    let input = open_file("./day1/input.txt");

    let day1_result = day1::find_first_duplicate(input.lines());

    println!("{}", day1_result);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}

