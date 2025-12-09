use advent_of_code_2025::read_input;

fn part1(input_file: &str) -> u16 {
    let mut result = 0;

    if let Ok(lines_iter) = read_input(input_file) {
        let lines = lines_iter
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        let length = lines.len();
        for i in 0..length {
            let line = &lines[i];
            let next_line = if i + 1 < length {
                Some(&lines[i + 1])
            } else {
                None
            };
            let prev_line = if i > 0 { Some(&lines[i - 1]) } else { None };
            for (j, c) in line.char_indices() {
                if c == '@' {
                    let mut total = 0;
                    match (i, j) {
                        // Corners automatically qualify
                        // as there is only 3 adjacent cells so it's always less than 4.

                        // Top left corner
                        (0, 0) => {
                            total += 1;
                        }
                        // Top right corner
                        (0, j_val) if j_val == line.chars().count() - 1 => {
                            total += 1;
                        }
                        // Bottom left corner
                        (i_val, 0) if i_val == length - 1 => {
                            total += 1;
                        }
                        // Bottom right corner
                        (i_val, j_val)
                            if i_val == length - 1 && j_val == line.chars().count() - 1 =>
                        {
                            total += 1;
                        }
                        // TERRIBLE
                        // Top edge cells
                        (0, _) => {
                            let left = line.chars().nth(j - 1_usize).unwrap();
                            let right = line.chars().nth(j + 1_usize).unwrap();
                            let bottom = next_line.unwrap().chars().nth(j).unwrap();
                            let bottom_left = next_line.unwrap().chars().nth(j - 1_usize).unwrap();
                            let bottom_right = next_line.unwrap().chars().nth(j + 1_usize).unwrap();
                            let sum = [left, right, bottom, bottom_left, bottom_right]
                                .iter()
                                .fold(0, |acc, item| if *item == '@' { acc + 1 } else { acc });
                            if sum < 4 {
                                total += 1;
                            }
                        }
                        // Left edge cells
                        (_, 0) => {
                            let top = prev_line.unwrap().chars().nth(j).unwrap();
                            let top_right = prev_line.unwrap().chars().nth(j + 1_usize).unwrap();
                            let right = line.chars().nth(j + 1_usize).unwrap();
                            let bottom = next_line.unwrap().chars().nth(j).unwrap();
                            let bottom_right = next_line.unwrap().chars().nth(j + 1_usize).unwrap();
                            let sum = [top, top_right, right, bottom, bottom_right]
                                .iter()
                                .fold(0, |acc, item| if *item == '@' { acc + 1 } else { acc });

                            if sum < 4 {
                                total += 1;
                            }
                        }
                        // Right edge cells
                        (_, j_val) if j_val == line.chars().count() - 1 => {
                            let top = prev_line.unwrap().chars().nth(j).unwrap();
                            let top_left = prev_line.unwrap().chars().nth(j - 1_usize).unwrap();
                            let left = line.chars().nth(j - 1_usize).unwrap();
                            let bottom = next_line.unwrap().chars().nth(j).unwrap();
                            let bottom_left = next_line.unwrap().chars().nth(j - 1_usize).unwrap();

                            let sum = [top, top_left, left, bottom, bottom_left]
                                .iter()
                                .fold(0, |acc, item| if *item == '@' { acc + 1 } else { acc });

                            if sum < 4 {
                                total += 1;
                            }
                        }
                        // Bottom edge cells
                        (i_val, _) if i_val == length - 1 => {
                            let top = prev_line.unwrap().chars().nth(j).unwrap();
                            let top_left = prev_line.unwrap().chars().nth(j - 1_usize).unwrap();
                            let top_right = prev_line.unwrap().chars().nth(j + 1_usize).unwrap();
                            let left = line.chars().nth(j - 1_usize).unwrap();
                            let right = line.chars().nth(j + 1_usize).unwrap();

                            let sum = [left, right, top, top_left, top_right]
                                .iter()
                                .fold(0, |acc, item| if *item == '@' { acc + 1 } else { acc });

                            if sum < 4 {
                                total += 1;
                            }
                        }
                        // All the other cells
                        _ => {
                            let top = prev_line.unwrap().chars().nth(j).unwrap();
                            let top_left = prev_line.unwrap().chars().nth(j - 1).unwrap();
                            let top_right = prev_line.unwrap().chars().nth(j + 1).unwrap();
                            let left = line.chars().nth(j - 1).unwrap();
                            let right = line.chars().nth(j + 1).unwrap();
                            let bottom = next_line.unwrap().chars().nth(j).unwrap();
                            let bottom_left = next_line.unwrap().chars().nth(j - 1).unwrap();
                            let bottom_right = next_line.unwrap().chars().nth(j + 1).unwrap();
                            let sum = [
                                top,
                                top_left,
                                top_right,
                                left,
                                right,
                                bottom,
                                bottom_left,
                                bottom_right,
                            ]
                            .iter()
                            .fold(0, |acc, item| if *item == '@' { acc + 1 } else { acc });
                            if sum < 4 {
                                total += 1;
                            }
                        }
                    }
                    result += total;
                }
            }
        }
    }

    result
}

fn main() {
    println!("{}", part1("./src/bin/day04/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/bin/day04/test.txt"), 13_u16);
    }
}
