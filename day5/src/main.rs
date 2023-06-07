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
    stacks: Stacks,
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
struct Stacks {
    positions: Vec<Vec<char>>,
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
        stacks: Stacks {
            positions: Vec::new(),
        },
        moves: Vec::new(),
    };

    //The strategy is to parse the input file into two parts,
    //which are split by the empty line in the input.
    //(line number 10)
    let number_of_stacks = 9; // This can and should be read from the input

    let mut i = 0;

    while i < number_of_stacks {
        let stack: Vec<char> = Vec::new();
        parseresult.stacks.positions.push(stack);
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
            parseresult.stacks.positions[b.stack].push(b.name);
        }
    }

    parseresult
}

fn is_all_numbers(s: &str) -> bool {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .all(char::is_numeric)
}

#[allow(dead_code)]
fn apply_move_p1(mut stacks: Stacks, mv: Move) -> Stacks {
    let mut i = 0;
    while i < mv.count {
        let b = stacks.positions[mv.from - 1].pop().unwrap();
        stacks.positions[mv.to - 1].push(b);
        i += 1;
    }

    stacks
}

#[allow(dead_code)]
fn apply_move_p2(mut stacks: Stacks, mv: Move) -> Stacks {
    let mut boxes: Vec<char> = Vec::new();
    let mut i = 0;
    while i < mv.count {
        let b = stacks.positions[mv.from - 1].pop().unwrap();
        boxes.push(b);
        i += 1;
    }
    boxes.reverse();

    stacks.positions[mv.to - 1].append(&mut boxes);

    stacks
}

fn main() {
    let parseresult = parseinput();

    let mut stacks = parseresult.stacks;

    //part 1
    // for mv in parseresult.moves {
    //     stacks = apply_move_p1(stacks, mv)
    // }

    //part 2
    for mv in parseresult.moves {
        stacks = apply_move_p2(stacks, mv);
    }

    println!("Stacks of boxes: {:?}", stacks);

    //I would like to find a nice way to obtain the last item in every stack,
    //But I could not get this to work with iterators and maps.

    //Maybe later! ;)
}
