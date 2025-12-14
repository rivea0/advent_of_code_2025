use std::fs;

fn part1(input_file: &str) -> u64 {
    let s = fs::read_to_string(input_file).unwrap();
    let mut total = 0;

    let ops_vec = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    let nums = s
        .lines()
        .take(s.lines().count() - 1)
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut by_column = vec![];

    // All the same size
    let col_length = nums[0].len();

    'outer: for _ in &nums {
        for col_idx in 0..col_length {
            if by_column.len() > 0 && col_idx == 0 {
                break 'outer;
            }
            by_column.push(nums.iter().map(|v| v[col_idx]).collect::<Vec<_>>());
        }
    }

    for i in 0..ops_vec.len() {
        let op = ops_vec[i];
        let res = match op {
            "*" => by_column[i]
                .iter()
                .fold(1, |acc, i| acc * (i.parse::<u64>().unwrap())),
            "+" => by_column[i]
                .iter()
                .fold(0, |acc, i| acc + (i.parse::<u64>().unwrap())),
            _ => 0_u64,
        };
        total += res;
    }

    total
}

fn main() {
    println!("{}", part1("./src/bin/day06/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day06/test.txt"), 4277556_u64);
    }
}
