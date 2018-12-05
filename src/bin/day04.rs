extern crate chrono;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    println!("Hello world!");
    let mut events_vec: Vec<String> = read_by_line("inputs/day04-test.txt").unwrap();
    println!("Events\n{:?}", events_vec);

    // 1. Sort Events in chronological order
    // 1a. Parse string from input into: Y-

    // let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
    events_vec.sort();
    println!("Events, sorted \n{:#?}", events_vec);

    let mut events_structs_vec: Vec<Event> = vec![];
    for event in &events_vec {
        events_structs_vec.push(build_event_structs(event.to_string()));
    }
    // may need to do a .sort_by(|event| {event.date_time }); here but I'm not sure

    // at this point, I have a Vector of Events sorted by time

    //
    // From this point, the date doesn't matter
    //

    // 1b. Go through events again and add a new parameter
    // minutes_till_next_event_or_1am
    // and (maybe) guard_id?

    // 2. Build Guard structs into a Hashmap
    let mut guards: HashMap<usize, Guard> = HashMap::new();

    let mut iter = events_structs_vec.into_iter().peekable();

    loop {
        match iter.next() {
            Some(event) => {
                // println!("event is {:?}, peek is {:?}", event, iter.peek());
                // let length_of_this_event = event.dt - iter.peek().unwrap().dt;
                let length_of_this_event = iter.peek().unwrap().dt - event.dt;
                println!("Length of this event is {:?}", length_of_this_event);
            }
            _ => break,
        }
    }
    // event is: guard 10 wakes up
    //
    // guards.entry(guard)
    // minutes_between_midnight_and_1am_asleep
    //     .entry(m)
    //     .and_modify(|count| *count += 1)
    //     .or_insert(1);
    // }
    // 3. Iterating through a collection of the Events, update
    //    (a) this_guard.minutes_between_midnight_and_1am_asleep and
    //    (b) this_guard.number_of_minutes_between_midnight_and_1am_asleep
    // 4. Find the Guard with highest number of minutes asleep (we'll call
    //    him "Sleepy")
    // 5. Find which minutes_between_midnight_and_1am that Sleepy is most
    //    often asleep
}

fn build_event_structs(event_string: String) -> Event {
    println!("event string: {}", event_string);
    // [1518-11-04 00:02] Guard #99 begins shift
    // [1518-11-03 00:24] falls asleep
    let white_space_split = event_string.split(' ');
    let white_space_split_vec: Vec<&str> = white_space_split.collect::<Vec<&str>>();
    let mut date_time: String =
        white_space_split_vec[0].to_owned() + " " + white_space_split_vec[1];

    // let date_time_len = &date_time.len();
    date_time.remove(0);
    date_time.pop();
    // println!("date_time: {}", date_time);

    // let date_split = white_space_split_vec[0].split("-");
    // let date_split_vec = date_split.collect::<Vec<&str>>();
    // let year = date_split_vec[0].parse::<i32>().unwrap();
    // let month = date_split_vec[1].parse::<u32>().unwrap();
    // let day = date_split_vec[2].parse::<u32>().unwrap();

    // let time_split = white_space_split_vec[1].split(":");
    // let time_split_vec: Vec<&str> = time_split.collect::<Vec<&str>>();
    // let hour = time_split_vec[0].parse::<u32>().unwrap();
    // let minute = time_split_vec[1].parse::<u32>().unwrap();

    // let dt: NaiveDateTime = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 0);
    // "%Y-%m-%d %H:%M:%S %z"
    let dt: NaiveDateTime = NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M").unwrap();

    let guard_id: Option<usize>;
    if white_space_split_vec[2] == "Guard" {
        let mut guard_id_string = white_space_split_vec[3].to_string();
        guard_id_string.remove(0);
        // println!("Guard id is {} ", guard_id_string);
        guard_id = match guard_id_string.parse::<usize>() {
            Ok(id) => Some(id),
            Err(_e) => None,
        };
    } else {
        guard_id = None;
    }
    println!("This event's guard's id is {:?}", guard_id);

    // now assign asleep either true (if "begins shift" or "wakes up") or false (if "falls asleep")

    //  [1518-03-06 23:59] Guard #997 begins shift
    //  [1518-11-09 00:21] falls asleep
    //  [1518-06-18 00:55] wakes up
    let asleep: bool;
    if white_space_split_vec[2] == "Guard" || white_space_split_vec[2] == "wakes" {
        asleep = false;
    } else {
        asleep = true;
    }
    println!("Is this guard asleep? {}", asleep);

    Event {
        date_time: date_time,
        dt: dt,
        guard_starting_id: guard_id,
        asleep: asleep,
        number_of_minutes_till_next_event_or_1am: None,
    }
}

#[derive(Debug)]
struct Event {
    // Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z
    date_time: String,
    dt: NaiveDateTime,
    // year: usize,
    // month: usize,
    // day: usize,
    // hour: u32,
    // minute: u32,
    guard_starting_id: Option<usize>,
    asleep: bool,
    number_of_minutes_till_next_event_or_1am: Option<u32>,
}
#[derive(Debug)]
struct Guard {
    id: usize,
    minutes_between_midnight_and_1am_asleep: HashMap<usize, usize>,
    number_of_minutes_between_midnight_and_1am_asleep: usize,
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
