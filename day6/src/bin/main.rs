extern crate day6;

use std::env;
use std::fs;
use day6::parse_instruction;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let mut grid: HashMap<(i16, i16), bool> = HashMap::with_capacity(1000000);

    content.split('\n').for_each(|s| parse_instruction(&mut grid, s));

    println!("Number of lights on: {}", grid.values().filter(|v| **v == true).count());
}
