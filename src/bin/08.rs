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
    let n = junctions.len();
    let distance_matrix = distance_matrix(junctions);
    let mut flat_d = Vec::new();
    for (i, row) in distance_matrix.iter().enumerate().take(n) {
        for (j, d) in row.iter().enumerate().take(n).skip(i + 1) {
            flat_d.push(((i, j), *d));
        }
    }
    flat_d.sort_by(|a, b| a.1.cmp(&b.1));
    let n_c = num_connections.unwrap_or(flat_d.len() - 1);
    flat_d.iter().take(n_c).map(|x| x.0).collect()
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
        let new_size = network[&s_a].network_size + network[&s_b].network_size;
        network
            .entry(s_a)
            .and_modify(|station| station.network_size = new_size);
    }
}

fn get_network_size(a: usize, network: &mut HashMap<usize, JunctionBox>) -> u64 {
    let s_a = find_subnetwork(a, network);
    network[&s_a].network_size
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let connections = find_connections(&junctions, Some(1000));
    let mut network: HashMap<usize, JunctionBox> = junctions.into_iter().enumerate().collect();
    for (a, b) in connections.into_iter() {
        join_networks(a, b, &mut network);
    }
    // now need to only collect unique subnetworks
    let mut network_sizes = HashMap::new();
    for i in network.clone().keys() {
        network_sizes.insert(
            find_subnetwork(*i, &mut network),
            get_network_size(*i, &mut network),
        );
    }
    let mut sorted_sizes: Vec<u64> = network_sizes.values().cloned().collect();
    sorted_sizes.sort_by(|a, b| b.cmp(a));
    Some(sorted_sizes[0] * sorted_sizes[1] * sorted_sizes[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = parse_input(input);
    let n = junctions.len() as u64;
    let connections = find_connections(&junctions, None);
    let mut network: HashMap<usize, JunctionBox> = junctions.into_iter().enumerate().collect();
    for (a, b) in connections.into_iter() {
        join_networks(a, b, &mut network);
        if get_network_size(a, &mut network) == n {
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
