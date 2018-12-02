use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::collections::HashSet;
use std::iter::FromIterator;

extern crate day1;
extern crate day2;

fn main() {
    let input = open_file("./day2/input.txt");

    let similar_ids = day2::get_similar_ids(&input);
    let ids: Vec<&str> = similar_ids.iter().cloned().collect();
    let id1 = ids.get(0).unwrap();
    let id2 = ids.get(1).unwrap();
    let day2_result: HashSet<char> =
        HashSet::from_iter(day2::get_similar_chars(id1, id2).into_iter());

    let stringified: String = day2_result.iter().collect();

    println!("{:?}", stringified);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


