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

fn form_shapes(points: &[Point]) -> Vec<Vec<Point>> {
    let mut out = Vec::new();
    let mut shape = Vec::new();
    for p in points {
        if shape.is_empty() {
            shape.push(*p);
        } else if shape[0] == *p {
            out.push(shape.clone());
            shape.clear();
        } else {
            shape.push(*p);
        }
    }
    if !shape.is_empty() {
        out.push(shape);
    }
    out
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
        if vertical_edge(pa, pb)
            && (p.x == pa.x)
            && (min(pa.y, pb.y) <= p.y)
            && (p.y <= max(pa.y, pb.y))
        {
            return true;
        } else if (p.y == pa.y) && (min(pa.x, pb.x) <= p.x) && (p.x <= max(pa.x, pb.x)) {
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
        if vertical_edge(pa, pb) {
            if (p.x > pa.x) && (min(pa.y, pb.y) < p.y) && (p.y <= max(pa.y, pb.y)) {
                crossings += 1;
            }
        }
    }
    crossings % 2 == 1
}

fn check_rectangle(p1: &Point, p2: &Point, shape: &[Point]) -> bool {
    // generate edge points
    let min_x = min(p1.x, p2.x);
    let x_diff = max(p1.x, p2.x) - min_x + 1;
    let min_y = min(p1.y, p2.y);
    let y_diff = max(p1.y, p2.y) - min_y + 1;
    // check horizontal 1
    for p in (min_x..min_x + x_diff)
        .zip(vec![p1.y; x_diff as usize])
        .map(|(x, y)| Point { x: x, y: y })
    {
        if !point_in_shape(&p, shape) {
            return false;
        }
    }
    // check horizontal 2
    for p in (min_x..min_x + x_diff)
        .zip(vec![p2.y; x_diff as usize])
        .map(|(x, y)| Point { x: x, y: y })
    {
        if !point_in_shape(&p, shape) {
            return false;
        }
    }
    // check vertical 1
    for p in std::iter::repeat(p1.x)
        .take(y_diff as usize)
        .into_iter()
        .zip(min_y..min_y + y_diff)
        .map(|(x, y)| Point { x: x, y: y })
    {
        if !point_in_shape(&p, shape) {
            return false;
        }
    }
    // check vertical 2
    for p in std::iter::repeat(p2.x)
        .take(y_diff as usize)
        .into_iter()
        .zip(min_y..min_y + y_diff)
        .map(|(x, y)| Point { x: x, y: y })
    {
        if !point_in_shape(&p, shape) {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let shapes = form_shapes(&points);
    let mut max_area = 0;
    /*
    outline for a better solution:
    1. generate all "intersection points" with a vertical and horisontal sweep.
       This can be done efficiently with creating ordered lists of edge x and y coords
       Store these by the line being swept (i.e. in two Vecs). do not include corners?
    2. generate all rectangles, sort by area
    3. for each rectangle, check each edge of the rectangle for intersection points;
       invalidate a rectangle if an edge contains an intersection point
    4. First valid rectangle wins.
    */

    // uncomment below for solution, the BF solution is for tests to pass
    // looking at the plot of the points, it is clear that only two points are real
    // candidates for one of the corners
    //    let cand1 = Point { x: 94693, y: 50233 };
    //    let cand2 = Point { x: 94693, y: 48547 };
    //    for p in shapes[0].iter().filter(|&p| p.y > cand1.y) {
    //        if check_rectangle(&cand1, p, &shapes[0]) {
    //            println!("Found valid rectangle: {:?} - {:?}", cand1, &p);
    //            let area = cand1.area_with(p);
    //            if area > max_area {
    //                max_area = area
    //            }
    //        }
    //    }
    //    for p in shapes[0].iter().filter(|&p| p.y < cand2.y) {
    //        if check_rectangle(&cand2, p, &shapes[0]) {
    //            println!("Found valid rectangle: {:?} - {:?}", cand2, &p);
    //            let area = cand2.area_with(p);
    //            if area > max_area {
    //                max_area = area
    //            }
    //        }
    //    }
    let n = shapes.len();
    for (i, s) in shapes.iter().enumerate() {
        println!("Checking shape {}/{}", i + 1, n);
        for (i, &p1) in s.iter().enumerate() {
            for &p2 in s.iter().skip(i + 1) {
                if check_rectangle(&p1, &p2, s) {
                    println!("Found valid rectangle: {:?} - {:?}", p1, p2);
                    let area = p1.area_with(&p2);
                    if area > max_area {
                        max_area = area
                    }
                }
            }
        }
    }
    Some(max_area)
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
