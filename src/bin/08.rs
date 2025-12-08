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
        + (j1.coordinates.2 - j2.coordinates.2)) as u64
}

fn distance_matrix(junctions: &Vec<JunctionBox>) -> Vec<Vec<u64>> {
    let n = junctions.len();
    let mut D = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            D[i][j] = squared_distance(&junctions[i], &junctions[j]);
        }
    }
    D
}

fn find_connections(junctions: Vec<JunctionBox>, num_connections: usize) -> Vec<(usize, usize)> {
    let D = distance_matrix(&junctions);
    let mut flat_D: Vec<u64> = D.clone().into_iter().flatten().filter(|&x| x > 0).collect();
    flat_D.sort();
    let max_distance = flat_D[num_connections];
    let mut out = Vec::new();
    let n = junctions.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if D[i][j] < max_distance {
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut junctions = parse_input(input);
    let connections = find_connections(junctions, 10);
    println!("{:?}", connections);
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
