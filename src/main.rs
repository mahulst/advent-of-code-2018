use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use std::collections::HashMap;
use std::prelude::v1::Vec;
use day4::GuardEvent;

extern crate day4;

fn main() {
    let input = open_file("./day4/input.txt");

    let mut events: Vec<GuardEvent> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    day4::sort_events(&mut events);

    let sleep_info = day4::events_to_sleep_info(&events);
    let sleepy_guard = day4::get_most_sleeping_guard(&sleep_info);
    let sleepy_minute = day4::get_most_sleeped_minute(&sleep_info);

    println!("{:?}", sleepy_guard);
    println!("{:?}", sleepy_minute);

}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


