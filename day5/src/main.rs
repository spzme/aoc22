use core::str::FromStr;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();

        let mut numbers = parts
            .into_iter()
            .filter(|s| is_all_numbers(s))
            .map(|s| s.parse::<usize>());

        match (numbers.next(), numbers.next(), numbers.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => {
                let mv = Move {
                    count: a,
                    from: b,
                    to: c,
                };
                Ok(mv)
            }

            _ => {
                println!("{:?}", s);
                Err("Unable to parse move.")
            }
        }
    }
}

#[derive(Debug)]
struct ParseResult {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Row {
    boxes: Vec<Box>,
}

impl FromStr for Row {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //The column nr determines which stack an item is in.
        //Stacknr = (colnr+3)/4
        let mut colnr = 0;
        let mut row = Row { boxes: Vec::new() };

        for char in s.chars() {
            if char.is_alphabetic() {
                //we found the contents of a box.
                let b = Box {
                    stack: (colnr - 1) / 4,
                    name: char,
                };
                row.boxes.push(b);
            }
            colnr += 1;
        }

        if colnr == s.len() {
            return Ok(row);
        }
        println!("{:?} {:?}", s, row);
        Err("Unable to parse row.")
    }
}

#[derive(Debug)]
struct Box {
    stack: usize,
    name: char,
}

fn parseinput() -> ParseResult {
    let mut parseresult = ParseResult {
        stacks: Vec::new(),
        moves: Vec::new(),
    };

    //The strategy is to parse the input file into two parts,
    //which are split by the empty line in the input.
    //(line number 10)
    let number_of_stacks = 9; // This can and should be read from the input

    let mut i = 0;

    while i < number_of_stacks {
        let mut stack: Vec<char> = Vec::new();
        parseresult.stacks.push(stack);
        i += 1;
    }

    let mut firstpart = true;
    let mut rows: Vec<Row> = Vec::new();

    for line in include_str!("../files/input.txt").lines() {
        if line.is_empty() {
            firstpart = false;
            continue;
        }
        if firstpart {
            //if the line contains only numbers, we do not parse it.
            if is_all_numbers(line) {
                continue;
            }
            let row = line.parse::<Row>().unwrap();
            rows.push(row);
            //parse this line as being part of the first part
        } else {
            //" for second part
            let mv = line.parse::<Move>().unwrap();
            parseresult.moves.push(mv);
        }
    }

    //Now we "transpose" (sort of) the rows into stacks.
    for row in rows.into_iter().rev() {
        for b in row.boxes {
            parseresult.stacks[b.stack].push(b.name);
        }
    }

    parseresult
}

fn is_all_numbers(s: &str) -> bool {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .all(char::is_numeric)
}

fn main() {
    let parseresult = parseinput();

    println!("{:?}", parseresult);
}
