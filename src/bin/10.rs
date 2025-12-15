use core::fmt;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

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
    for (i, row) in out.iter_mut().enumerate() {
        row[i] = 1;
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

fn swap_column(m: &mut Matrix2D, c1: usize, c2: usize) {
    // matrix has columns for rows
    m.swap(c1, c2);
}

fn update_column(m: &mut Matrix2D, col1: usize, col2: usize, u: i64, v: i64, c1: i64, c2: i64) {
    for i in 0..m[0].len() {
        let val_a = m[col1][i];
        let val_b = m[col2][i];

        m[col1][i] = (u * val_a) + (v * val_b);
        m[col2][i] = (c1 * val_a) + (c2 * val_b);
    }
}

fn multiply_column(m: &mut Matrix2D, col: usize, mult: i64) {
    m[col].iter_mut().for_each(|x| *x *= mult);
}

fn subtract_column(m: &mut Matrix2D, col1: usize, col2: usize, factor: i64) {
    for i in 0..m[0].len() {
        m[col1][i] -= m[col2][i] * factor;
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

fn hnf(a: &Matrix2D) -> (Matrix2D, Matrix2D) {
    let n = a.len();
    let m = a[0].len();
    let mut u = identity_matrix(m);
    let mut h = transpose2(a.to_vec());

    let mut current_col = 0;
    for i in 0..n {
        // 1. pivot selection; need a row with a non-zero elem right of curr_col
        let mut pivot_col = None;
        for (k, hrow) in h.iter().enumerate().skip(current_col) {
            if hrow[i] != 0 {
                pivot_col = Some(k);
                break;
            }
        }

        if pivot_col.is_none() {
            continue;
        }

        let k = pivot_col.unwrap();
        swap_column(&mut h, current_col, k);
        swap_column(&mut u, current_col, k);

        // 2. elimination
        for j in current_col + 1..m {
            if h[j][i] != 0 {
                let pivot = h[current_col][i];
                let target = h[j][i];

                let (g, x, y) = extended_gcd(pivot, target);
                let c1 = -target / g;
                let c2 = pivot / g;

                update_column(&mut h, current_col, j, x, y, c1, c2);
                update_column(&mut u, current_col, j, x, y, c1, c2);
            }
        }

        // 3. modulo reduction
        let mut pivot_val = h[current_col][i];
        if pivot_val < 0 {
            multiply_column(&mut h, current_col, -1);
            multiply_column(&mut u, current_col, -1);
            pivot_val = -pivot_val;
        }
        for c in 0..current_col {
            let factor = h[c][i].div_euclid(pivot_val);
            subtract_column(&mut h, c, current_col, factor);
            subtract_column(&mut u, c, current_col, factor);
        }
        current_col += 1;
    }
    (transpose2(h), transpose2(u))
}

fn actions_to_matrix(a: &[Vec<usize>], n: usize) -> Matrix2D {
    let m = a.len();
    let mut out = vec![vec![0; m]; n];
    for (i, actions) in a.iter().enumerate() {
        for &action in actions.iter() {
            out[action][i] = 1;
        }
    }
    out
}

fn matrix_vector_product(a: &Matrix2D, x: &[i64]) -> Vec<i64> {
    assert!(a[0].len() == x.len(), "Need to have equal length columns");
    a.iter()
        .map(|row| row.iter().zip(x.iter()).map(|(a, b)| a * b).sum())
        .collect()
}

fn solve_y_initial(h: &Matrix2D, j: &[u64]) -> Option<(Vec<i64>, Vec<usize>)> {
    let rows = h.len();
    let cols = h[0].len();
    let mut y = vec![None; cols];

    for r in 0..rows {
        let mut sum = 0;
        let mut pivot_col = None;
        let mut pivot_val = 0;

        for c in 0..cols {
            let val = h[r][c];
            if val == 0 {
                continue;
            }
            if let Some(known_y) = y[c] {
                sum += val * known_y;
            } else if pivot_col.is_none() {
                pivot_col = Some(c);
                pivot_val = val;
            } else {
                println!("Wasn't triangular!");
                return None;
            }
        }
        let target = j[r] as i64 - sum;
        match pivot_col {
            Some(c) => {
                if let Some(exiting_y) = y[c] {
                    if pivot_val * exiting_y != target {
                        println!(
                            "HNF conflict at row {}: {} * {} != {}",
                            r, exiting_y, pivot_val, target
                        );
                        return None;
                    }
                } else {
                    if target % pivot_val != 0 {
                        println!("No integer solution!");
                        return None;
                    }
                    y[c] = Some(target / pivot_val);
                }
            }
            None => {
                if target != 0 {
                    println!("No integer solution!");
                    return None;
                }
            }
        }
    }
    let mut solution = Vec::with_capacity(cols);
    let mut free_vars = Vec::new();

    for (i, val_opt) in y.into_iter().enumerate() {
        match val_opt {
            Some(val) => solution.push(val),
            None => {
                solution.push(0);
                free_vars.push(i);
            }
        }
    }
    Some((solution, free_vars))
}

fn gaussian_solver(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();

    for i in 0..n {
        // pick pivot
        let mut pivot_row = i;
        for j in i + 1..n {
            if a[i][j].abs() > a[pivot_row][i].abs() {
                pivot_row = j;
            }
        }
        a.swap(i, pivot_row);
        b.swap(i, pivot_row);

        if a[i][i].abs() < 1e-9 {
            return None;
        }

        // eliminate
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            b[j] -= factor * b[i];
            #[allow(clippy::needless_range_loop)] // no idea how to do this with iters...
            for k in i..n {
                a[j][k] -= factor * a[i][k];
            }
        }
    }

    // back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += x[j] * a[i][j];
        }
        x[i] = (b[i] - sum) / a[i][i];
    }
    Some(x)
}

fn find_bounds(x_base: &[i64], u_null: &Matrix2D) -> Vec<(i64, i64)> {
    let num_constraints = x_base.len();
    let num_vars = u_null[0].len();

    let xf: Vec<f64> = x_base.iter().map(|&x| x as f64).collect();
    let uf: Vec<Vec<f64>> = u_null
        .iter()
        .map(|row| row.iter().map(|&x| x as f64).collect())
        .collect();

    let mut max_bounds = vec![f64::NEG_INFINITY; num_vars];
    let mut min_bounds = vec![f64::INFINITY; num_vars];
    let mut found_bounds = false;

    let combos = (0..num_constraints).combinations(num_vars);
    for indices in combos {
        let mut a = Vec::with_capacity(num_vars);
        let mut b = Vec::with_capacity(num_vars);

        for idx in indices {
            a.push(uf[idx].clone());
            b.push(-xf[idx]);
        }
        if let Some(k_candidate) = gaussian_solver(a, b) {
            let mut valid = true;
            for r in 0..num_constraints {
                let mut val = xf[r];
                for v in 0..num_vars {
                    val += uf[r][v] * k_candidate[v];
                }
                if val < -0.001 {
                    valid = false;
                    break;
                }
            }
            if valid {
                found_bounds = true;
                for v in 0..num_vars {
                    if k_candidate[v] < min_bounds[v] {
                        min_bounds[v] = k_candidate[v]
                    };
                    if k_candidate[v] > max_bounds[v] {
                        max_bounds[v] = k_candidate[v]
                    };
                }
            }
        }
    }
    if !found_bounds {
        println!("Warning! Did not find bounds.");
        return vec![(-200, 200); num_vars];
    }

    let mut out = Vec::new();
    for v in 0..num_vars {
        let min_val = if min_bounds[v].is_infinite() {
            -1000
        } else {
            min_bounds[v] as i64 - 30
        };
        let max_val = if max_bounds[v].is_infinite() {
            1000
        } else {
            max_bounds[v] as i64 + 30
        };
        out.push((min_val, max_val));
    }
    out
}

fn recursive_search(
    depth: usize,
    current_k: &mut Vec<i64>,
    bounds: &Vec<(i64, i64)>,
    u: &Matrix2D,
    y_p: &Vec<i64>,
    free_vars: &Vec<usize>,
    best_sum: &mut u64,
) {
    if depth == bounds.len() {
        let mut final_y = y_p.clone();

        for (i, &var_idx) in free_vars.iter().enumerate() {
            final_y[var_idx] = current_k[i];
        }

        let x_candidate = matrix_vector_product(u, &final_y);
        if x_candidate.iter().all(|&x| x >= 0) {
            let s: u64 = x_candidate.iter().map(|&x| x as u64).sum();
            if s < *best_sum {
                *best_sum = s;
            }
        }
        return;
    }
    let (start, end) = bounds[depth];
    for val in start..=end {
        current_k[depth] = val;
        recursive_search(depth + 1, current_k, bounds, u, y_p, free_vars, best_sum);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, actions, joltages) = parse_input(input);
    let mut sum_of_bests = 0;
    for (accs, jolts) in std::iter::zip(actions, joltages) {
        let m = jolts.len();
        // solve Hermitian normal form for A
        let a = actions_to_matrix(&accs, m);
        let (h, u) = hnf(&a);
        if let Some((y_p, free_vars)) = solve_y_initial(&h, &jolts) {
            if free_vars.is_empty() {
                // solution was unique; no need to search more
                let x_unique = matrix_vector_product(&u, &y_p);
                sum_of_bests += x_unique.iter().sum::<i64>() as u64;
            } else {
                /* The involved bit.
                  1. Find particular solution y_p from H y_p = jolts, x_base = Uy_p
                  2. Extract null basis U_null from U
                  3. Solve U_null y >= -x_base to get vertices of the solution polygon
                     (with +- 30(!) to really consider all possibilities)
                  4. Iterate through all possibilities to find the best solution
                */
                let x_base = matrix_vector_product(&u, &y_p);
                let mut u_null = Vec::new();
                for &var_idx in &free_vars {
                    let basis_vector: Vec<i64> = u.iter().map(|row| row[var_idx]).collect();
                    u_null.push(basis_vector);
                }
                u_null = transpose2(u_null);
                let k_bounds = find_bounds(&x_base, &u_null);
                let mut current_k = vec![0; free_vars.len()];
                let mut best_sum = u64::MAX;
                recursive_search(
                    0,
                    &mut current_k,
                    &k_bounds,
                    &u,
                    &y_p,
                    &free_vars,
                    &mut best_sum,
                );
                sum_of_bests += best_sum;
            }
        } else {
            println!("Could not solve system!");
            println!("--------------");
            println!("H = ");
            for row in h.iter() {
                println!("{:?}", row);
            }
            println!("--------------");
            println!("U = ");
            for row in u.iter() {
                println!("{:?}", row);
            }
        }
    }
    Some(sum_of_bests)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(463));
    }
}
