use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: {} input.txt", args[0]).unwrap();
        std::process::exit(1);
    }
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Cannot read file");

    let number_of_nice_strings = content.split('\n')
        .filter(|string| is_this_string_nice(string))
        .count();
    println!("Number of nice strings in the input file: {}", number_of_nice_strings);
}

fn is_this_string_nice(string: &str) -> bool {
    let forbidden_strings: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
    let vowels: Vec<&str> = vec!["a", "e", "i", "o", "u"];

    // Must be true to be a nice string
    let mut contains_three_vowels = false;
    let mut counter_of_vowels = 0;

    // Must be true to be a nice string
    let mut contains_duplicate_letter = false;
    // That's bold of me to do this as it would returns true on empty string, but there's two rules to prevent false positive.
    let mut previous_char: char = '\0';

    for character in string.chars() {
        if previous_char == character {
            contains_duplicate_letter = true;
        }
        if vowels.contains(&character.to_string().as_str()) {
            counter_of_vowels += 1;
            if counter_of_vowels == 3 {
                contains_three_vowels = true;
            }
        }

        if contains_duplicate_letter && contains_three_vowels {
            break;
        }
        previous_char = character;
    }

    // Must be false to be a nice string
    let mut contains_forbidden_strings = false;
    for forbidden_string in forbidden_strings {
        if string.contains(forbidden_string) {
            contains_forbidden_strings = true;
            break;
        }
    }

    contains_three_vowels && contains_duplicate_letter && !contains_forbidden_strings
}

fn is_this_string_nice_too(string: &str) -> bool {
    // With the new rules, we need at least 4 letters.
    if string.len() < 4 {
        return false;
    }

    // Must be true to be a nice string
    let mut contains_pair_of_two_letters = false;

    // Must be true to be a nice string
    let mut contains_letter_between_letters = false;

    let mut buffer = vec![];
    buffer.reserve(string.len() - 1);

    println!("{}", string);
    let mut chars = string.chars();
    println!("{:?}", chars);

    // for i in 1..string.len() - 2 {
        //
        // if chars.nth(i - 1).unwrap() == chars.nth(i + 1).unwrap() && chars.nth(i).unwrap() != chars.nth(i + 1).unwrap() {
        //     contains_letter_between_letters = true;
        // }
        // if buffer.contains(&format!("{}{}", chars.nth(i).unwrap(), chars.nth(i + 1).unwrap())) {
        //     contains_pair_of_two_letters = true;
        // }
        //
        // if contains_letter_between_letters && contains_pair_of_two_letters {
        //     break;
        // }
        // buffer.push(format!("{}{}", string[i], string[i + 1]));
    // }


    contains_pair_of_two_letters && contains_letter_between_letters
}

#[test]
fn test() {
    assert_eq!(is_this_string_nice("ugknbfddgicrmopn"), true);
    assert_eq!(is_this_string_nice("aaa"), true);
    assert_eq!(is_this_string_nice("jchzalrnumimnmhp"), false);
    assert_eq!(is_this_string_nice("haegwjzuvuyypxyu"), false);
    assert_eq!(is_this_string_nice("dvszwmarrgswjxmb"), false);
}

#[test]
fn test_two() {
    assert_eq!(is_this_string_nice_too("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_this_string_nice_too("xxyxx"), true);
    assert_eq!(is_this_string_nice_too("uurcxstgmygtbstg"), false);
    assert_eq!(is_this_string_nice_too("ieodomkazucvgmuy"), false);
}