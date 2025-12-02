advent_of_code::solution!(2);
use std::cmp::{max, min};
use std::collections::HashSet;
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

        let lo_half = lo_str[0..(lo_str.len() / 2)].to_string();
        let hi_half = hi_str[0..(hi_str.len() / 2)].to_string();
        let lo = lo_str.parse::<u64>().unwrap();
        let hi = hi_str.parse::<u64>().unwrap();
        if lo_half == hi_half {
            let repeated = lo_half.repeat(2).parse::<u64>().unwrap();
            if (lo <= repeated) && (repeated <= hi) {
                out += repeated;
            }
            continue;
        } else {
            let loh = lo_half.parse::<u64>().unwrap();
            let hih = hi_half.parse::<u64>().unwrap();
            let lo_repeat = lo_half.repeat(2).parse::<u64>().unwrap();
            let hi_repeat = hi_half.repeat(2).parse::<u64>().unwrap();
            let lo_lim = max(lo, lo_repeat);
            let hi_lim = min(hi, hi_repeat);
            for half in loh..hih + 1 {
                let repeated = half.to_string().repeat(2).parse::<u64>().unwrap();
                if (lo_lim <= repeated) && (repeated <= hi_lim) {
                    out += repeated;
                }
            }
        }
    }
    Some(out)
}

fn sum_candidates(lo_raw: String, hi_raw: String) -> u64 {
    let len_lo = lo_raw.len();
    let len_hi = hi_raw.len();
    // if lens differ, the problem splits into two; recursion?
    if len_lo == len_hi - 1 {
        let lo_1: String = iter::once('1')
            .chain(iter::repeat('0').take(len_hi - 1))
            .collect();
        let hi_2 = "9".repeat(len_lo);
        return sum_candidates(lo_raw, hi_2) + sum_candidates(lo_1, hi_raw);
    }
    let mut out = 0;
    let mut found = HashSet::new();
    let lo = lo_raw.parse::<u64>().unwrap();
    let hi = hi_raw.parse::<u64>().unwrap();
    for i in 1..(len_lo / 2) + 1 {
        if len_lo % (i) != 0 {
            continue;
        }
        let lo_lim = lo_raw[0..i].to_string().parse::<u64>().unwrap();
        let hi_lim = hi_raw[0..i].to_string().parse::<u64>().unwrap();
        if lo_lim == hi_lim {
            let repeated = lo_lim
                .to_string()
                .repeat(len_lo / i)
                .parse::<u64>()
                .unwrap();
            if (lo <= repeated) && (repeated <= hi) {
                if found.insert(repeated) {
                    out += repeated;
                }
            }
            continue;
        }
        for j in lo_lim..hi_lim + 1 {
            let repeated = j.to_string().repeat(len_lo / i).parse::<u64>().unwrap();

            if (lo <= repeated) && (repeated <= hi) {
                if found.insert(repeated) {
                    out += repeated;
                }
            }
        }
    }
    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut out = 0;
    for (lo_raw, hi_raw) in ranges.into_iter() {
        out += sum_candidates(lo_raw.to_string(), hi_raw.to_string());
    }
    Some(out)
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
        assert_eq!(result, Some(4174379265));
    }
}
