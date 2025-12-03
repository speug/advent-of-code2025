advent_of_code::solution!(3);
use std::cmp::max;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u64)
            .collect();
        out.push(row);
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let mut out = 0;
    let n = banks[0].len();
    for bank in banks.iter() {
        // first, find the highest digit after each position
        let mut highest_after = vec![0; n];
        for i in (0..n - 1).rev() {
            highest_after[i] = max(highest_after[i + 1], bank[i + 1]);
        }
        // then we can just iterate through the list once, trying each bank value in
        // the tens place and use the precalculate highest_after list to get best
        // value for the ones place. You could probably stop early, but all of this
        // is O(n) anyway so good enough.
        let mut best = 0;
        for i in 0..(n - 1) {
            let cand = bank[i] * 10 + highest_after[i];
            if cand > best {
                best = cand;
            }
        }
        out += best;
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    let mut out = 0;
    let n = banks[0].len();
    for bank in banks.iter() {
        // initial best candidate is the last 12 digits
        let mut best = bank[(n - 12)..n].to_vec();
        for i in (0..n - 12).rev() {
            // "bumping"; for each new value, we accept if it is higher than current
            // highest power of ten. We then take the old value, and try to place it
            // in the next power of ten slot. Repeat taking the lower value and trying
            // to place it later in the sequence until we reach a higher value or the
            // the number ends. In a way, the higher digit always "bumps" the lower one
            // either to further in the number or out altogether, if there is already
            // a better candidate.
            let mut bumper = bank[i];
            let mut bi = 0; // bumping index
            while (bi < 12) && (bumper >= best[bi]) {
                std::mem::swap(&mut best[bi], &mut bumper);
                bi += 1;
            }
        }
        // now we have all the best numbers in the best places, so just build the final
        let best_val = best.iter().fold(0, |acc, elem| acc * 10 + elem);
        out += best_val;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
