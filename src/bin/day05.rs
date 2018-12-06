use std::fs::File;
use std::io;
use std::io::prelude::*;
fn main() {
    // let test_polymer: String = "dabAcCaCBAcCcaDA".to_string();
    // let mut p_vec: Vec<char> = vec![];
    // for c in test_polymer.chars() {
    //     p_vec.push(c);
    // }
    let mut p_vec: Vec<char> = read_string_from_file_to_vector("inputs/day05.txt").unwrap();
    p_vec.pop();
    println!("p_vec is, initially {:?}", p_vec);

    let mut p_vec_len = p_vec.len();
    for _t in 0..1000000000 {
        let mut previous_c: char = p_vec[0];
        let mut indexes_to_remove: Vec<usize> = vec![];
        for c in 1..p_vec_len {
            if do_these_two_chars_cancel(p_vec[c], previous_c) {
                println!("found a pair: {} and {}", previous_c, p_vec[c]);

                // indexes_to_remove.push(c - 1);
                // indexes_to_remove.push(c);
                // p_vec.drain((c - 1)..c);
                p_vec.remove(c);
                p_vec.remove(c - 1);
                // println!("p_vec is {:?}", p_vec);
                break;
            } else {
                previous_c = p_vec[c];
            }
        }
        // println!("polymer is now {:?}", p_vec);
        // println!("indexes_to_remove is now {:?}", indexes_to_remove);
        // do the removing
        // for i in &indexes_to_remove {
        //     p_vec.remove(*i);
        // }
        if p_vec.len() == p_vec_len {
            break;
        } else {
            p_vec_len = p_vec.len();
        }
    }
    println!(
        "polymer length is {}, which is likely 1 fewer than {}",
        p_vec.len(),
        p_vec_len
    );
    // let s: String = p_vec.into_iter().collect();
    // println!("polymer is now {:?}", s);
}

fn do_these_two_chars_cancel(a: char, b: char) -> bool {
    // if c is upcase && previous_c is downcase && c.downcase() = previous_c
    if a.is_uppercase() && b.is_lowercase() && a.to_lowercase().to_string() == b.to_string() {
        true
    } else if a.is_lowercase() && b.is_uppercase() && a.to_uppercase().to_string() == b.to_string()
    {
        true
    } else {
        false
    }
}

#[test]
fn can_find_polymer_pair() {
    assert_eq!(do_these_two_chars_cancel('a', 'A'), true);
    assert_eq!(do_these_two_chars_cancel('d', 'd'), false);
    assert_eq!(do_these_two_chars_cancel('E', 'E'), false);
    assert_eq!(do_these_two_chars_cancel('F', 'g'), false);
    assert_eq!(do_these_two_chars_cancel('H', 'h'), true);
}
fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}
