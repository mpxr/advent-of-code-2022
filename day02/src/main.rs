use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut score1 = 0;
    let mut score2 = 0;
    for line in reader.lines() {
        let line = line?;
        let elems: Vec<&str> = line.split(' ').collect();
        let first = elems[0];
        let second = elems[1];

        // part 1:

        if (rock(first) && rock(second))
            || (paper(first) && paper(second))
            || (scissors(first) && scissors(second))
        {
            score1 += 3;
        } else if (scissors(first) && rock(second))
            || (paper(first) && scissors(second))
            || (rock(first) && paper(second))
        {
            score1 += 6;
        } else {
            score1 += 0;
        }

        score1 += shape_score(second);

        // part 2:

        match second {
            "X" => score2 += 0, //loose
            "Y" => score2 += 3, //draw
            "Z" => score2 += 6, //win
            _ => score2 += 0
        }

        const ROCK_SCORE:usize = 1;
        const PAPER_SCORE:usize = 2;
        const SCISSORS_SCORE:usize = 3;

        if scissors(second) {
            // I need to win
            if rock(first) {
                score2 += PAPER_SCORE;
            } else if paper(first) {
                score2 += SCISSORS_SCORE;
            } else if scissors(first) {
                score2 += ROCK_SCORE;
            }
        } else if rock(second) {
            // I need to loose
            if rock(first) {
                score2 += SCISSORS_SCORE;
            } else if paper(first) {
                score2 += ROCK_SCORE;
            } else if scissors(first) {
                score2 += PAPER_SCORE;
            }
        } else if paper(second) {
            // draw
            if rock(first) {
                score2 += ROCK_SCORE;
            } else if paper(first) {
                score2 += PAPER_SCORE;
            } else if scissors(first) {
                score2 += SCISSORS_SCORE;
            }
        }
    }
    println!("* Plan 1 score: {}", score1);
    println!("* Plan 2 score: {}", score2);

    Ok(())
}

fn shape_score(val: &str) -> i32 {
    match val {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn rock(val: &str) -> bool {
    if val == "X" || val == "A" {
        return true;
    }
    return false;
}

fn paper(val: &str) -> bool {
    if val == "Y" || val == "B" {
        return true;
    }
    return false;
}

fn scissors(val: &str) -> bool {
    if val == "Z" || val == "C" {
        return true;
    }
    return false;
}
