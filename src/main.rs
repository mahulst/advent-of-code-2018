use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;

extern crate day11;
extern crate time;

fn main() {
    let input = open_file("./day10/input.txt");
    let time1 = precise_time_ns();
    let result = day11::get_largest_cell_of_any_size(1309);

    println!("{:?}", result);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


