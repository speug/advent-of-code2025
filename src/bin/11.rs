use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut out = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(": ");
        let vertex = split.next().unwrap().to_string();
        let edges: Vec<String> = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        out.insert(vertex, edges);
    }
    out
}

fn dfs_count_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u64 {
    fn dfs_inner(
        graph: &HashMap<String, Vec<String>>,
        curr: &str,
        end: &str,
        counts: &mut HashMap<String, u64>,
    ) -> u64 {
        if curr == end {
            return 1;
        }
        if let Some(&count) = counts.get(curr) {
            return count;
        }
        let mut path_count = 0;
        if let Some(neighs) = graph.get(curr) {
            for v in neighs {
                path_count += dfs_inner(graph, v, end, counts);
            }
        }
        counts.insert(curr.to_string(), path_count);
        path_count
    }
    let mut initial: HashMap<String, u64> = HashMap::new();
    dfs_inner(graph, start, end, &mut initial)
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    Some(dfs_count_paths(&graph, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    // we can simply enumerate all paths svr -> dac -> fft -> out and
    // svr -> fft -> dac -> out; the result is the sum between the two.
    let svr_to_dac = dfs_count_paths(&graph, "svr", "dac");
    let svr_to_fft = dfs_count_paths(&graph, "svr", "fft");
    let dac_to_fft = dfs_count_paths(&graph, "dac", "fft");
    let fft_to_dac = dfs_count_paths(&graph, "fft", "dac");
    let fft_to_out = dfs_count_paths(&graph, "fft", "out");
    let dac_to_out = dfs_count_paths(&graph, "dac", "out");
    Some((svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
