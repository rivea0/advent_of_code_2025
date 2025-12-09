use advent_of_code_2025::read_input;

fn part1(input_file: &str) -> u16 {
    let mut result = 0;
    if let Ok(lines) = read_input(input_file) {
        for line in lines.map_while(Result::ok) {
            let mut max_joltage: u16 = 0;
            // terrible
            for i in 0..line.chars().count() {
                for j in (i + 1)..line.chars().count() {
                    let first_digit = line.chars().nth(i).unwrap();
                    let second_digit = line.chars().nth(j).unwrap();
                    let num = format!("{}{}", first_digit, second_digit);
                    let n = num.parse::<u16>().unwrap();
                    if n > max_joltage {
                        max_joltage = n;
                    }
                }
            }

            result += max_joltage;
        }
    }

    result
}

fn main() {
    println!("{}", part1("./src/bin/day03/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day03/test.txt"), 357_u16);
    }
}
