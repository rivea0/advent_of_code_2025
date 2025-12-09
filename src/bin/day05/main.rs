use advent_of_code_2025::day05::{Range, merge_ranges};
use std::fs;

fn part1(input_file: &str) -> u64 {
    let s = fs::read_to_string(input_file).unwrap();
    let mut result = 0;
    let splitted = s.split("\n").collect::<Vec<_>>();
    let idx = splitted.iter().position(|x| *x == "");
    let (ranges, available) = splitted.split_at(idx.unwrap());
    let ranges = ranges
        .iter()
        .map(|x| {
            let parts = x.split("-").collect::<Vec<_>>();
            return Range {
                start: parts[0].parse::<u64>().unwrap(),
                end: parts[1].parse::<u64>().unwrap(),
            };
        })
        .collect::<Vec<Range>>();
    let available = available.iter().filter(|&&x| x != "").collect::<Vec<_>>();

    let merged = merge_ranges(ranges);

    for n in available {
        let num = n.parse::<u64>().unwrap();
        for range in merged.iter() {
            if num >= range.start && num <= range.end {
                result += 1;
            }
        }
    }

    result
}

fn part2(input_file: &str) -> u64 {
    let s = fs::read_to_string(input_file).unwrap();
    let mut result = 0;
    let splitted = s.split("\n").collect::<Vec<_>>();
    let idx = splitted.iter().position(|x| *x == "");
    let (ranges, _) = splitted.split_at(idx.unwrap());
    let ranges = ranges
        .iter()
        .map(|x| {
            let parts = x.split("-").collect::<Vec<_>>();
            return Range {
                start: parts[0].parse::<u64>().unwrap(),
                end: parts[1].parse::<u64>().unwrap(),
            };
        })
        .collect::<Vec<Range>>();

    let merged = merge_ranges(ranges);

    for range in merged.iter() {
        // Plus 1 to include the start
        result += (range.end - range.start) + 1;
    }

    result
}

fn main() {
    println!("{}", part1("./src/bin/day05/input.txt"));
    println!("{}", part2("./src/bin/day05/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day05/test.txt"), 3_u64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./src/bin/day05/test.txt"), 14_u64);
    }
}
