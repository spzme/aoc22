use core::str::FromStr;

#[derive(Debug)]
struct Pair {
    low1: usize,
    low2: usize,
    high1: usize,
    high2: usize,
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c: char| c == ',' || c == '-').collect();

        let mut numbers = parts.into_iter().map(|s| s.parse::<usize>());

        match (
            numbers.next(),
            numbers.next(),
            numbers.next(),
            numbers.next(),
        ) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c)), Some(Ok(d))) => {
                let pair = Pair {
                    low1: a,
                    high1: b,
                    low2: c,
                    high2: d,
                };
                return Ok(pair);
            }
            _ => Err("Was unable to parse pair"),
        }
    }
}

fn find_overlap(pair: Pair) -> bool {
    pair.low1 >= pair.low2 && pair.high1 <= pair.high2  //duty of elf 1 fits in duty of elf 2
        || pair.low2 >= pair.low1 && pair.high2 <= pair.high1 //vice versa
}

fn main() {
    let mut count = 0;
    for pair in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Pair>())
    {
        let pair = pair.unwrap();
        if find_overlap(pair) {
            count += 1;
        }
    }
    println!("The number of pairs with an overlap equals {:?}", count);
}
