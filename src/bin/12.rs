use std::collections::{HashMap, HashSet};

use advent_of_code::get_neighboring_indices_2d;
use regex::Regex;

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Shape {
    id: i8,
    tiles: Vec<(isize, isize)>,
    area: u8,
}

struct Region {
    w: isize,
    h: isize,
    grid: Vec<Vec<i8>>,
    shapes: HashMap<(usize, usize), Shape>,
}

impl Region {
    fn new(h: usize, w: usize) -> Self {
        Self {
            w: w as isize,
            h: h as isize,
            grid: vec![vec![-1; w]; h],
            shapes: HashMap::new(),
        }
    }

    fn check_coord(&self, i: isize, j: isize) -> bool {
        (i >= 0)
            && (i < self.h)
            && (j >= 0)
            && (j < self.w)
            && (self.grid[i as usize][j as usize] == -1)
    }

    fn place_shape(&mut self, i: isize, j: isize, s: Shape) -> bool {
        let mut coords = vec![(i, j)];
        for (dx, dy) in s.tiles.iter().skip(1) {
            let (tile_i, tile_j) = (i + dx, j + dy);
            if self.check_coord(tile_i, tile_j) {
                coords.push((tile_i, tile_j));
            } else {
                return false;
            }
        }
        self.shapes.insert((i as usize, j as usize), s.clone());
        for (x, y) in coords.into_iter() {
            self.grid[x as usize][y as usize] = s.id;
        }
        true
    }

    fn remove_shape(&mut self, i: usize, j: usize) {
        let Some(shape) = self.shapes.remove(&(i, j)) else {
            panic!("Tried to remove non-existent shape!")
        };
        self.grid[i][j] = -1;
        for (ti, tj) in shape.tiles {
            self.grid[(i as isize + ti) as usize][(j as isize + tj) as usize] = -1;
        }
    }

    fn print(&self) {
        println!("Grid {}x{}, {} shapes", self.h, self.w, self.shapes.len());
        for row in self.grid.iter() {
            println!(
                "{}",
                row.iter()
                    .map(|&x| if x == -1 {
                        '.'
                    } else {
                        (b'0' + x as u8) as char
                    })
                    .collect::<String>()
            )
        }
    }

    fn usable_area(&self) -> u8 {
        fn inner(
            grid: &Vec<Vec<i8>>,
            h: &usize,
            w: &usize,
            area: &mut u8,
            visited: &mut HashSet<(usize, usize)>,
            i: usize,
            j: usize,
        ) {
            visited.insert((i, j));
            *area += 1;
            let neighs = get_neighboring_indices_2d(i, j, h, w, false);
            for &(ni, nj) in neighs.iter().filter(|(x, y)| grid[*x][*y] == -1) {
                if !visited.contains(&(ni, nj)) {
                    inner(grid, h, w, area, visited, ni, nj);
                }
            }
            return;
        }
        let mut area = 0;
        let mut visited = HashSet::new();
        inner(
            &self.grid,
            &(self.h as usize),
            &(self.w as usize),
            &mut area,
            &mut visited,
            (self.h - 1) as usize,
            (self.w - 1) as usize,
        );
        area
    }
}

type Chargrid = Vec<Vec<char>>;
fn parse_input(input: &str) -> (Vec<(i8, Chargrid)>, Vec<(usize, usize)>, Vec<Vec<u8>>) {
    let mut shapes = Vec::new();
    let mut gridsizes = Vec::new();
    let mut requirements = Vec::new();
    let mut blocks: Vec<&str> = input.split("\n\n").collect();
    let grids = blocks.split_off(blocks.len() - 1);
    let idre = Regex::new(r"(\d+):").unwrap();
    let gridre = Regex::new(r"^(\d+)x(\d+): (.+)$").unwrap();
    for block in blocks.into_iter() {
        let mut shape = Vec::new();
        let mut id = 0;
        for (i, line) in block.lines().enumerate() {
            if i == 0 {
                let Some(captures) = idre.captures(line) else {
                    panic!("No match!");
                };
                id = captures[1].parse::<i8>().unwrap();
            } else {
                shape.push(line.chars().collect::<Vec<char>>());
            }
        }
        shapes.push((id, shape));
    }
    for gridline in grids[0].lines() {
        let Some(captures) = gridre.captures(gridline) else {
            panic!("Invalid grid line! {gridline}");
        };
        let w = captures[1].parse::<usize>().unwrap();
        let h = captures[2].parse::<usize>().unwrap();
        gridsizes.push((h, w));
        let reqs = captures[3]
            .split_ascii_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        requirements.push(reqs);
    }
    (shapes, gridsizes, requirements)
}

fn generate_shape(chars: &Chargrid, id: i8) -> Shape {
    let mut start_coords = None;
    let mut tiles = Vec::new();
    for (i, row) in chars.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                if start_coords.is_none() {
                    start_coords = Some((i as isize, j as isize))
                }
                let (i0, j0) = start_coords.unwrap();
                tiles.push((i as isize - i0, j as isize - j0));
            }
        }
    }
    let area = tiles.len() as u8;
    Shape { id, tiles, area }
}

fn rotate(chars: &Chargrid) -> Chargrid {
    let n = chars.len();
    let mut rotchars = vec![vec!['.'; n]; n];
    for (i, row) in chars.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            rotchars[j][(n - 1) - i] = c;
        }
    }
    rotchars
}

fn generate_shapes(mut chars: Chargrid, id: i8) -> Vec<Shape> {
    let mut unique_shapes = HashSet::new();
    // base shape
    unique_shapes.insert(chars.clone());
    // rotations
    for _ in 0..4 {
        chars = rotate(&chars);
        unique_shapes.insert(chars.clone());
    }
    // flip
    for row in chars.iter_mut() {
        row.swap(0, 2);
    }
    for _ in 0..4 {
        chars = rotate(&chars);
        unique_shapes.insert(chars.clone());
    }
    let mut shapes = Vec::new();
    for cgrid in unique_shapes.into_iter() {
        shapes.push(generate_shape(&cgrid, id));
    }
    shapes
}

fn solve(
    gridsize: (usize, usize),
    requirements: &Vec<u8>,
    shapes: &HashMap<i8, Vec<Shape>>,
) -> bool {
    let mut reqs = requirements.clone();
    let mut grid = Region::new(gridsize.0, gridsize.1);

    fn next_hole(grid: &Region, curr_i: usize, curr_j: usize) -> Option<(isize, isize)> {
        // find next de Bruijn hole
        let (mut hi, mut hj) = (curr_i as usize, curr_j as usize);
        // base case at the start
        if (hi == 0) && (hj == 0) {
            return Some((hi as isize, hj as isize));
        }
        loop {
            if hj < (grid.w - 1).try_into().unwrap() {
                hj += 1;
            } else if hi < (grid.h - 1).try_into().unwrap() {
                hi += 1;
                hj = 0;
            } else {
                break;
            }
            if grid.grid[hi][hj] == -1 {
                return Some((hi.try_into().unwrap(), hj.try_into().unwrap()));
            }
        }
        None
    }

    fn remaining_area(requirements: &Vec<u8>, shapes: &HashMap<i8, Vec<Shape>>) -> u8 {
        // simple flood fill from bottom right corner (de Bruijn starts from top left so should work)
        let mut out = 0;
        for (i, &req) in requirements.iter().enumerate() {
            if req > 0 {
                out += shapes.get(&(i as i8)).unwrap()[0].area;
            }
        }
        out
    }

    fn inner(
        grid: &mut Region,
        reqs: &mut Vec<u8>,
        shapes: &HashMap<i8, Vec<Shape>>,
        curr_i: isize,
        curr_j: isize,
    ) -> bool {
        // check if required shapes are placed
        if reqs.iter().all(|&x| x == 0) {
            grid.print();
            return true;
        }
        // get next hole
        let Some((hi, hj)) = next_hole(grid, curr_i as usize, curr_j as usize) else {
            return false;
        };
        // get ids of the remaining shapes
        let remaining_shapes: Vec<i8> = (0..reqs.len())
            .filter(|&x| reqs[x] > 0)
            .map(|x| x as i8)
            .collect();
        for shape_id in remaining_shapes.iter() {
            if let Some(shapevec) = shapes.get(shape_id) {
                // shapevec now contains all rotations and flips of the polymino in question
                for shape in shapevec {
                    if grid.place_shape(hi, hj, shape.clone()) {
                        reqs[*shape_id as usize] -= 1;
                        // remaining area heuristic
                        if remaining_area(&reqs, &shapes) > grid.usable_area() {
                            grid.remove_shape(hi as usize, hj as usize);
                            reqs[*shape_id as usize] += 1;
                        // recursive call
                        } else if !inner(grid, reqs, shapes, hi, hj + 1) {
                            // could not solve, backtrack to next shape
                            grid.remove_shape(hi as usize, hj as usize);
                            reqs[*shape_id as usize] += 1;
                        } else {
                            // inner function returned true, so must be solvable!
                            return true;
                        }
                    }
                }
            }
        }
        // could not place anything to this hole, move on to the next
        inner(grid, reqs, shapes, hi, hj + 1)
    }

    inner(&mut grid, &mut reqs, shapes, 0, 0)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapegrids, gridsizes, requirements) = parse_input(input);
    let mut shape_variants = HashMap::new();
    for (id, cgrid) in shapegrids {
        let shapes = generate_shapes(cgrid, id);
        shape_variants.insert(id, shapes);
    }
    let mut solvable = 0;
    for (&gs, reqs) in gridsizes.iter().zip(requirements) {
        if solve(gs, &reqs, &shape_variants) {
            println!("{}x{}: {:?}", gs.0, gs.1, reqs);
            solvable += 1;
        }
    }
    Some(solvable)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = std::fs::read_to_string(
            "/home/lauri/Documents/misc/advent-of-code2025/data/examples/12.txt",
        )
        .unwrap();
        let result = part_one(example.as_str());
        assert_eq!(result, None);
    }

    //    #[test]
    //    fn test_part_two() {
    //        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //        assert_eq!(result, None);
    //    }
}
