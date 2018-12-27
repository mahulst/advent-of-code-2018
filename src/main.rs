use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use time::precise_time_ns;


extern crate day16;

fn main() {
    let time1 = precise_time_ns();

    let input = open_file("./day16/program.txt");

    let opcodes: Vec<day16::OpCode> = input.lines().map(|l| {
        l.parse().expect("Unwrapping opcode")
    }).collect();

    let mut reg = day16::Register::new(0,0,0,0);


    opcodes.iter().for_each(|oc| {
        let func = day16::opcode_id_to_fn(oc.id);
        reg = func(&reg, oc);
    });

    println!("{:?}", reg);
    let measure = (precise_time_ns() - time1) / 1000 / 1000;

    println!("time taken: {} ms", measure);
}

fn open_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Input not found");

    let mut string = String::new();
    file.read_to_string(&mut string).expect("Error while reading file");

    string
}


