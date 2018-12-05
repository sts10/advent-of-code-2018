use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    println!("Hello world!");
    let events_vec: Vec<String> = read_by_line("inputs/day04-test.txt").unwrap();
    println!("Events\n{:?}", events_vec);
    // 1. Sort Events in chronological order
    // 2. Build Guard structs
    // 3. Iterating through a collection of the Events, update
    //    (a) this_guard.minutes_between_midnight_and_1am_asleep and
    //    (b) this_guard.number_of_minutes_between_midnight_and_1am_asleep
    // 4. Find the Guard with highest number of minutes asleep (we'll call
    //    him "Sleepy")
    // 5. Find which minutes_between_midnight_and_1am that Sleepy is most
    //    often asleep
}

// #[derive(Debug)]
// struct Guard {
//     id: usize,
//     minutes_between_midnight_and_1am_asleep: Hashmap,
//     number_of_minutes_between_midnight_and_1am_asleep: usize,
// }

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
fn can_sort_list_of_events_chronologically() {
    // [1518-11-01 00:00] Guard #10 begins shift
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:25] wakes up
    // [1518-11-01 00:30] falls asleep
    // [1518-11-01 00:55] wakes up
    // [1518-11-01 23:58] Guard #99 begins shift
    // [1518-11-02 00:40] falls asleep
    // [1518-11-02 00:50] wakes up
    // [1518-11-03 00:05] Guard #10 begins shift
    // [1518-11-03 00:24] falls asleep
    // [1518-11-03 00:29] wakes up
    // [1518-11-04 00:02] Guard #99 begins shift
    // [1518-11-04 00:36] falls asleep
    // [1518-11-04 00:46] wakes up
    // [1518-11-05 00:03] Guard #99 begins shift
    // [1518-11-05 00:45] falls asleep
    // [1518-11-05 00:55] wakes up
}
