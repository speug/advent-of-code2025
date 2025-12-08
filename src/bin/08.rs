use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Eq, Clone)]
struct JunctionBox {
    id: usize,
    parent: usize,
    network_size: u64,
    coordinates: (i64, i64, i64),
}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn squared_distance(j1: &JunctionBox, j2: &JunctionBox) -> u64 {
    ((j1.coordinates.0 - j2.coordinates.0).pow(2)
        + (j1.coordinates.1 - j2.coordinates.1).pow(2)
        + (j1.coordinates.2 - j2.coordinates.2).pow(2)) as u64
}

fn distance_matrix(junctions: &[JunctionBox]) -> Vec<Vec<u64>> {
    let n = junctions.len();
    let mut distance_matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            distance_matrix[i][j] = squared_distance(&junctions[i], &junctions[j]);
        }
    }
    distance_matrix
}

fn find_connections(
    junctions: &[JunctionBox],
    num_connections: Option<usize>,
) -> Vec<(usize, usize)> {
    // TODO: return the connections in an ordered format; then 2 should be simple
    let distance_matrix = distance_matrix(junctions);
    let mut flat_d: Vec<u64> = distance_matrix
        .clone()
        .into_iter()
        .flatten()
        .filter(|&x| x > 0)
        .collect();
    flat_d.sort();
    let n_c = num_connections.unwrap_or(flat_d.len() - 1);
    let max_distance = flat_d[n_c];
    let mut out = Vec::new();
    let n = junctions.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if distance_matrix[i][j] < max_distance {
                out.push((i, j));
            }
        }
    }
    out
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    let mut out = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let coords: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        out.push(JunctionBox {
            id: i,
            parent: i,
            network_size: 1,
            coordinates: (coords[0], coords[1], coords[2]),
        });
    }
    out
}

fn find_subnetwork(s: usize, network: &mut HashMap<usize, JunctionBox>) -> usize {
    if network[&s].id == network[&s].parent {
        return s;
    }
    let new_parent = find_subnetwork(network[&s].parent, network);
    network
        .entry(s)
        .and_modify(|station| station.parent = new_parent);
    new_parent
}

fn join_networks(a: usize, b: usize, network: &mut HashMap<usize, JunctionBox>) {
    let mut s_a = find_subnetwork(a, network);
    let mut s_b = find_subnetwork(b, network);
    if s_a != s_b {
        if network[&s_a].network_size < network[&s_b].network_size {
            (s_b, s_a) = (s_a, s_b);
        }
        network
            .entry(s_b)
            .and_modify(|station| station.parent = s_a);
        let b_size = network[&s_b].network_size;
        network
            .entry(s_a)
            .and_modify(|station| station.network_size += b_size);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let connections = find_connections(&junctions, Some(10));
    let mut network: HashMap<usize, JunctionBox> = junctions.into_iter().enumerate().collect();
    for (a, b) in connections.into_iter() {
        join_networks(a, b, &mut network);
    }
    let mut network_sizes: Vec<u64> = network.values().map(|x| x.network_size).collect();
    network_sizes.sort_by(|a, b| b.cmp(a));
    Some(network_sizes[0] * network_sizes[1] * network_sizes[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let n = junctions.len() as u64;
    let connections = find_connections(&junctions, None);
    let mut network: HashMap<usize, JunctionBox> = junctions.into_iter().enumerate().collect();
    for (a, b) in connections.into_iter() {
        println!(
            "Connecting {:?} - {:?}",
            network[&a].coordinates, network[&b].coordinates
        );
        join_networks(a, b, &mut network);
        if (network[&a].network_size == n) || (network[&b].network_size == n) {
            return Some((network[&a].coordinates.0 * network[&b].coordinates.0) as u64);
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
