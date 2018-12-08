fn main() {
    let test_input: Vec<&str> = vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
    let input_vec: Vec<(usize, usize)> = parse_input_vector(test_input);
    let signs: Vec<&str> = vec!["A", "B", "C", "D", "E", "F"];

    let mut grid: [[&str; 10]; 10] = [["."; 10]; 10];

    for (sign_index, coord_to_place) in input_vec.iter().enumerate() {
        let x = coord_to_place.0;
        let y = coord_to_place.1;
        grid[y][x] = &signs[sign_index];
    }

    print_grid(grid);

    // 1. Write  a `search` function:  Looks for characters, calls `spread(x,y)`
    // 2. fn spread(initial_coord: (x,y), grid) -> {}
    //   a. Look in all directions.
    //      match found {
    //        "." => fill it with uppercase // found an empty space.
    //        "upper" => change it to "," // found a frontier. challenge it
    //        "lower => do nothing // found claimed land
    // 3. When search has gone through the whole grid, go through again and make everything lowercase
    //
    //
}

fn search_and_spread(grid: [[&str; 10]; 10]) {
    for (x, _col) in grid.iter().enumerate() {
        for (y, &space) in grid[x].iter().enumerate() {
            if space != "." && space != "," {
                let new_grid = spread(x, y, grid);
            }
        }
    }
}
fn spread(x: usize, y: usize, grid: [[&str; 10]; 10]) -> [[&str; 10]; 10] {
    let sign = grid[y][x];
    let nine_squares: Vec<&str> = vec![
        grid[y][x + 1],
        grid[y][x - 1],
        grid[y + 1][x],
        grid[y - 1][x],
        grid[y + 1][x + 1],
        grid[y - 1][x - 1],
        grid[y + 1][x - 1],
        grid[y - 1][x + 1],
    ];
    for square in nine_squares {
        match square {
            "." => square = sign,
            _ => (),
        }
    }
    grid
}
fn print_grid(grid: [[&str; 10]; 10]) {
    for (i, _row) in grid.iter().enumerate() {
        for col in grid[i].iter() {
            print!("{}", col);
        }
        println!();
    }
}

fn parse_input_vector(input: Vec<&str>) -> Vec<(usize, usize)> {
    let mut vector_ints: Vec<(usize, usize)> = vec![];
    // pattern is x, y
    for coord_string in input {
        let x: usize = split_and_vectorize(coord_string, ",")[0].parse().unwrap();
        let y: usize = split_and_vectorize(coord_string, " ")[1].parse().unwrap();
        vector_ints.push((x, y));
    }
    vector_ints
}

fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    let split = string_to_split.split(splitter);
    split.collect::<Vec<&str>>()
}

#[test]
fn can_parse_input() {
    let test_input: Vec<&str> = vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
    let input_vec: Vec<(usize, usize)> = parse_input_vector(test_input);
    assert_eq!(input_vec[1].1, 6);
    assert_eq!(input_vec[2], (8, 3));
}
