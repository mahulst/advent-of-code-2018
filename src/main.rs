use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;

extern crate day12;
extern crate time;

fn main() {
    let pattern_input = open_file("./day12/patterns.txt");
    let input = "##.##.##..#..#.#.#.#...#...#####.###...#####.##..#####.#..#.##..#..#.#...#...##.##...#.##......####.";
    let time1 = precise_time_ns();
    let mut initial_state = day12::input_to_list(input);
    let patterns = pattern_input.lines().map(|l| day12::parse_pattern(l)).collect();
    let mut new_row = day12::tick_row(&patterns, &mut initial_state);

    let generations: usize = 50000000000;

    let mut increase: usize = 0;
    let mut previous_sum: usize = 0;
    for x in 0..100 - 1 {
        new_row = day12::tick_row(&patterns, &mut new_row);

        let sum = day12::count_row(&new_row) as usize;

        increase = sum - previous_sum;
        previous_sum = sum;
    }

    let total = (increase * (generations - 100)) + previous_sum;
    println!("{:?}", total);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


