use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let vector_of_box_ids: Vec<String> = read_by_line("inputs/day02.txt").unwrap();

    // part 1
    println!(
        "checksum that should be 12 is {}",
        get_checksum(&vector_of_box_ids)
    );

    // part 2
    let number_of_ids = vector_of_box_ids.len();

    for index_of_box_id in 0..number_of_ids {
        for index_of_box_id_to_compare in 0..number_of_ids {
            if let Some(common_characters) =
                find_common_characters_if_there_is_only_one_that_is_different(
                    &vector_of_box_ids[index_of_box_id],
                    &vector_of_box_ids[index_of_box_id_to_compare],
                ) {
                println!("common characters are {}", common_characters);
                if common_characters == "lujnogabetpmsydyfcovzixaw" {
                    println!("Correct!");
                }
            }
        }
    }
}

fn find_common_characters_if_there_is_only_one_that_is_different(
    a: &str,
    b: &str,
) -> Option<String> {
    let mut a_vec: Vec<char> = [].to_vec();
    let mut b_vec: Vec<char> = [].to_vec();
    let mut common_characters: String = "".to_string();

    // still not loving this rigamarole
    for c in a.chars() {
        a_vec.push(c);
    }
    for c in b.chars() {
        b_vec.push(c);
    }
    let mut how_many_characters_are_different = 0;
    for (index, c) in a_vec.iter().enumerate() {
        if *c != b_vec[index] {
            how_many_characters_are_different += 1;
        } else {
            // add c to the end of common_characters using format!
            common_characters = format!("{}{}", common_characters, *c);
        }
    }
    if how_many_characters_are_different == 1 {
        Some(common_characters)
    } else {
        None
    }
}

fn get_checksum(vector_of_box_ids: &Vec<String>) -> usize {
    // let mut checksum = 0;
    let mut boxes_with_twice = 0;
    let mut boxes_with_thrice = 0;

    for box_id in vector_of_box_ids {
        let (twice, thrice) = analyze_box_id(&box_id);
        if twice {
            boxes_with_twice += 1;
        }
        if thrice {
            boxes_with_thrice += 1;
        }
    }
    boxes_with_twice * boxes_with_thrice
}
fn analyze_box_id(box_id: &str) -> (bool, bool) {
    let mut box_id_hashmap: HashMap<char, usize> = HashMap::new();
    let mut has_letter_twice: bool = false;
    let mut has_letter_thrice: bool = false;

    for c in box_id.chars() {
        box_id_hashmap
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for box_pair in box_id_hashmap {
        if box_pair.1 == 2 {
            has_letter_twice = true;
        } else if box_pair.1 == 3 {
            has_letter_thrice = true;
        }
    }
    (has_letter_twice, has_letter_thrice)
}

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
fn can_analyze_a_box_id() {
    assert_eq!(analyze_box_id("abcdef"), (false, false)); // contains no letters that appear exactly two or three times.
    assert_eq!(analyze_box_id("bababc"), (true, true)); //  contains two a and three b, so it counts for both.
    assert_eq!(analyze_box_id("abbcde"), (true, false)); //  contains two b, but no letter appears exactly three times.
    assert_eq!(analyze_box_id("abcccd"), (false, true)); // contains three c, but no letter appears exactly two times.
    assert_eq!(analyze_box_id("aabcdd"), (true, false)); // contains two a and two d, but it only counts once.
    assert_eq!(analyze_box_id("abcdee"), (true, false)); // contains two e.
    assert_eq!(analyze_box_id("ababab"), (false, true)); // contains three a and three b, but it only counts once.
}
