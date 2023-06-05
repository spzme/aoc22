use core::str::FromStr;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();

        let mut numbers = parts.into_iter().map(|s| s.parse::<usize>());
        match (numbers.next(), numbers.next(), numbers.next()) {
            Some(Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => {
                let mv = Move {
                    count: a,
                    from: b,
                    to: c,
                };
                Ok(mv)
            }
            _ => Err("Unable to parse move."),
        }
    }
}

struct ParseResult {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parseinput() -> ParseResult {
    let parseresult = ParseResult {
        stacks: Vec::new(),
        moves: Vec::new(),
    };

    //The strategy is to parse the input file into two parts,
    //which are split by the empty line in the input.
    //(line number 10)

    let mut firstpart = true;

    for line in include_str!("../files/input.txt").lines() {
        if line.is_empty() {
            firstpart = false;
            break;
        }
        if firstpart {
            //parse this line as being part of the first part
        } else {
            //" for second part
        }
    }
    parseresult
}

fn main() {
    let parseresult = parseinput();

    println!("Hello, world!");
}
