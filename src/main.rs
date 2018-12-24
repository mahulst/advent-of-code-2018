use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;


extern crate day15;

fn main() {

    let time1 = precise_time_ns();

    let input = open_file("./day15/input.txt");

    let map = day15::parse_map(&input);

    let result = day15::simulate_battle(map);
    println!("{:?}", result);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

//    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


