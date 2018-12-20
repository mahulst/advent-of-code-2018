use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::{thread, time};


extern crate day13;

fn main() {
    let input = open_file("./day13/input.txt");

    let empty_track = day13::get_empty_tracks(&input);
    let mut carts = day13::find_carts(&input);

    let mut i = 0;

    let mut done = false;

    let result = day13::find_last_cart(&empty_track, &mut carts);

    println!("{:#?}", result);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


