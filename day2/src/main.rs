fn parse_dimension_and_shortest_perimeter(line: &str) -> (i32, i32) {
    let mut sides: Vec<i32> = line.split('x').map(|x| (x.parse::<i32>().unwrap())).collect();
    if sides.len() < 3 {
        panic!("Input is not a pattern of lxwxh");
    }

    println!("Unsorted sides: {:?}", sides);

    let length = sides[0];
    let width = sides[1];
    let height = sides[2];

    sides.sort();

    let multiplied_sides = vec![length * width, width * height, height * length];
    let min_multiplied_side = multiplied_sides.iter().min().unwrap();


    let sum_sides: i32 = multiplied_sides.iter().map(|number| 2 * number).sum();

    println!("({} * 2) + ({} * 2)", sides[0], sides[1]);

    let cubic_volume = sides[0] * sides[1] * sides[2];
    let smallest_perimeter = (sides[0] * 2) + (sides[1] * 2);
    (min_multiplied_side + sum_sides, smallest_perimeter + cubic_volume)
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let mut total_required_wrapper_paper: i32 = 0;
    let mut total_required_ribbon: i32 = 0;

    // DEVNOTE: \n if for unix files, \r\n for windows files
    for line in content.split('\n') {
        let (required_wrapped_paper, required_ribbon) = parse_dimension_and_shortest_perimeter(line);
        total_required_wrapper_paper += required_wrapped_paper;
        total_required_ribbon += required_ribbon;
    }

    println!("Total square feet of wrapping paper required: {}\nTotal ribbon required: {}", total_required_wrapper_paper, total_required_ribbon);
}

#[test]
fn test() {
    assert_eq!(parse_dimension_and_shortest_perimeter("2x3x4"), (58, 10));
    assert_eq!(parse_dimension_and_shortest_perimeter("1x1x10"), (43, 4));
}
