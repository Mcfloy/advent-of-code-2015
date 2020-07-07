extern crate fancy_regex;

use fancy_regex::Regex;

fn is_this_string_nice_too(string: &str) -> bool {
    let contains_pair_of_two_letters_regex = Regex::new(r"([a-z][a-z]).*\1").unwrap();
    let contains_two_letters_between_one_regex = Regex::new(r"([a-z])[a-z]\1").unwrap();

    contains_pair_of_two_letters_regex.is_match(string).unwrap() && contains_two_letters_between_one_regex.is_match(string).unwrap()
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let number_of_nice_strings = content.split('\n')
        .filter(|string| is_this_string_nice_too(string))
        .count();
    println!("Number of nice strings with the updated rules: {}", number_of_nice_strings);
}

#[test]
fn test() {
    assert_eq!(is_this_string_nice_too("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_this_string_nice_too("xxyxx"), true);
    assert_eq!(is_this_string_nice_too("uurcxstgmygtbstg"), false);
    assert_eq!(is_this_string_nice_too("ieodomkazucvgmuy"), false);
}
