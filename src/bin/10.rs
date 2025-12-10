use core::fmt;
use std::{
    cmp::min,
    collections::{HashSet, VecDeque},
    num::ParseIntError,
};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Lights {
    n: usize,
    s: u16,
}

impl fmt::Display for Lights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = self.s;
        let binary = format!("{:#0width$b}", num, width = self.n);
        let formatted: String = binary
            .chars()
            .rev()
            .filter(|&c| c == '0' || c == '1')
            .map(|c| if c == '0' { '.' } else { '#' })
            .collect();
        write!(f, "[{}]", formatted)
    }
}

#[derive(Debug)]
pub struct ParseLightsError;

impl std::str::FromStr for Lights {
    type Err = ParseLightsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err(ParseLightsError);
        }
        let inner = &s[1..s.len() - 1];
        let n = inner.len();

        let mut state = 0;
        for c in inner.chars().rev() {
            state <<= 1;
            match c {
                '#' => state |= 1,
                '.' => {}
                _ => return Err(ParseLightsError),
            }
        }
        Ok(Lights { n, s: state })
    }
}

impl Lights {
    fn apply_action(&self, action: &Vec<usize>) -> Self {
        let mut state = self.s;
        for l in action.iter() {
            state ^= 1 << l;
        }
        Lights {
            n: self.n,
            s: state,
        }
    }
}

fn bfs(initial: &Lights, goal: &Lights, actions: &Vec<Vec<usize>>) -> u64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*initial, 0));

    while let Some((state, count)) = queue.pop_front() {
        for a in actions.iter() {
            let s_next = state.apply_action(&a);
            if s_next == *goal {
                return count + 1;
            }
            if visited.insert(s_next) {
                queue.push_back((s_next, count + 1));
            }
        }
    }
    unreachable!("Could not reach goal!")
}

fn parse_input(input: &str) -> (Vec<Lights>, Vec<Vec<Vec<usize>>>, Vec<Vec<u64>>) {
    let mut goals = Vec::new();
    let mut actions = Vec::new();
    let mut joltages = Vec::new();
    for line in input.lines() {
        let mut split: VecDeque<&str> = line.split_ascii_whitespace().collect();
        let goal: Lights = split.pop_front().unwrap().parse().unwrap();
        goals.push(goal);
        let jolt_str = split.pop_back().unwrap();
        let jolt: Vec<u64> = jolt_str[1..jolt_str.len() - 1]
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        joltages.push(jolt);
        let mut action = Vec::new();
        while let Some(a) = split.pop_front() {
            let acc: Vec<usize> = a[1..a.len() - 1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            action.push(acc);
        }
        actions.push(action);
    }
    (goals, actions, joltages)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (goals, actions, _) = parse_input(input);
    let mut minims = 0;
    for (g, accs) in std::iter::zip(goals, actions) {
        let initial = Lights { n: g.n, s: 0 };
        minims += bfs(&initial, &g, &accs);
    }
    Some(minims)
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
