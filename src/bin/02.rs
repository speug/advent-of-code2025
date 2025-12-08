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
            // low limit was odd; replace with lowest value with len = len_hi (1000...)
            lo_str = iter::once('1')
                .chain(iter::repeat_n('0', len_hi - 1))
                .collect();
            hi_str = hi_raw.to_string();
        } else {
            // high limit was odd; replace with highest value with len = len_lo (999...)
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
            // numeric versions of the halves for iteration
            let loh = lo_half.parse::<u64>().unwrap();
            let hih = hi_half.parse::<u64>().unwrap();
            let lo_repeat = lo_half.repeat(2).parse::<u64>().unwrap();
            let hi_repeat = hi_half.repeat(2).parse::<u64>().unwrap();
            // sometimes the number will land outside the limits due to the last digits
            // hence, pick the tighter bounds
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
    // if lens differ, the problem splits into two; can recursively solve each half
    // here we obviously use the fact that there is at most len diff of one (verified
    // on my input)
    if len_lo == len_hi - 1 {
        let lo_1: String = iter::once('1')
            .chain(iter::repeat_n('0', len_hi - 1))
            .collect();
        let hi_2 = "9".repeat(len_lo);
        return sum_candidates(lo_raw, hi_2) + sum_candidates(lo_1, hi_raw);
    }
    // set up collections and numeric values
    let mut out = 0;
    // could find doubles, so set up hashset to stop double counting
    let mut found = HashSet::new();
    let lo = lo_raw.parse::<u64>().unwrap();
    let hi = hi_raw.parse::<u64>().unwrap();
    for i in 1..(len_lo / 2) + 1 {
        // unless mod corresponds, cannot have exact repeats
        if len_lo % i != 0 {
            continue;
        }
        // possible candidate values; take first i
        let lo_lim = lo_raw[0..i].to_string().parse::<u64>().unwrap();
        let hi_lim = hi_raw[0..i].to_string().parse::<u64>().unwrap();
        // test each possible repeat candidate in the range
        for j in lo_lim..hi_lim + 1 {
            let repeated = j.to_string().repeat(len_lo / i).parse::<u64>().unwrap();

            if (lo <= repeated) && (repeated <= hi) && (found.insert(repeated)) {
                out += repeated;
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
