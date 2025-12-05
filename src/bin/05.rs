advent_of_code::solution!(5);
use regex::Regex;
use std::cmp::{max, min};

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let needle = Regex::new(r"(\d+)-(\d+)").unwrap();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(r) = needle.captures(line) {
            let start = r[1].parse::<u64>().unwrap();
            let end = r[2].parse::<u64>().unwrap();
            ranges.push((start, end));
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    }
    (ranges, ids)
}
pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, mut ids) = parse_input(input);
    // sorting provides a small speed gain; taking the joining from part 2 is better
    // than using original, sorted ranges (by like 6 us ":D")
    ranges.sort();
    ids.sort();
    let mut to_join = ranges[0];
    let mut joined_ranges = Vec::new();
    for i in 1..ranges.len() {
        if let Some(new_range) = join_ranges(to_join, ranges[i]) {
            to_join = new_range;
        } else {
            joined_ranges.push(to_join);
            to_join = ranges[i];
        }
    }
    joined_ranges.push(to_join);
    let mut fresh = 0;
    let mut start_idx = 0;
    let n_ranges = joined_ranges.len();
    for id in ids.into_iter() {
        for i in start_idx..n_ranges {
            let (lo, hi) = joined_ranges[i];
            if (lo <= id) && (id <= hi) {
                fresh += 1;
                start_idx = i;
                break;
            }
        }
    }
    Some(fresh)
}

fn join_ranges(r1: (u64, u64), r2: (u64, u64)) -> Option<(u64, u64)> {
    // assume ranges are sorted (r1 <= r2)
    if r1.1 < r2.0 {
        return None;
    } else {
        let new_start = min(r1.0, r2.0);
        let new_end = max(r1.1, r2.1);
        return Some((new_start, new_end));
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);
    ranges.sort();
    let mut to_join = ranges[0];
    let mut joined_ranges = Vec::new();
    for i in 1..ranges.len() {
        if let Some(new_range) = join_ranges(to_join, ranges[i]) {
            to_join = new_range;
        } else {
            joined_ranges.push(to_join);
            to_join = ranges[i];
        }
    }
    joined_ranges.push(to_join);
    let mut fresh = 0;
    for (low, high) in joined_ranges.into_iter() {
        fresh += high - low + 1;
    }
    Some(fresh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
