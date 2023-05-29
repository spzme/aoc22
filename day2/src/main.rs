#![allow(dead_code)]
#![allow(unused_variables)]
//We allow this because we only use some of the functions
//for part 1 and part 2.

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(self, theirs: Move) -> Outcome {
        match (self, theirs) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }

    fn our_move(self, outcome: Outcome) -> Move {
        match (self, outcome) {
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    ours: Move,
    theirs: Move,
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Not a valid char."),
        }
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(ours), Some(' '), Some(theirs), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("invalid line");
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

fn add_points_p1(previouspoints: usize, round: Round) -> usize {
    //Evaluate value of round

    previouspoints + determine_points_p1(round)
}

fn determine_points_p1(round: Round) -> usize {
    let outcome = round.ours.outcome(round.theirs);
    let outcome_val = outcome.inherent_points();
    let move_val = round.ours.inherent_points();

    outcome_val + move_val
}

fn add_points_p2(previouspoints: usize, round: Round) -> usize {
    previouspoints + determine_points_p2(round)
}

fn determine_points_p2(round: Round) -> usize {
    let their_move = round.ours;
    let outcome = match round.theirs {
        Move::Rock => Outcome::Loss,
        Move::Paper => Outcome::Draw,
        Move::Scissors => Outcome::Win,
    };
    let our_move = their_move.our_move(outcome);

    let outcome_val = outcome.inherent_points();
    let move_val = our_move.inherent_points();

    outcome_val + move_val
}

fn main() {
    part1();

    //part();
}

fn part1() {
    let mut sum = 0;

    for round in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
    {
        match round {
            Ok(round) => sum = add_points_p1(sum, round),
            Err(e) => println!("Error {}", e),
        }
    }

    println!("part 1 Final sum of points {} ", sum)
}

fn part2() {
    let mut sum = 0;

    for round in include_str!("../files//input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
    {
        match round {
            Ok(round) => sum = add_points_p2(sum, round),
            Err(e) => println!("Error {}", e),
        }

        //DEBUG
        match round {
            Ok(round) => print_round_p2(round),
            Err(e) => println!("Error {}", e),
        }
    }

    println!("part 2 Final sum of points {} ", sum)
}

//Debugging function for p2
fn print_round_p2(round: Round) {
    let their_move = round.ours;
    let outcome = match round.theirs {
        Move::Rock => Outcome::Loss,
        Move::Paper => Outcome::Draw,
        Move::Scissors => Outcome::Win,
    };
    let our_move = their_move.our_move(outcome);

    // println!(
    //     "Our move: {:?} Their move: {:?}, Outcome: {:?}, round_theirs: {:?}",
    //     our_move, their_move, outcome, round.theirs
    // );

    println!("ours: {:?} theirs: {:?}", round.ours, round.theirs)
}
