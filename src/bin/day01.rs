// 1. Install Rust
// 2. To create a new (executable) project: `cargo new day01-rust --bin`
// 3. To run: `cargo run`
// This file is src/main.rs

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let file_name = "inputs/day01.txt"; // &str (rather than a String)
    let frequency_changes: Vec<isize> = read_by_line(file_name).unwrap();

    // part 1: Find last frequency (sum of frequency changes, since starts at 0)
    let sum = sum_vector(&frequency_changes);
    println!("Part 1 answer is {}", sum);

    // part 2: What is the first frequency your device reaches twice?
    let mut current_frequency = 0;
    // To decrease look-up times, we're going to use a HashSet where I might have
    // used a Vector
    let mut recorded_frequencies = HashSet::new();
    recorded_frequencies.insert(current_frequency);

    let mut answer = None;
    // The cycle method is key here-- it makes the for loop go
    // "around the horn" of frequency_changes changes
    for frequency_change in frequency_changes.iter().cycle() {
        // find the new (current) frequency
        current_frequency += frequency_change;

        // now check list (Vector) of recorded_frequencies to see if this new_frequency
        // has occurred before
        if recorded_frequencies.contains(&current_frequency) {
            answer = Some(current_frequency);
            break;
        }
        // add the new_frequency to the list of recorded_frequencies
        recorded_frequencies.insert(current_frequency);
    }
    println!("Part 2 (a frequency a second time): {}", answer.unwrap());
    // for me it's 76787
}

fn sum_vector(vec: &[isize]) -> isize {
    let sum: isize = vec.iter().sum();
    sum
}

// from https://github.com/sts10/eyeoh/blob/master/src/lib.rs#L33
fn read_by_line<T: FromStr>(file_path: &str) -> io::Result<Vec<T>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(_e) => {
                panic!("Error reading a line of the file");
            }
        }
    }
    Ok(vec)
}

#[test]
fn can_sum_up_vec_of_integers() {
    let test_vector: Vec<isize> = [3, -7, 200, 13, -123].to_vec();

    assert_eq!(sum_vector(test_vector), (3 + -7 + 200 + 13 + -123));
}
