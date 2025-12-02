use advent_of_code_2025::day01::{rotate_dial, split_line};
use advent_of_code_2025::read_input;

fn part1(input_file: &str) -> i32 {
    let limit = 100;
    let mut dial = 50;
    let mut num_zeros = 0;

    if let Ok(lines) = read_input(input_file) {
        for line in lines.map_while(Result::ok) {
            let (direction, num_rotations) = split_line(&line);
            rotate_dial(&direction, &mut dial, &num_rotations);

            dial = dial.rem_euclid(limit);

            if dial == 0 {
                num_zeros += 1;
            }
        }
    }
    num_zeros
}

fn part2(input_file: &str) -> i32 {
    let limit = 100;
    let mut dial = 50;
    let mut num_zeros = 0;

    if let Ok(lines) = read_input(input_file) {
        for line in lines.map_while(Result::ok) {
            let (direction, num_rotations) = split_line(&line);

            let prev_dial = dial;

            rotate_dial(&direction, &mut dial, &num_rotations);

            // How many full turns needed without wrapping
            let full_turns: i32 = dial / limit;
            // Don't care if it's forward or backward
            let mut num_full_turns = full_turns.abs();

            // prev_dial is between 0 and 99, it cannot be negative
            // as we use rem_euclid which returns the nonnegative remainder.
            if dial == 0 || (dial < 0 && prev_dial != 0) {
                num_full_turns += 1;
            }

            dial = dial.rem_euclid(limit);
            num_zeros += num_full_turns;
        }
    }
    num_zeros
}

fn main() {
    let result = part1("./src/bin/day01/input.txt");
    let result2 = part2("./src/bin/day01/input.txt");
    println!("{}, {}", result, result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day01/test.txt"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./src/bin/day01/test.txt"), 6);
    }
}
