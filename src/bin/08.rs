advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn squared_distance(&self, other: &Point) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as u64
    }
}

// DSU
struct DisjointNetwork {
    parents: Vec<usize>,
    subnetwork_sizes: Vec<u64>,
    subnetwork_count: usize,
}

impl DisjointNetwork {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            subnetwork_sizes: vec![1; n],
            subnetwork_count: n,
        }
    }

    fn find_subnetwork(&mut self, mut i: usize) -> usize {
        while i != self.parents[i] {
            self.parents[i] = self.parents[self.parents[i]];
            i = self.parents[i];
        }
        i
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find_subnetwork(i);
        let root_j = self.find_subnetwork(j);
        if root_i != root_j {
            if self.subnetwork_sizes[root_i] < self.subnetwork_sizes[root_j] {
                self.parents[root_i] = root_j;
                self.subnetwork_sizes[root_j] += self.subnetwork_sizes[root_i];
            } else {
                self.parents[root_j] = root_i;
                self.subnetwork_sizes[root_i] += self.subnetwork_sizes[root_j];
            }
            self.subnetwork_count -= 1;
            return true;
        }
        false
    }
}

type Connection = ((usize, usize), u64);

fn get_sorted_connections(points: &[Point]) -> Vec<Connection> {
    let n = points.len();
    let mut connections = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            connections.push(((i, j), points[i].squared_distance(&points[j])));
        }
    }
    connections.sort_unstable_by_key(|&(_, d)| d);
    connections
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut out = Vec::new();
    for line in input.lines() {
        let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        out.push(Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let connections = get_sorted_connections(&junctions);
    let mut network = DisjointNetwork::new(junctions.len());
    // Uncomment below for actual solution; test uses 10
    for &((a, b), _) in connections.iter().take(1000) {
        //for &((a, b), _) in connections.iter().take(10) {
        network.union(a, b);
    }
    let mut sizes: Vec<u64> = (0..junctions.len())
        .filter(|&i| network.parents[i] == i)
        .map(|i| network.subnetwork_sizes[i])
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(sizes[0] * sizes[1] * sizes[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let connections = get_sorted_connections(&junctions);
    let mut network = DisjointNetwork::new(junctions.len());
    for &((a, b), _) in connections.iter() {
        network.union(a, b);
        if network.subnetwork_count == 1 {
            return Some((junctions[a].x * junctions[b].x) as u64);
        }
    }
    unreachable!("Should eventually connect all!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
