advent_of_code::solution!(2);
use std::iter;

use regex::Regex;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    let needle = Regex::new(r"(\d+)-(\d+)").unwrap();
    for (_, [lo, hi]) in needle.captures_iter(input).map(|c| c.extract()) {
        out.push((lo, hi));
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut out = 0;
    for (lo_raw, hi_raw) in ranges.into_iter() {
        let len_lo = lo_raw.len();
        let len_hi = hi_raw.len();
        let lo_str: String;
        let hi_str: String;
        if len_lo == len_hi {
            if len_lo % 2 == 1 {
                // both strings are odd and same length; cannot have repeats
                continue;
            }
            lo_str = lo_raw.to_string();
            hi_str = hi_raw.to_string();
        } else if len_lo % 2 == 1 {
            lo_str = iter::once('1')
                .chain(iter::repeat('0').take(len_hi - 1))
                .collect();
            hi_str = hi_raw.to_string();
        } else {
            lo_str = lo_raw.to_string();
            hi_str = "9".repeat(len_lo);
        }

        println!("Raw values {} ({})-{} ({})", lo_raw, len_lo, hi_raw, len_hi);
        let cand_a = lo_str[0..(lo_str.len() / 2)]
            .to_string()
            .repeat(2)
            .parse::<u64>()
            .unwrap();
        let cand_b = hi_str[0..(hi_str.len() / 2)]
            .to_string()
            .repeat(2)
            .parse::<u64>()
            .unwrap();
        let lo = lo_str.parse::<u64>().unwrap();
        let hi = hi_str.parse::<u64>().unwrap();
        println!("{}-{}", lo, hi);
        if (lo <= cand_a) && (cand_a <= hi) {
            println!("Found proper candidate {}", cand_a);
            out += cand_a;
        }
        if (lo <= cand_b) && (cand_b <= hi) && (cand_a != cand_b) {
            println!("Found proper candidate {}", cand_b);
            out += cand_b;
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
