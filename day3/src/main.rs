use std::collections::HashMap;

fn parse_instructions(instructions: &str) -> usize {
    let mut position_map: HashMap<(i32, i32), i32> = HashMap::new();
    position_map.insert((0, 0), 2);
    let mut x;
    let mut y;
    let mut santa_position = (0, 0);
    let mut robot_position = (0, 0);
    for (index, instruction) in instructions.char_indices() {
        if index % 2 == 0 {
            x = santa_position.0;
            y = santa_position.1;
        } else {
            x = robot_position.0;
            y = robot_position.1;
        }
        match instruction {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!("Undefined character in the instructions")
        }

        if index % 2 == 0 {
            santa_position = (x, y);
        } else {
            robot_position = (x, y);
        }
        let mut updated_value = 1;
        match position_map.get(&(x, y)) {
            Some(value) => updated_value = value + 1,
            _ => {}
        };
        position_map.insert((x, y), updated_value);
    }
    position_map.len()
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let visited_houses = parse_instructions(content.as_str());
    println!("{} houses receive at least one present.", visited_houses);
}

#[test]
fn test() {
    assert_eq!(parse_instructions("^v"), 3);
    assert_eq!(parse_instructions("^>v<"), 3);
    assert_eq!(parse_instructions("^v^v^v^v^v"), 11);
}