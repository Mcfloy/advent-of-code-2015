use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;

pub fn parse_instruction(grid: &mut HashMap<(i16, i16), bool>, instruction: &str) {
    if instruction.is_empty() {
        return;
    }
    let keywords: Vec<&str> = instruction.split(' ').collect();
    match keywords[0] {
        "turn" => {
            if keywords.len() != 5 {
                writeln!(std::io::stderr(), "Instruction example: turn (on|off) 0,0 through 999,999").unwrap();
                std::process::exit(1);
            }
            turn_lights(grid, keywords[1] == "on", str_to_tuple::<i16>(keywords[2], ',').unwrap(), str_to_tuple::<i16>(keywords[4], ',').unwrap());
        },
        "toggle" => {
            if keywords.len() != 4 {
                writeln!(std::io::stderr(), "Instruction example: toggle 0,0 through 999,999").unwrap();
                std::process::exit(1);
            }
            toggle_lights(grid, str_to_tuple::<i16>(keywords[1], ',').unwrap(), str_to_tuple::<i16>(keywords[3], ',').unwrap());
        },
        value => {
            writeln!(std::io::stdout(), "{:?}", instruction).unwrap();
            writeln!(std::io::stdout(), "{:?}", keywords).unwrap();
            panic!("Incorrect keyword '{}', only accepts turn and toggle", value);
        }
    }
}

fn turn_lights(grid: &mut HashMap<(i16, i16), bool>, state: bool, start_position: (i16, i16), end_position: (i16, i16)) {
    for x in (start_position.0)..(end_position.0 + 1) {
        for y in (start_position.1)..(end_position.1 + 1) {
            (*grid).insert((x, y), state);
        }
    }
}

fn toggle_lights(grid: &mut HashMap<(i16, i16), bool>, start_position: (i16, i16), end_position: (i16, i16)) {
    for x in (start_position.0)..(end_position.0 + 1) {
        for y in (start_position.1)..(end_position.1 + 1) {
            let value = *grid.get_mut(&(x, y)).unwrap_or(&mut false);
            (*grid).insert((x, y), !value);
        }
    }
}

fn str_to_tuple<T: FromStr>(string: &str, separator: char) -> Option<(T, T)> {
    match string.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&string[..index]), T::from_str(&string[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test() {
    let mut grid: HashMap<(i16, i16), bool> = HashMap::with_capacity(1000000);
    parse_instruction(&mut grid, "turn on 0,0 through 999,999");
    assert_eq!(grid.values().filter(|v| **v == true).count(), 1000000);
    parse_instruction(&mut grid, "toggle 0,0 through 999,0");
    assert_eq!(grid.values().filter(|v| **v == true).count(), 999000);
    parse_instruction(&mut grid, "turn off 499,499 through 500,500");
    assert_eq!(grid.values().filter(|v| **v == true).count(), 998996);
}
