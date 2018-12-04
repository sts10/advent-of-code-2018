use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    // let input: Vec<&str> = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x3"];
    // let mut whole_piece: [[usize; 8]; 8] = [[0; 8]; 8];

    let input: Vec<String> = read_by_line("inputs/day03.txt").unwrap();
    let mut whole_piece: [[usize; 1000]; 1000] = [[0; 1000]; 1000];

    // build claims into Vector of Claims
    let mut claims_vec: Vec<Claim> = vec![];
    for input_claim in input {
        claims_vec.push(build_claim(&input_claim));
    }

    // fill in claims to the whole_piece
    for claim in &claims_vec {
        for col_to_fill_in in claim.col..(claim.col + claim.height) {
            for row_to_fill_in in claim.row..(claim.row + claim.width) {
                whole_piece[row_to_fill_in][col_to_fill_in] += 1;
            }
        }
    }

    //print whole_piece
    // for row in whole_piece.iter() {
    //     println!("{:?}", row);
    // }

    // count the overlaps (more-than-ones)
    let mut number_of_overlaps = 0;
    for row in whole_piece.iter() {
        for square_inch in row.iter() {
            if *square_inch > 1 {
                number_of_overlaps += 1;
            }
        }
    }

    // if you're running the test input, this should be 4
    // if using the real input, should be 104241
    println!("Number of overlaps: {}", number_of_overlaps);

    // part 2: Find the one claim that is all 1s
    for claim in &claims_vec {
        let mut this_claim_in_not_overlapped = true;
        for col_to_fill_in in claim.col..(claim.col + claim.height) {
            for row_to_fill_in in claim.row..(claim.row + claim.width) {
                if whole_piece[row_to_fill_in][col_to_fill_in] != 1 {
                    this_claim_in_not_overlapped = false;
                }
            }
        }
        if this_claim_in_not_overlapped {
            println!("Found the not overlapped claim. It's id is #{}", &claim.id);
            break;
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: usize,
    col: usize,
    row: usize,
    width: usize,
    height: usize,
}
fn build_claim(claim: &str) -> Claim {
    let white_space_split = claim.split(' ');
    let white_space_split_vec: Vec<&str> = white_space_split.collect::<Vec<&str>>();

    let mut id_str: String = white_space_split_vec[0].to_string();
    id_str.remove(0);
    let id: usize = id_str.parse::<usize>().unwrap();

    let starting_coordinates: &str = white_space_split_vec[2];
    let starting_coordinates_split = starting_coordinates.split(',');
    let mut starting_coordinates_split_vec: Vec<&str> =
        starting_coordinates_split.collect::<Vec<&str>>();
    starting_coordinates_split_vec[1] =
        starting_coordinates_split_vec[1].trim_matches(|c| c == ':');

    let size: &str = white_space_split_vec[3];
    let size_split = size.split('x');
    let size_split_vec: Vec<&str> = size_split.collect::<Vec<&str>>();

    // build and return the claim
    Claim {
        id: id,
        col: starting_coordinates_split_vec[0]
            .to_string()
            .parse::<usize>()
            .unwrap(),
        row: starting_coordinates_split_vec[1]
            .to_string()
            .parse::<usize>()
            .unwrap(),
        height: size_split_vec[0].to_string().parse::<usize>().unwrap(),
        width: size_split_vec[1].to_string().parse::<usize>().unwrap(),
    }
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
fn can_build_a_claim_struct() {
    let test_claim = "#12 @ 644,603: 29x16";
    let this_claim: Claim = build_claim(test_claim);

    assert_eq!(this_claim.col, 644);
    assert_eq!(this_claim.row, 603);
    assert_eq!(this_claim.height, 29);
    assert_eq!(this_claim.width, 16);
}
