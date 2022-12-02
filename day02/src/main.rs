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

        let rock1 = "A";
        let rock2 = "X";

        let paper1 = "B";
        let paper2 = "Y";

        let scissors1 = "C";
        let scissors2 = "Z";

        if (first == rock1 && second == rock2)
            || (first == paper1 && second == paper2)
            || (first == scissors1 && second == scissors2)
        {
            score1 += 3;
        } else if (first == scissors1 && second == rock2)
            || (first == paper1 && second == scissors2)
            || (first == rock1 && second == paper2)
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
            _ => println!("invalid"),
        }

        if second == scissors2 {
            // I need to win
            if first == rock1 {
                score2 += shape_score(paper2);
            } else if first == paper1 {
                score2 += shape_score(scissors2);
            } else if first == scissors1 {
                score2 += shape_score(rock2);
            }
        } else if second == rock2 {
            // I need to loose
            if first == rock1 {
                score2 += shape_score(scissors2);
            } else if first == paper1 {
                score2 += shape_score(rock2);
            } else if first == scissors1 {
                score2 += shape_score(paper2);
            }
        } else if second == paper2 {
            // draw
            if first == rock1 {
                score2 += shape_score(rock2);
            } else if first == paper1 {
                score2 += shape_score(paper2);
            } else if first == scissors1 {
                score2 += shape_score(scissors2);
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
