use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;
use day10::Star;

extern crate day10;
extern crate time;

fn main() {
    let input = open_file("./day10/input.txt");
    let time1 = precise_time_ns();

    let mut stars = day10::input_to_stars(&input);
    for i in 0..13000 {
        println!("tick {}", i);

        let bounds = day10::get_bounds(&stars);
        if bounds.0 % 100 < 50 {
            eprintln!("bounds = {:?}", bounds);

        }

        // If stars align and are quite close together
        // Manual reviewing for every drawing
        if bounds.1 - bounds.0 < 350 {
            day10::draw_sky(&stars);
        };
        stars = day10::tick_stars(&stars);

    }
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


