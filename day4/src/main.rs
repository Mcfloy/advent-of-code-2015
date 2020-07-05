extern crate md5;

use std::sync::{Arc, Mutex};

fn generate_second_part_md5(hash: &str, min: u32, max: u32, pattern: &str) -> Option<u32> {
    for number in min..max {
        let digest = md5::compute(format!("{}{}", hash, number));
        if format!("{:x}", digest).starts_with(pattern) {
            return Some(number);
        }
    }
    None
}

use std::env;
use std::io::Write;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        writeln!(std::io::stderr(), "Usage: {} input pattern threads_used", args[0]).unwrap();
        std::process::exit(1);
    }
    let entry: &String = &args[1];
    let pattern: &String = &args[2];
    let threads: u32 = args[3].parse().unwrap();

    let number_chunk: u32 = 10000000 / threads;
    let shared_numbers = Arc::new(Mutex::new(vec![]));

    crossbeam::scope( |spawner| {
        for i in 0..threads {
            let shared_numbers: Arc<Mutex<Vec<u32>>> = Arc::clone(&shared_numbers);
            let min = i * number_chunk;
            let max = ((i + 1) * number_chunk) - 1;
            spawner.spawn(move |_| {
                let value = generate_second_part_md5(entry, min, max, pattern);
                if value != None {
                    let mut numbers = shared_numbers.lock().unwrap();
                    numbers.push(value.unwrap());
                }
            });
        }
    }).unwrap();

    (*shared_numbers.lock().unwrap()).sort();

    println!("First number that matches the hash is: {}", (*shared_numbers.lock().unwrap())[0]);
}

#[test]
fn test() {
    assert_eq!(generate_second_part_md5("abcdef", 609040, 609050, "00000"), 609043);
    assert_eq!(generate_second_part_md5("pqrstuv", 1048900, 1050000, "00000"), 1048970);
}