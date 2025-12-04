advent_of_code::solution!(4);
use advent_of_code::get_neighboring_indices_2d;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn get_accessible_rolls(
    grid: &Vec<Vec<char>>,
    &height: &usize,
    &width: &usize,
) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] != '@' {
                continue;
            }
            let mut counter = 0;
            let neighs = get_neighboring_indices_2d(i, j, &height, &width, true);
            for (ii, jj) in neighs.into_iter() {
                if grid[ii][jj] == '@' {
                    counter += 1;
                }
            }
            if counter < 4 {
                accessible.push((i, j));
            }
        }
    }
    accessible
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let n_accessible = get_accessible_rolls(&grid, &h, &w).len() as u64;
    Some(n_accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    // do the same except in a while loop.
    let mut grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut removed = 0;
    loop {
        let removable = get_accessible_rolls(&grid, &h, &w);
        if removable.is_empty() {
            break;
        }
        removed += removable.len() as u64;
        for (i, j) in removable.into_iter() {
            grid[i][j] = '.';
        }
    }
    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
