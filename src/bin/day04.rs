use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
fn main() {
    // let mut events_vec: Vec<String> = read_by_line("inputs/day04-test.txt").unwrap();
    let mut events_vec: Vec<String> = read_by_line("inputs/day04.txt").unwrap();

    // 1. Sort Events in chronological order (just do it alphabetically)
    events_vec.sort();
    // build a vector of structs of these events
    let mut events_structs_vec: Vec<Event> = vec![];
    for event in &events_vec {
        events_structs_vec.push(build_event_structs(event));
    }
    // may need to do a .sort_by(|event| {event.date_time }); here but I'm not sure

    // At this point, I have a Vector of Events sorted by time
    // So, from this point, the date doesn't matter

    // 2. Build Guard structs into a Hashmap
    let mut guards_map: HashMap<usize, [usize; 60]> = HashMap::new();
    // initialize guards map with the guard id and empty minutes arrays of 60 zeros
    for event in events_structs_vec.iter() {
        match event.guard_id {
            Some(id) => {
                guards_map.entry(id).or_insert([0; 60]);
            }
            None => continue,
        }
    }

    // fill in guards_map arrays
    let mut minute_of_previous_event = 0;
    let mut guard_of_previous_event = 10;
    let mut asleep_of_previous_event = false;

    for next_event in events_structs_vec.iter().skip(1) {
        let next_event_minute = if next_event.hour > 20 {
            0
        } else {
            next_event.minute
        };
        if next_event_minute < minute_of_previous_event {
            // assume it's a new night
            minute_of_previous_event = 0;
        }
        // maybe something wrong with this asleep bool
        if asleep_of_previous_event {
            for m in minute_of_previous_event..next_event_minute {
                guards_map
                    .entry(guard_of_previous_event)
                    .and_modify(|arr| arr[m as usize] += 1);
            }
        }
        minute_of_previous_event = next_event_minute;
        guard_of_previous_event = match next_event.guard_id {
            Some(id) => id,
            None => guard_of_previous_event,
        };
        asleep_of_previous_event = next_event.asleep;
    }

    // for guard in &guards_map {
    //     let mut i = 0;
    //     for m in guard.1.iter() {
    //         i += 1;
    //     }
    // }

    // 4. Find the Guard with highest number of minutes asleep (we'll call
    //    him "Sleepy")
    let mut number_of_minutes_the_sleepiest_guard_slept: usize = 0;
    let mut sleepiest_guard_id: usize = 10;
    for guard in &guards_map {
        let mut this_guards_total_minutes_asleep: usize = 0;
        for m in guard.1.iter() {
            this_guards_total_minutes_asleep += m;
        }
        if this_guards_total_minutes_asleep > number_of_minutes_the_sleepiest_guard_slept {
            sleepiest_guard_id = *guard.0;
            number_of_minutes_the_sleepiest_guard_slept = this_guards_total_minutes_asleep;
        }
    }
    println!(
        "The id of the sleepiest guard is {}, who was asleep for {}",
        sleepiest_guard_id, number_of_minutes_the_sleepiest_guard_slept
    );

    // 5. Find which minute Sleepy is most often asleep
    let sleepiest_guard_minute_array = guards_map[&sleepiest_guard_id];
    let mut sleepiest_minute_position = 1000;
    let mut sleepiest_minute_amount_slept = 0;
    // let mut minute_number = 0;
    for (minute_number, &amount_slept_that_minute) in
        sleepiest_guard_minute_array.iter().enumerate()
    {
        if amount_slept_that_minute > sleepiest_minute_amount_slept {
            println!("Found a new sleepiest minute");
            sleepiest_minute_position = minute_number;
            sleepiest_minute_amount_slept = amount_slept_that_minute;
        }
        // minute_number += 1;
    }
    println!("the miniute that Sleepy is most often asleep is {}. He was asleep during that minute for {} minutes", sleepiest_minute_position, sleepiest_minute_amount_slept);

    println!(
        "So! The answer to part 1 is {}",
        sleepiest_guard_id * sleepiest_minute_position
    );
    assert_eq!(sleepiest_guard_id * sleepiest_minute_position, 35184);

    // Part 2: Of all guards, which guard is most frequently asleep on the same minute?
    // In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total.
    // (In all other cases, any guard spent any minute asleep at most twice.)
    // What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 99 * 45 = 4455.)

    let mut guard_with_highest_peak: usize = 1000;
    let mut x_position_of_highest_peak = 1000;
    let mut height_of_highest_peak: usize = 0;
    for guard in &guards_map {
        // find this guards sleepiest minute
        let this_guard_minutes_slept_array = guard.1;
        for (minute_number, &amount_slept_that_minute) in
            this_guard_minutes_slept_array.iter().enumerate()
        {
            if amount_slept_that_minute > height_of_highest_peak {
                println!("Found a new highest peak");
                x_position_of_highest_peak = minute_number;
                height_of_highest_peak = amount_slept_that_minute;
                guard_with_highest_peak = *guard.0;
            }
        }
    }
    println!(
        "Part 2: Guard id is {} and the minute they were asleep the most was {}",
        guard_with_highest_peak, x_position_of_highest_peak
    );
    println!(
        "Multiply them and you get {}",
        guard_with_highest_peak * x_position_of_highest_peak
    );
    assert_eq!(guard_with_highest_peak * x_position_of_highest_peak, 37886);
}

fn build_event_structs(event_string: &str) -> Event {
    // [1518-11-04 00:02] Guard #99 begins shift
    // [1518-11-03 00:24] falls asleep
    let white_space_split_vec: Vec<&str> = split_and_vectorize(event_string, " ");
    let mut date_time: String =
        white_space_split_vec[0].to_owned() + " " + white_space_split_vec[1];

    date_time.remove(0);
    date_time.pop();

    let time_split_vec: Vec<&str> = split_and_vectorize(white_space_split_vec[1], ":");

    let hour = time_split_vec[0].parse::<u32>().unwrap();

    let mut minutes_str: String = time_split_vec[1].to_string();
    minutes_str.pop();
    let minute = minutes_str.parse::<u32>().unwrap();

    let guard_id: Option<usize> = if white_space_split_vec[2] == "Guard" {
        let mut guard_id_string = white_space_split_vec[3].to_string();
        guard_id_string.remove(0);
        // println!("Guard id is {} ", guard_id_string);
        match guard_id_string.parse::<usize>() {
            Ok(id) => Some(id),
            Err(_e) => None,
        }
    } else {
        None
    };

    // now assign asleep either true (if "begins shift" or "wakes up") or false (if "falls asleep")

    //  [1518-03-06 23:59] Guard #997 begins shift
    //  [1518-11-09 00:21] falls asleep
    //  [1518-06-18 00:55] wakes up
    let asleep = !(white_space_split_vec[2] == "Guard" || white_space_split_vec[2] == "wakes");

    Event {
        date_time, // this notation is equivalent to `date_time: date_time,`
        minute,
        hour,
        guard_id,
        asleep,
    }
}

#[derive(Debug)]
struct Event {
    date_time: String,
    hour: u32,
    minute: u32,
    guard_id: Option<usize>,
    asleep: bool,
}

fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    let split = string_to_split.split(splitter);
    split.collect::<Vec<&str>>()
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

// Notes
//
// on the `peek` method:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=b9c0979cb1319898ed6077c0114b3a4d
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=f4f826811e89f013877a4449e73dfde9
