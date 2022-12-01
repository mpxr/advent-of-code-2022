use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut elf_calories = 0;
    for line in reader.lines() {
        let line = line?;
        if line.trim().len() > 0 {
            elf_calories += line.parse::<usize>().unwrap();
        } else {
            heap.push(elf_calories);
            elf_calories = 0;
        }
    }

    println!(
        "* The Elf carrying the most Calories is {}",
        heap.peek().expect("Oouch")
    );
    println!(
        "** The top three Elves carrying {} calories",
        heap.pop().expect("Not a number")
            + heap.pop().expect("Not a number")
            + heap.pop().expect("Not a number")
    );

    Ok(())
}
