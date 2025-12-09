use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day01 {
    pub fn rotate_dial(direction: &str, dial: &mut i32, n: &i32) {
        if direction == "L" {
            *dial += -n;
        } else if direction == "R" {
            *dial += n;
        }
    }

    pub fn split_line(line: &String) -> (&str, i32) {
        let result = line
            .split_inclusive(char::is_uppercase)
            .collect::<Vec<&str>>();
        let direction = *result.get(0).expect("No direction");
        let num_rotations = result
            .get(1)
            .expect("No number of rotations")
            .parse::<i32>()
            .expect("Cannot parse number");

        (direction, num_rotations)
    }
}

pub mod day05 {
    #[derive(Debug)]
    pub struct Range {
        pub start: u64,
        pub end: u64,
    }

    pub fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
        ranges.sort_by_key(|r| r.start);
        let mut result: Vec<Range> = Vec::new();
        for range in ranges {
            // Extend the previous range with the current range
            // if it's overlapping.
            if let Some(last) = result.last_mut() {
                if range.start <= last.end {
                    last.end = last.end.max(range.end);
                    continue;
                }
            }

            result.push(range);
        }

        result
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
pub fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
