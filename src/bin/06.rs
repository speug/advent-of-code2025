use std::iter::zip;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        let operatorline = line
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<u64>()
            .is_err();
        for (j, val) in line.split_ascii_whitespace().enumerate() {
            if i == 0 {
                numbers.push(vec![val.parse::<u64>().unwrap()]);
            } else if !operatorline {
                numbers[j].push(val.parse::<u64>().unwrap());
            } else if val == "*" {
                operators.push(Operator::Multiply);
            } else if val == "+" {
                operators.push(Operator::Add);
            }
        }
    }
    (numbers, operators)
}

fn parse_column_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let mut operators = Vec::new();
    // get line length
    // iterate over each "column"
    // also need to find which line is the operatorline, and parse that separately
    let mut lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    // parse operators
    let operatorline = lines.split_off(lines.len() - 1);
    for op in operatorline[0].split_ascii_whitespace() {
        if op == "*" {
            operators.push(Operator::Multiply);
        } else if op == "+" {
            operators.push(Operator::Add);
        }
    }
    // parse numbers
    let linechars: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let chars = linechars[0].len();
    let rows = linechars.len();
    let mut numbers = Vec::new();
    let mut opnums = Vec::new();
    for c in 0..chars {
        let mut digits = Vec::new();
        for r in 0..rows {
            if let Some(digit) = linechars[r][c].to_digit(10) {
                digits.push(digit as u64);
            }
        }
        if digits.is_empty() {
            numbers.push(opnums);
            opnums = Vec::new();
        } else {
            opnums.push(digits.iter().fold(0, |acc, elem| acc * 10 + elem));
        }
    }
    if !opnums.is_empty() {
        numbers.push(opnums);
    }
    (numbers, operators)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers, operators) = parse_input(input);
    let mut out = 0;
    for (nums, operator) in zip(numbers, operators) {
        if operator == Operator::Add {
            out += nums.iter().sum::<u64>()
        } else {
            out += nums.iter().product::<u64>()
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers, operators) = parse_column_input(input);
    let mut out = 0;
    for (nums, operator) in zip(numbers, operators) {
        if operator == Operator::Add {
            out += nums.iter().sum::<u64>()
        } else {
            out += nums.iter().product::<u64>()
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
