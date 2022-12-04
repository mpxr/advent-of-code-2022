use itertools::Itertools;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./src/input.txt").unwrap();

    let lines: Vec<String> = raw
        .trim()
        .split('\n')
        .map(|o| o.parse::<String>().unwrap())
        .collect();

    // part 1

    let mut counter = 0;
    for line in lines.iter() {
        if line_overlaps_pt1(line) {
            counter += 1;
        }
    }

    println!("* p1: {}", counter);

    // part 2

    let mut ctr = 0;
    for line in lines.iter() {
        if line_overlaps_pt2(line) {
            ctr += 1;
        }
    }

    println!("** p2: {}", ctr);
}

fn line_overlaps_pt1(line: &str) -> bool {
    let (elf1, elf2) = line.trim().split(",").collect_tuple().unwrap();
    let (elf1_start, elf1_end) = elf1.split("-").collect_tuple().unwrap();
    let (elf2_start, elf2_end) = elf2.split("-").collect_tuple().unwrap();
    compare_ranges_p1((elf1_start, elf1_end), (elf2_start, elf2_end))
}

fn line_overlaps_pt2(line: &str) -> bool {
    let (elf1, elf2) = line.trim().split(",").collect_tuple().unwrap();
    let (elf1_start, elf1_end) = elf1.split("-").collect_tuple().unwrap();
    let (elf2_start, elf2_end) = elf2.split("-").collect_tuple().unwrap();
    compare_ranges_p2((elf1_start, elf1_end), (elf2_start, elf2_end))
}

fn compare_ranges_p1(a: (&str, &str), b: (&str, &str)) -> bool {
    let amin = a.0.parse::<usize>().unwrap();
    let amax = a.1.parse::<usize>().unwrap();
    let bmin = b.0.parse::<usize>().unwrap();
    let bmax = b.1.parse::<usize>().unwrap();

    (amin <= bmin && amax >= bmax) || (amin >= bmin && amax <= bmax)
}

fn compare_ranges_p2(a: (&str, &str), b: (&str, &str)) -> bool {
    let amin = a.0.parse::<usize>().unwrap();
    let amax = a.1.parse::<usize>().unwrap();
    let bmin = b.0.parse::<usize>().unwrap();
    let bmax = b.1.parse::<usize>().unwrap();

    amin <= bmax && bmin <= amax
}
