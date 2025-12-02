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

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
pub fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
