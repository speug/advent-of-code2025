use std::{collections::HashMap, iter};

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Shape {
    id: u8,
    tiles: Vec<(isize, isize)>,
}

struct Region {
    w: isize,
    h: isize,
    grid: Vec<Vec<u8>>,
    shapes: HashMap<(usize, usize), Shape>,
}

impl Region {
    fn new(h: usize, w: usize) -> Self {
        Self {
            w: w as isize,
            h: h as isize,
            grid: vec![vec![0; w]; h],
            shapes: HashMap::new(),
        }
    }

    fn check_coord(&self, i: isize, j: isize) -> bool {
        (i >= 0)
            && (i < self.h)
            && (j >= 0)
            && (j < self.w)
            && (self.grid[i as usize][j as usize] == 0)
    }

    fn place_shape(&mut self, i: isize, j: isize, s: Shape) -> bool {
        let mut coords = vec![(i, j)];
        for (dx, dy) in s.tiles.iter() {
            coords.push((i + dx, j + dy));
        }
        if !coords.iter().all(|&(x, y)| self.check_coord(x, y)) {
            return false;
        }
        self.shapes.insert((i as usize, j as usize), s.clone());
        for (x, y) in coords.into_iter() {
            self.grid[x as usize][y as usize] = s.id;
        }
        true
    }

    fn print(&self) {
        println!("Grid {}x{}, {} shapes", self.h, self.w, self.shapes.len());
        for row in self.grid.iter() {
            println!(
                "{}",
                row.iter()
                    .map(|&x| if x == 0 { '.' } else { (b'0' + x) as char })
                    .collect::<String>()
            )
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    None
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
