use std::fs;

fn get_line(input_file: &str) -> Result<String, std::io::Error> {
    let line = fs::read_to_string(input_file)?;

    Ok(line)
}

fn part1(input_file: &str) -> u64 {
    let mut result = 0;
    let line = get_line(input_file).expect("Cannot get line");
    let ranges = line.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
    for range in ranges.iter() {
        let parts = range.split("-").collect::<Vec<_>>();
        let (start_range, end_range) = (parts[0], parts[1]);
        // If length is not even, it cannot be made of a twice-repeating sequence.
        // It's also invalid if starts with 0.
        if (start_range.chars().count() % 2 != 0 && end_range.chars().count() % 2 != 0)
            || start_range.starts_with("0")
        {
            continue;
        }

        let start = start_range.parse::<u64>().expect("Cannot get start range");
        let end = end_range.parse::<u64>().expect("Cannot get end range");

        for i in start..=end {
            let n = i.to_string();
            let (first_half, second_half) = n.split_at(n.chars().count() / 2);
            if first_half == second_half {
                result += i;
            }
        }
    }
    result
}

fn part2() {

}

fn main() {
    println!("{}", part1("./src/bin/day02/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day02/test.txt"), 1227775554_u64);
    }
}
