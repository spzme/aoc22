use std::collections::HashSet;
use std::time::Instant;

const DIGITS: usize = 14; //set to desired number of distinct characters

fn process(count: usize, s: &str) -> usize {
    if first_n_unique(s) {
        return count + DIGITS;
    }

    if s.len() < 4 {
        println!("Reached end of string.");
        return 0;
    }

    process(count + 1, &s[1..s.len()])
}

fn first_n_unique(s: &str) -> bool {
    let mut unique: HashSet<char> = HashSet::new();
    let mut i: usize = 0;

    while i < DIGITS {
        let newchar = match s.chars().nth(i) {
            Some(a) => a,
            None => return false,
        };

        //small optimization
        if !unique.insert(newchar) {
            return false;
        }
        i += 1;
    }

    unique.len() == DIGITS
}

fn main() {
    let input = include_str!("../files//smalldemo.txt")
        .lines()
        .next()
        .unwrap();

    let start = Instant::now();
    let result = process(0, input);
    let duration = start.elapsed();

    println!("Result: {:?}, duration: {:?}", result, duration);
}
