fn parse_instructions(instructions: &str) -> (i32, Option<usize>) {
    let mut current_floor: i32 = 0;
    let mut index_first_time_in_basement: Option<usize> = None;
    // SPLIT EACH CHARACTER

    // LOOP THEM
    for (index, character) in instructions.char_indices() {
        match character {
            '(' => current_floor += 1,
            ')' => {
                current_floor -= 1;
                if current_floor < 0 && index_first_time_in_basement == None {
                    index_first_time_in_basement = Some(index + 1);
                }
            },
            _ => { panic!("Unexpected character, check the input given to the program.") }
        }
    }
    // INCREMENT OR DECREMENT THE FLOOR
    // OUTPUT THE NUMBER
    (current_floor, index_first_time_in_basement)
}

use std::env;

fn main() {
    // READ THE INPUT
    let args: Vec<String> = env::args().collect();

    let input = &args[1].replace("\"","");

    // PARSE THE INSTRUCTIONS
    let (expected_floor, index_first_time_in_basement) = parse_instructions(input.as_str());

    // OUTPUT THE EXPECTED FLOOR
    println!("expected floor is {}, index of first time in basement is: {:?}", expected_floor, index_first_time_in_basement);
}

#[test]
fn test() {
    assert_eq!(parse_instructions("(())"), (0, None));
    assert_eq!(parse_instructions("()()"), (0, None));
    assert_eq!(parse_instructions("((("), (3, None));
    assert_eq!(parse_instructions("(()(()("), (3, None));
    assert_eq!(parse_instructions("))((((("), (3, Some(1)));
    assert_eq!(parse_instructions("())"), (-1, Some(3)));
    assert_eq!(parse_instructions("))("), (-1, Some(1)));
    assert_eq!(parse_instructions(")))"), (-3, Some(1)));
    assert_eq!(parse_instructions(")())())"), (-3, Some(1)));
}