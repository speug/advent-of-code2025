advent_of_code::solution!(7);
// use advent_of_code::prettyprint_grid;
use std::cmp::max;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push(row);
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut components = parse_input(input);
    let mut splits = 0;
    let mut to_activate = Vec::new();
    let n = components.len();
    let m = components[0].len();
    for i in 1..n {
        for (j, c) in components[i].iter().enumerate() {
            let incoming = (components[i - 1][j] == 'S') || (components[i - 1][j] == '|');
            if !incoming {
                continue;
            }
            if *c == '.' {
                to_activate.push(j);
            } else if *c == '^' {
                if j > 0 {
                    to_activate.push(j - 1);
                }
                if j < (m - 1) {
                    to_activate.push(j + 1);
                }
                splits += 1;
            }
        }
        for &active in to_activate.iter() {
            components[i][active] = '|';
        }
        to_activate.clear();
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut components = parse_input(input);
    let mut to_activate = Vec::new();
    let n = components.len();
    let m = components[0].len();
    let mut timelines: Vec<Vec<u64>> = vec![vec![0; m]; n];
    for i in 1..n {
        for (j, c) in components[i].iter().enumerate() {
            let incoming = (components[i - 1][j] == 'S') || (components[i - 1][j] == '|');
            if !incoming {
                continue;
            }
            if *c == '.' {
                to_activate.push(j);
                timelines[i][j] += max(1, timelines[i - 1][j]);
            } else if *c == '^' {
                if j > 0 {
                    to_activate.push(j - 1);
                    timelines[i][j - 1] += timelines[i - 1][j];
                }
                if j < (m - 1) {
                    to_activate.push(j + 1);
                    timelines[i][j + 1] += timelines[i - 1][j];
                }
            }
        }
        for &active in to_activate.iter() {
            components[i][active] = '|';
        }
        to_activate.clear();
    }
    Some(timelines.last().unwrap().iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
