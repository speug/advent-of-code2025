use itertools::Itertools;
use std::cmp::{max, min};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area_with(&self, other: &Point) -> u64 {
        (((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)) as u64
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut out = Vec::new();
    for line in input.lines() {
        let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        out.push(Point {
            x: coords[0],
            y: coords[1],
        });
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut max_area = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let area = p1.area_with(p2);
            if area > max_area {
                max_area = area
            }
        }
    }
    Some(max_area)
}

fn vertical_edge(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x
}

fn point_in_shape(p: &Point, shape: &[Point]) -> bool {
    // check if point is on a border
    let n = shape.len();
    for (i, pa) in shape.iter().enumerate() {
        if p == pa {
            return true;
        }
        let pb = if i < (n - 1) {
            &shape[i + 1]
        } else {
            &shape[0]
        };
        let vertical_contains =
            (p.x == pa.x) && (min(pa.y, pb.y) <= p.y) && (p.y <= max(pa.y, pb.y));
        let is_vertical = vertical_edge(pa, pb);
        let horizontal_contains =
            (p.y == pa.y) && (min(pa.x, pb.x) <= p.x) && (p.x <= max(pa.x, pb.x));
        if (is_vertical && vertical_contains) || (!is_vertical && horizontal_contains) {
            return true;
        }
    }
    // point was not on border; either entirely inside or outside
    // resolve by casting a ray through vertical edges
    let mut crossings = 0;
    for (i, pa) in shape.iter().enumerate() {
        let pb = if i < (n - 1) {
            &shape[i + 1]
        } else {
            &shape[0]
        };
        if vertical_edge(pa, pb)
            && (p.x > pa.x)
            && (min(pa.y, pb.y) < p.y)
            && (p.y <= max(pa.y, pb.y))
        {
            crossings += 1;
        }
    }
    crossings % 2 == 1
}

type Edge = (Point, Point);
fn generate_ranges(points: &[Point]) -> (Vec<Edge>, Vec<Edge>) {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    for (&p1, &p2) in points.iter().circular_tuple_windows() {
        if p1.x == p2.x {
            // vertical
            let (low, high) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };
            vertical.push((low, high));
        } else {
            let (low, high) = if p1.x < p2.x { (p1, p2) } else { (p2, p1) };
            horizontal.push((low, high));
        }
    }
    (horizontal, vertical)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    /*
    outline for a solution:
    1. generate all "intersection points" with a vertical and horisontal sweep.
       This can be done efficiently with creating ordered lists of edge x and y coords
       Store these by the line being swept (i.e. in two Vecs). do not include corners?
    2. generate all rectangles, sort by area
    3. for each rectangle, check each edge of the rectangle for intersection points;
       invalidate a rectangle if an edge contains an intersection point OR if a corner
       is out
    4. First valid rectangle wins.
    */
    let (horizontal, vertical) = generate_ranges(&points);
    let hmin = horizontal.iter().map(|&p| p.0.x).min().unwrap();
    let hmax = horizontal.iter().map(|&p| p.1.x).max().unwrap();
    let vmin = horizontal.iter().map(|&p| p.0.y).min().unwrap();
    let vmax = horizontal.iter().map(|&p| p.1.y).max().unwrap();
    let mut vert_crossings: Vec<Vec<i64>> = vec![Vec::new(); (hmax - hmin) as usize + 1];
    let mut hor_crossings: Vec<Vec<i64>> = vec![Vec::new(); (vmax - vmin) as usize + 1];
    for &(low, high) in horizontal.iter() {
        for x in (low.x + 1)..high.x {
            vert_crossings[(x - hmin) as usize].push(low.y);
        }
    }
    for &(low, high) in vertical.iter() {
        for y in (low.y + 1)..high.y {
            hor_crossings[(y - vmin) as usize].push(low.x);
        }
    }
    let mut rectangles = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let area = p1.area_with(p2);
            rectangles.push((area, (*p1, *p2)));
        }
    }
    rectangles.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    'outer: for &(a, (p1, p2)) in rectangles.iter() {
        let (ymin, ymax) = if p1.y < p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };
        let (xmin, xmax) = if p1.x < p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        // check that each of the other corners are inside (p1 and p2 always guaranteed
        // to be inside)
        if !point_in_shape(&Point { x: p1.x, y: p2.y }, &points) {
            continue;
        };
        if !point_in_shape(&Point { x: p2.x, y: p1.y }, &points) {
            continue;
        };
        // then check each edge for crossings
        for &y in vert_crossings[(p1.x - hmin) as usize].iter() {
            if (ymin < y) && (y < ymax) {
                continue 'outer;
            }
        }
        for &y in vert_crossings[(p2.x - hmin) as usize].iter() {
            if (ymin < y) && (y < ymax) {
                continue 'outer;
            }
        }
        for &x in hor_crossings[(p1.y - vmin) as usize].iter() {
            if (xmin < x) && (x < xmax) {
                continue 'outer;
            }
        }
        for &x in hor_crossings[(p2.y - vmin) as usize].iter() {
            if (xmin < x) && (x < xmax) {
                continue 'outer;
            }
        }
        return Some(a);
    }
    unreachable!("Should find rectangle!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
