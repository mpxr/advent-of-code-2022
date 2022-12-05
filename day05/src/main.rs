use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn main() {
    let raw = fs::read_to_string("./src/input.txt").unwrap();

    let (cargo, instructions) = raw.split("\n\n").collect_tuple().unwrap();

    let lines: Vec<String> = cargo
        .split('\n')
        .map(|o| o.parse::<String>().unwrap())
        .collect();

    let mut p1_stacks: Vec<VecDeque<String>> = Vec::new();

    let number_of_stacks = lines[lines.len() - 1]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    for _ in 0..number_of_stacks {
        p1_stacks.push(VecDeque::new());
    }

    for (cargo_idx, line) in lines.iter().enumerate() {
        if cargo_idx < cargo.len() - 2 {
            for (idx, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let val = chunk.skip(1).step_by(4).format("").to_string();
                if val.trim().len() > 0 {
                    p1_stacks[idx].push_front(val);
                }
            }
        }
    }

    let mut p2_stacks = p1_stacks.clone();

    let lines: Vec<String> = instructions
        .trim()
        .split('\n')
        .map(|o| o.parse::<String>().unwrap())
        .collect();

    for line in lines.iter() {
        let (_, stack_num, _, from, _, to) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let stack_num = stack_num.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();

        // part 1
        for _ in 0..stack_num {
            let v = p2_stacks[from - 1].pop_back().unwrap();
            p2_stacks[to - 1].push_back(v);
        }

        // part 2
        let mut temp_queue: VecDeque<String> = VecDeque::new();
        for _ in 0..stack_num {
            let v = p1_stacks[from - 1].pop_back().unwrap();
            temp_queue.push_back(v);
        }
        for _ in 0..temp_queue.len() {
            p1_stacks[to - 1].push_back(temp_queue.pop_back().unwrap());
        }
    }

    let p1_result = get_last_chars(p2_stacks);
    println!("* Part 1 result: {}", p1_result);

    let p2_result = get_last_chars(p1_stacks);
    println!("** Part 2 result: {}", p2_result);
}

fn get_last_chars(p2_stacks: Vec<VecDeque<String>>) -> String {
    let mut result = String::new();
    for current_stack in p2_stacks.iter() {
        let last = current_stack.back().unwrap();
        result.push_str(&last);
    }
    result
}
