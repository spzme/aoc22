use std::collections::HashSet;

const DIGITS: usize = 14;

fn process(count: usize, s: &str) -> usize {
    if first_4_unique(s) {
        return count + DIGITS; //we add 4, because the marker is AFTER the sequence
    }

    if s.len() < 4 {
        println!("Reached end of string.");
        return 0;
    }

    process(count + 1, &s[1..s.len()])
}

fn first_4_unique(s: &str) -> bool {
    let mut unique: HashSet<char> = HashSet::new();
    let mut i = 0;

    while i < DIGITS {
        let newchar = match s[i..DIGITS].chars().next() {
            Some(a) => a,
            None => return false,
        };
        //println! {"{:?} {:?}", i, unique};
        unique.insert(newchar);
        i += 1;
    }
    //println! {"{:?}", unique};
    unique.len() == DIGITS
}

fn main() {
    let input = include_str!("../files//smalldemo.txt")
        .lines()
        .next()
        .unwrap();

    let result = process(0, input);
    println!("{:?}", result);
}
