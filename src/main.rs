use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;

extern crate day8;
extern crate time;

fn main() {
    let input = open_file("./day8/input.txt");
    let time1 = precise_time_ns();


    let result = day8::count_metadata(&day8::input_to_nodes(&day8::parse(&input)));
    let result2 = day8::count_metadata_with_references(&day8::input_to_nodes(&day8::parse(&input)));
    println!("length: {}", result);
    println!("length 2: {}", result2);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


