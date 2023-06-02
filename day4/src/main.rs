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
                Ok(pair)
            }
            _ => Err("Was unable to parse pair"),
        }
    }
}

fn find_full_overlap(pair: &Pair) -> bool {
    pair.low1 >= pair.low2 && pair.high1 <= pair.high2  //duty of elf 1 fits in duty of elf 2
        || pair.low2 >= pair.low1 && pair.high2 <= pair.high1 //vice versa
}

fn find_partial_overlap(pair: &Pair) -> bool {
    pair.low2 <= pair.low1 && pair.low1 <= pair.high2
        || pair.low2 <= pair.high1 && pair.high1 <= pair.high2
        || pair.low1 <= pair.low2 && pair.low2 <= pair.high1
        || pair.low1 <= pair.high2 && pair.high2 <= pair.high1
}

fn main() {
    let mut count_full = 0;
    let mut count_partial = 0;
    for pair in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Pair>())
    {
        let pair = pair.unwrap();
        if find_full_overlap(&pair) {
            count_full += 1;
        }

        if find_partial_overlap(&pair) {
            count_partial += 1;
        }
    }
    println!(
        "PART1: The number of pairs with a full overlap equals {:?}",
        count_full
    );
    println!(
        "PART2: The number of fairs with a partial overlap equals {:?}",
        count_partial
    );
}
