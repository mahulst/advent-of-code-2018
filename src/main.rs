use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;

extern crate day7;
extern crate time;

fn main() {
    let input = open_file("./day7/input.txt");
    let time1 = precise_time_ns();

    let reverse_deps = day7::to_nodes(&input);

    let result = day7::order_build_steps_sleigh(&mut reverse_deps.clone());
    let time_taken_to_build = day7::build_sleigh(
        &mut reverse_deps.clone(),
        5,
        60,
    );
    println!("length: {}", result);
    eprintln!("time_taken_to_build = {:?}", time_taken_to_build);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


