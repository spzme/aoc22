use core::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Item(usize);

#[derive(Debug)]
struct Rucksack(Vec<Item>);

impl TryFrom<char> for Item {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(Item(c as usize - 96)),
            'A'..='Z' => Ok(Item(c as usize - 64 + 26)),
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

        if s.chars().count() == rucksack.0.len() {
            return Ok(rucksack);
        }

        Err("Unable to parse rucksack from input")
    }
}

fn find_both(rucksack: Rucksack) -> Result<Item, &'static str> {
    let mut contents = rucksack.0.to_vec();
    let len = contents.len();
    let firsthalf = Vec::split_off(&mut contents, len / 2);
    let secondhalf = contents;

    for item in firsthalf {
        if secondhalf.contains(&item) {
            return Ok(item);
        }
    }
    Err("No item that was in both compartments was found.")
}

fn part1() {
    let mut sum = 0;
    for rucksack in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Rucksack>())
    {
        let rucksack = rucksack.unwrap();
        //println!("We have a rucksack: {:?}", rucksack);
        let duplicate = find_both(rucksack).unwrap();
        //println!("Duplicate item in both compartments: {:?} ", duplicate)
        sum += duplicate.0;
    }
    println!("[PART1] The sum of the duplicate items is {:?} ", sum);
}

fn part2() {
    //First, construct the groups of 3 rucksacks
    let mut rucksacks: Vec<Vec<Rucksack>> = Vec::new();
    rucksacks.push(Vec::new()); //first group
    let mut count = 0;
    for rucksack in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Rucksack>())
    {
        let rucksack = rucksack.unwrap();

        if count > 2 {
            rucksacks.push(Vec::new());
            count = 0;
        }
        let group_to_add_to = rucksacks.last_mut().unwrap();
        group_to_add_to.push(rucksack);
        count += 1;
    }

    println!("number of groups: {:?}", rucksacks.len());

    let mut sum = 0;

    //Now, determine the common item in the group.
    for group in rucksacks {
        let common_element = determine_common_element(group);
        sum += common_element.0;
    }

    println!("The sum of the common elements equals {:?}", sum);
}

fn determine_common_element(rucksacks: Vec<Rucksack>) -> Item {
    let rucksack = rucksacks.first().unwrap();
    let items = rucksack.0.clone();

    let items2 = rucksacks.get(1).unwrap().0.clone();
    let items3 = rucksacks.get(2).unwrap().0.clone();

    for item in items {
        if items2.contains(&item) && items3.contains(&item) {
            return item;
        }
    }

    Item(0)
}

fn main() {
    part1();

    part2();
}
