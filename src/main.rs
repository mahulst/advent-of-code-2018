use std::fs::File;
use std::io::prelude::*;
use std::string::String;

extern crate day5;

fn main() {
    let input = open_file("./day5/input.txt");


    let polymer = day5::find_most_blocking_unit(&input);

    println!("{}", polymer);

}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


