extern crate day7;

use std::env;
use std::fs;
use std::collections::HashMap;
use day7::calculate_wire_with_cache;
use day7::parse_instruction;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let mut hash_map = HashMap::new();
    content.split('\n').for_each(|s| parse_instruction(&mut hash_map, s));

    let wire_name = String::from("a");
    let shared_cache = Rc::new(RefCell::new(HashMap::new()));
    println!("Signal value of {} is {}", &wire_name, calculate_wire_with_cache(&mut hash_map, wire_name.parse().unwrap(), shared_cache));
}
