use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use day6::Point;
use time::precise_time_ns;

extern crate day6;
extern crate time;
fn main() {
    let input = open_file("./day6/input.txt");
    let time1 = precise_time_ns();

    let points: Vec<Point> = input.lines().map(|l| l.parse().unwrap()).collect();
    let areas = day6::get_area(&points);
    let areas = day6::count_areas(&areas);

    let region = day6::get_area_of_distances(&points);
    let result: Vec<i32> = region.into_iter().filter(| size| size < &10000).collect();
    println!("length: {}", result.len());
    println!("{:?}", areas.iter().map(|(id, size)| size).max());
    let measure = (precise_time_ns() - time1)/ 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


