use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;


extern crate day14;

fn main() {

    let time1 = precise_time_ns();

    let mut recipes = vec![3, 7];

    let result = day14::find_first_occurence(&mut recipes, &[5,1,3,4,0,1], 6, 25000000);
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


