#![allow(unused_variables)]
use core::str::FromStr;

#[derive(Debug)]
struct Item(usize);
struct Rucksack(Vec<Item>);

impl TryFrom<char> for Item {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(Item(c as usize - 95)),
            'A'..='Z' => Ok(Item(c as usize - 64)),
            _ => Err("Invalid char!"),
        }
    }
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rucksack: Rucksack = Rucksack(Vec::new());
        for c in s.chars() {
            let item = Item::try_from(c).unwrap();
            rucksack.0.push(item);
        }

        Err("Unable to parse rucksack from input")
    }
}

fn main() {
    for rucksack in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Rucksack>())
    {}

    let c = 'c';
    println!("We have an item: {:?}", Item::try_from(c))
}
