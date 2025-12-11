use core::fmt;
use std::{
    collections::{HashSet, VecDeque},
    iter,
};

advent_of_code::solution!(10);

type Action = Vec<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn apply_action(&self, action: &Action) -> Self {
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

fn bfs(initial: &Lights, goal: &Lights, actions: &[Action]) -> u64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*initial, 0));

    while let Some((state, count)) = queue.pop_front() {
        for a in actions.iter() {
            let s_next = state.apply_action(a);
            if s_next == *goal {
                return count + 1;
            }
            if visited.insert(s_next.s) {
                queue.push_back((s_next, count + 1));
            }
        }
    }
    unreachable!("Could not reach goal!")
}

fn parse_input(input: &str) -> (Vec<Lights>, Vec<Vec<Action>>, Vec<Vec<u64>>) {
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

type Matrix2D = Vec<Vec<i64>>;

fn identity_matrix(n: usize) -> Matrix2D {
    let mut out = vec![vec![0; n]; n];
    for i in 0..n {
        out[i][i] = 1;
    }
    out
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        if a < 0 {
            return (-a, -1, 0);
        } else {
            return (a, 1, 0);
        };
    }
    let (g, u1, v1) = extended_gcd(b, a.rem_euclid(b));
    let u = v1;
    let v = u1 - a.div_euclid(b) * v1;
    (g, u, v)
}

fn swap_column(M: &mut Matrix2D, c1: usize, c2: usize) {
    // matrix has columns for rows
    M.swap(c1, c2);
}

fn update_column(M: &mut Matrix2D, col1: usize, col2: usize, u: i64, v: i64, c1: i64, c2: i64) {
    for i in 0..M[0].len() {
        let val_a = M[col1][i];
        let val_b = M[col2][i];

        M[col1][i] = (u * val_a) + (v * val_b);
        M[col2][i] = (c1 * val_a) + (c2 * val_b);
    }
}

fn multiply_column(M: &mut Matrix2D, col: usize, mult: i64) {
    M[col].iter_mut().for_each(|x| *x *= mult);
}

fn subtract_column(M: &mut Matrix2D, col1: usize, col2: usize, factor: i64) {
    for i in 0..M[0].len() {
        M[col1][i] -= M[col2][i] * factor;
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn hnf(A: &Matrix2D) -> (Matrix2D, Matrix2D) {
    let n = A.len();
    let m = A[0].len();
    let mut U = identity_matrix(m);
    let mut H = transpose2(A.to_vec());

    let mut current_col = 0;
    for i in 0..n {
        // 1. pivot selection; need a row with a non-zero elem right of curr_col
        let mut pivot_col = None;
        for k in current_col..m {
            if H[k][i] != 0 {
                pivot_col = Some(k);
                break;
            }
        }

        if pivot_col.is_none() {
            continue;
        }

        let k = pivot_col.unwrap();
        swap_column(&mut H, current_col, k);
        swap_column(&mut U, current_col, k);

        // 2. elimination
        for j in current_col + 1..m {
            if H[j][i] != 0 {
                let pivot = H[current_col][i];
                let target = H[j][i];

                let (g, u, v) = extended_gcd(pivot, target);
                let c1 = -target / g;
                let c2 = pivot / g;

                update_column(&mut H, current_col, j, u, v, c1, c2);
                update_column(&mut U, current_col, j, u, v, c1, c2);
            }
        }

        // 3. modulo reduction
        let mut pivot_val = H[current_col][i];
        if pivot_val < 0 {
            multiply_column(&mut H, current_col, -1);
            multiply_column(&mut U, current_col, -1);
            pivot_val = -pivot_val;
        }
        for c in 0..current_col {
            let factor = H[c][i].div_euclid(pivot_val);
            subtract_column(&mut H, c, current_col, factor);
            subtract_column(&mut U, c, current_col, factor);
        }
        current_col += 1;
    }
    (transpose2(H), transpose2(U))
}

fn actions_to_matrix(A: &Vec<Vec<usize>>, n: usize) -> Matrix2D {
    let m = A.len();
    let mut out = vec![vec![0; m]; n];
    for (i, actions) in A.iter().enumerate() {
        for &action in actions.iter() {
            out[action][i] = 1;
        }
    }
    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, actions, joltages) = parse_input(input);
    let ji = 0;
    let m = joltages[ji].len();
    let a = actions_to_matrix(&actions[ji], m);

    for row in a.iter() {
        println!("{:?}", row);
    }
    println!("--------------");
    let (h, u) = hnf(&a);
    for row in h.iter() {
        println!("{:?}", row);
    }
    println!("--------------");
    for row in u.iter() {
        println!("{:?}", row);
    }
    let mut y = vec![0; m];
    for i in 0..m {
        let mut hsum = 0;
        for j in 0..i {
            hsum += y[j] * h[i][j];
        }
        y[i] = (joltages[ji][i] as i64 - hsum) / h[i][i];
    }
    println!("Particular solution: {:?}", y);
    Some(0)
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
