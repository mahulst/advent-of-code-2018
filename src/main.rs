use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;

extern crate day9;
extern crate time;

fn main() {
    let input = open_file("./day9/input.txt");
    let time1 = precise_time_ns();

    // 424 players; last marble is worth 71482 points
    let result = day9::play_game(71482 * 100, 424);
    println!("length: {}", result);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


