use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|o| o.unwrap()).collect();

    // part 1

    let mut p1_priority_sum = 0;
    for line in lines.iter() {
        let first_half_set = to_char_set(&line, 0, line.chars().count() / 2);
        let second_half_set = to_char_set(
            &line,
            line.chars().count() / 2,
            line.chars().count(),
        );

        let intersection: Vec<&char> = first_half_set.intersection(&second_half_set).collect();

        p1_priority_sum += calc_prio_sum(intersection);
    }
    println!("* Sum of the priorities: {}", p1_priority_sum);

    // part 2

    let mut p2_priority_sum = 0;
    for chunks in lines.chunks(3) {
        let first_set = to_char_set(&chunks[0], 0, chunks[0].chars().count());
        let second_set = to_char_set(&chunks[1], 0, chunks[1].chars().count());
        let third_set = to_char_set(&chunks[2], 0, chunks[2].chars().count());

        let intersection: Vec<&char> = first_set.intersection(&second_set).collect();

        let first_intersection: String = intersection.iter().map(|c| *c).collect();

        let one_two_set = to_char_set(&first_intersection, 0, first_intersection.chars().count());

        let intersection: Vec<&char> = one_two_set.intersection(&third_set).collect();

        p2_priority_sum += calc_prio_sum(intersection);
    }

    println!("** Sum of the priorities: {}", p2_priority_sum);

    Ok(())
}

fn calc_prio_sum(intersection: Vec<&char>) -> usize {
    let lower_case_alphabet = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    let upper_case_alphabet = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    let mut sum = 0;
    for c in intersection.iter() {
        if lower_case_alphabet.contains(c) {
            sum += lower_case_alphabet.iter().position(|&r| r == **c).unwrap() + 1;
        } else {
            sum += lower_case_alphabet.len()
                + upper_case_alphabet.iter().position(|&r| r == **c).unwrap()
                + 1;
        }
    }
    sum
}

fn to_char_set(line: &String, skip: usize, take: usize) -> HashSet<char> {
    let sub_string: String = line.chars().skip(skip).take(take).collect();
    let char_set: HashSet<char> = HashSet::from_iter(sub_string.chars());
    char_set
}
