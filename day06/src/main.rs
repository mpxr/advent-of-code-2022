use std::collections::HashSet;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./src/input.txt").unwrap();

    let char_vec: Vec<char> = raw.chars().collect();

    let marker_length = 4;
    for (idx, window) in char_vec.as_slice().windows(marker_length).enumerate() {
        let char_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if char_set.len() == marker_length {
            println!("* Result: {}", idx + 4);
            break;
        }
    }

    let marker_length = 14;
    for (idx, window) in char_vec.as_slice().windows(marker_length).enumerate() {
        let char_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if char_set.len() == marker_length {
            println!("** Result: {}", idx + 14);
            break;
        }
    }
}
