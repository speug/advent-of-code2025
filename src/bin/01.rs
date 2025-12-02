advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<(char, i32)> {
    let mut out = Vec::new();
    for s in input.lines() {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let steps = chars.collect::<String>().parse::<i32>().unwrap();
        out.push((direction, steps));
    }
    out
}

fn _process_move(pos: i32, instruction: &(char, i32)) -> (i32, u64) {
    let (direction, steps) = instruction;
    let mut clicks = 0;
    let mut new_pos = pos;
    if *direction == 'L' {
        new_pos -= steps;
        while new_pos < 0 {
            // each full rotation produces a click
            clicks += 1;
            new_pos += 100;
        }
        // if we decrement starting from 0, we don't actually click for the first!
        if pos == 0 {
            clicks -= 1;
        }
        // finally, when decrementing, we can hit exactly zero without going over
        if new_pos == 0 {
            clicks += 1;
        }
    } else {
        new_pos += steps;
        while new_pos > 99 {
            // each full rotation produces a click
            clicks += 1;
            new_pos -= 100;
        }
    }
    (new_pos, clicks)
}

// "smart" way; about the same speed as the while loops
fn process_move_mod(pos: i32, instruction: &(char, i32)) -> (i32, u64) {
    let (direction, steps) = instruction;
    let multiplier = if *direction == 'L' { -1 } else { 1 };
    let new_raw = pos + multiplier * steps;
    let full_rotations = new_raw.div_euclid(100);
    let new_pos = new_raw.rem_euclid(100);
    let mut clicks = full_rotations.abs() as u64;
    if (new_pos == 0) && (multiplier == -1) {
        clicks += 1;
    }
    if pos == 0 {
        clicks = clicks.saturating_sub(1);
    }
    (new_pos, clicks)
}

pub fn part_one(input: &str) -> Option<u64> {
    let moves = parse_input(input);
    let mut pos = 50;
    let mut out = 0;
    for m in moves.iter() {
        (pos, _) = process_move_mod(pos, m);
        if pos == 0 {
            out += 1
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let moves = parse_input(input);
    let mut pos = 50;
    let mut out = 0;
    let mut clicks;
    for m in moves.iter() {
        (pos, clicks) = process_move_mod(pos, m);
        out += clicks;
    }
    Some(out)
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
        assert_eq!(result, Some(6));
    }
}
