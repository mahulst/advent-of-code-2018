use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::collections::HashMap;
use day3::Claim;
use std::prelude::v1::Vec;

extern crate day3;

fn main() {
    let input = open_file("./day3/input.txt");

    let mut fabric = HashMap::new();
    let claims: Vec<Claim> = input.lines().map(|line| day3::parse_line(line)).collect();

    claims.iter().for_each(|claim| {
        day3::plot_square(&mut fabric, claim);
    });

    let overlapping_fabric = day3::count_overlap(&fabric, 2);
    let non_overlapping_claim = day3::find_non_overlapping(&claims, &fabric).unwrap();
    println!("{}", overlapping_fabric);
    println!("{:?}", non_overlapping_claim);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


