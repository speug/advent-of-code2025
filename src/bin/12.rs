use std::collections::{HashMap, HashSet};

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
        let h = captures[1].parse::<usize>().unwrap();
        let w = captures[2].parse::<usize>().unwrap();
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

pub fn part_one(input: &str) -> Option<u64> {
    let (shapegrids, gridsizes, requirements) = parse_input(input);
    for (id, cgrid) in shapegrids.iter() {
        println!("{}", id);
        for row in cgrid.iter() {
            println!("{:?}", row);
        }
    }
    for (gs, reqs) in gridsizes.iter().zip(requirements) {
        println!("{}x{}: {:?}", gs.0, gs.1, reqs);
    }
    let mut shape_variants = HashMap::new();
    for (id, cgrid) in shapegrids {
        let shapes = generate_shapes(cgrid, id);
        println!("{}", id);
        println!("{:?}", shapes);
        shape_variants.insert(id, shapes);
    }
    for (id, shapes) in shape_variants.iter() {
        let n_unique = shapes.len();
        let mut grid = if n_unique <= 4 {
            Region::new(3, n_unique * 6)
        } else {
            Region::new(7, n_unique * 3)
        };
        for (i, shape) in shapes.iter().enumerate() {
            let mut col = if i < 4 {
                6 * i as isize
            } else {
                6 * (i as isize - 4)
            };
            let row = if i < 4 { 0 } else { 4 };
            loop {
                if grid.place_shape(row, col, shape.clone()) {
                    break;
                } else {
                    col += 1;
                }
            }
        }
        println!("{}", id);
        grid.print();
    }
    Some(0)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
