
#[macro_use] extern crate lazy_static;

extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum Operator {
    NOT
}

pub fn parse_instruction(hash_map: &mut HashMap<String, Vec<String>>, string: &str) {
    lazy_static! {
        static ref REGEX_TRANSFER: Regex = Regex::new(r"^(?P<input>[a-z]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_NUMBER: Regex = Regex::new(r"^(?P<value>[0-9]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_AND: Regex = Regex::new(r"^(?P<left>[a-z0-9]*) AND (?P<right>[a-z]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_OR: Regex = Regex::new(r"^(?P<left>[a-z]*) OR (?P<right>[a-z]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_LSHIFT: Regex = Regex::new(r"^(?P<input>[a-z]*) LSHIFT (?P<value>[1-9]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_RSHIFT: Regex = Regex::new(r"^(?P<input>[a-z]*) RSHIFT (?P<value>[1-9]*) -> (?P<output>[a-z]*)$").unwrap();
        static ref REGEX_NOT: Regex = Regex::new(r"^NOT (?P<input>[a-z]*) -> (?P<output>[a-z]*)$").unwrap();
    }
    if REGEX_TRANSFER.is_match(string) {
        let captures = REGEX_TRANSFER.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["input"].parse().unwrap(), "TRANSFER".parse().unwrap()]);
    } else if REGEX_NUMBER.is_match(string) {
        let captures = REGEX_NUMBER.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["value"].parse().unwrap()]);
    } else if REGEX_NOT.is_match(string) {
        let captures = REGEX_NOT.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["input"].parse().unwrap(), "NOT".parse().unwrap()]);
    } else if REGEX_AND.is_match(string) {
        let captures = REGEX_AND.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["left"].parse().unwrap(), captures["right"].parse().unwrap(), "AND".parse().unwrap()]);
    } else if REGEX_OR.is_match(string) {
        let captures = REGEX_OR.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["left"].parse().unwrap(), captures["right"].parse().unwrap(), "OR".parse().unwrap()]);
    } else if REGEX_LSHIFT.is_match(string) {
        let captures = REGEX_LSHIFT.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["input"].parse().unwrap(), captures["value"].parse().unwrap(), "LSHIFT".parse().unwrap()]);
    } else if REGEX_RSHIFT.is_match(string) {
        let captures = REGEX_RSHIFT.captures(string).unwrap();
        hash_map.entry(captures["output"].parse().unwrap()).or_insert(vec![captures["input"].parse().unwrap(), captures["value"].parse().unwrap(), "RSHIFT".parse().unwrap()]);
    }
}

// Recursive function that was too much consuming CPU and memory
// fn calculate_wire(hash_map: &HashMap<String, Vec<String>>, wire_name: String, depth_counter: u16) -> u16 {
//     // println!("Calculate wire {}", wire_name);
//     if depth_counter > 348 {
//         panic!("Unexpected high depth for value {}", wire_name);
//     }
//     let entry = hash_map.get(&wire_name).unwrap();
//     println!("wire {} - {}: {:?}", wire_name, depth_counter, entry);
//     let operation = entry.last().expect("Cannot get operation of the entry");
//     if let Ok(value) = operation.parse::<u16>() {
//         value
//     } else {
//         match operation.as_str() {
//             "TRANSFER" => calculate_wire(hash_map, entry.first().unwrap().parse().unwrap(), depth_counter + 1),
//             "NOT" => !calculate_wire(hash_map, entry.first().unwrap().parse().unwrap(), depth_counter + 1),
//             "AND" => {
//                 if let Ok(value) = entry.get(0).unwrap().parse::<u16>() {
//                     value & calculate_wire(hash_map, entry.get(1).unwrap().parse().unwrap(), depth_counter + 1)
//                 } else {
//                     calculate_wire(hash_map, entry.get(0).unwrap().parse().unwrap(), depth_counter + 1) & calculate_wire(hash_map, entry.get(1).unwrap().parse().unwrap(), depth_counter + 1)
//                 }
//             },
//             "OR" => calculate_wire(hash_map, entry.get(0).unwrap().parse().unwrap(), depth_counter + 1) | calculate_wire(hash_map, entry.get(1).unwrap().parse().unwrap(), depth_counter + 1),
//             "LSHIFT" => calculate_wire(hash_map, entry.get(0).unwrap().parse().unwrap(), depth_counter + 1) << entry.get(1).unwrap().parse::<u16>().unwrap(),
//             "RSHIFT" => calculate_wire(hash_map, entry.get(0).unwrap().parse().unwrap(), depth_counter + 1) >> entry.get(1).unwrap().parse::<u16>().unwrap(),
//             _ => panic!("Undefined operation")
//         }
//     }
// }

pub fn calculate_wire_with_cache(hash_map: &HashMap<String, Vec<String>>, wire_name: String, shared_cache: Rc<RefCell<HashMap<String, u16>>>) -> u16 {
    let entry = hash_map.get(&wire_name).unwrap();
    {
        if let Some(value) = shared_cache.borrow().get(&*wire_name.clone()) {
            return value.clone();
        }
    }
    let operation = entry.last().expect("Cannot get operation of the entry");
    if let Ok(value) = operation.parse::<u16>() {
        let mut cache = shared_cache.borrow_mut();
        cache.insert(wire_name.clone(), value);
        value
    } else {
        match operation.as_str() {
            "TRANSFER" => {
                let result = calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache));
                let mut cache = shared_cache.borrow_mut();
                cache.insert(wire_name, result);
                result
            },
            "NOT" => {
                let result = !calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache));
                let mut cache = shared_cache.borrow_mut();
                cache.insert(wire_name, result);
                result
            },
            "OR" => {
                let result = calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache)) | calculate_wire_with_cache(hash_map, entry[1].clone(), Rc::clone(&shared_cache));
                let mut cache = shared_cache.borrow_mut();
                cache.insert(wire_name, result);
                result
            },
            "AND" => {
                if let Ok(value) = entry[0].clone().parse::<u16>() {
                    let result = value & calculate_wire_with_cache(hash_map, entry[1].clone(), Rc::clone(&shared_cache));
                    let mut cache = shared_cache.borrow_mut();
                    cache.insert(wire_name, result);
                    result
                } else {
                    let result = calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache)) & calculate_wire_with_cache(hash_map, entry[1].clone(), Rc::clone(&shared_cache));
                    let mut cache = shared_cache.borrow_mut();
                    cache.insert(wire_name, result);
                    result
                }
            }
            "LSHIFT" => {
                let result = calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache)) << entry[1].clone().parse::<u16>().unwrap();
                let mut cache = shared_cache.borrow_mut();
                cache.insert(wire_name, result);
                result
            },
            "RSHIFT" => {
                let result = calculate_wire_with_cache(hash_map, entry[0].clone(), Rc::clone(&shared_cache)) >> entry[1].clone().parse::<u16>().unwrap();
                let mut cache = shared_cache.borrow_mut();
                cache.insert(wire_name, result);
                result
            }
            _ => panic!("Undefined operation")
        }
    }
}

#[test]
fn test() {
    let mut hash_map = HashMap::new();
    parse_instruction(&mut hash_map, "123 -> x");
    parse_instruction(&mut hash_map, "456 -> y");
    parse_instruction(&mut hash_map, "x AND y -> d");
    parse_instruction(&mut hash_map, "x OR y -> e");
    parse_instruction(&mut hash_map, "x LSHIFT 2 -> f");
    parse_instruction(&mut hash_map, "y RSHIFT 2 -> g");
    parse_instruction(&mut hash_map, "NOT x -> h");
    parse_instruction(&mut hash_map, "NOT y -> i");

    assert_eq!(calculate_wire(&hash_map, String::from("d"), 1), 72);
    assert_eq!(calculate_wire(&hash_map, String::from("e"), 1), 507);
    assert_eq!(calculate_wire(&hash_map, String::from("f"), 1), 492);
    assert_eq!(calculate_wire(&hash_map, String::from("g"), 1), 114);
    assert_eq!(calculate_wire(&hash_map, String::from("h"), 1), 65412);
    assert_eq!(calculate_wire(&hash_map, String::from("i"), 1), 65079);
    assert_eq!(calculate_wire(&hash_map, String::from("x"), 1), 123);
    assert_eq!(calculate_wire(&hash_map, String::from("y"), 1), 456);
}