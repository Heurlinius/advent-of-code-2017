extern crate regex;

use std::collections::{HashMap, HashSet};
use regex::Regex;

fn parse_pipes(input: &str) -> HashMap<usize, HashSet<usize>> {
    let regex = Regex::new(r"([0-9]+) <-> ([0-9]+(?:, [0-9]+)*)").unwrap();

    let mut result = HashMap::new();
    for line in input.lines() {
        let m = regex.captures(line).unwrap();
        let current = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let connected: Vec<usize> = m.get(2).unwrap().as_str()
            .split(", ")
            .map(|id| id.parse::<usize>().unwrap())
            .collect();

        for &id in connected.iter() {
            // Connection is bidirectional; add the current ID to each connected node too.
            result.entry(id)
                .or_insert(HashSet::new())
                .insert(current);
        }
        // Add the current ID with all its connections
        result.entry(current)
            .or_insert(HashSet::new())
            .extend(connected.into_iter());
    }

    result
}

fn count_group(start: usize, pipes: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut visited = HashSet::new();
    count_connected(start, pipes, &mut visited);
    visited.len()
}

fn count_connected(node: usize, pipes: &HashMap<usize, HashSet<usize>>, visited: &mut HashSet<usize>) {
    visited.insert(node);
    for &next in pipes[&node].iter() {
        if !visited.contains(&next) {
            count_connected(next, pipes, visited);
        }
    }
}

fn count_all_groups(pipes: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut remaining: HashSet<_> = pipes.keys().map(|k| *k).collect();
    let mut group_count = 0;
    loop {
        let next = match remaining.iter().nth(0) {
            Some(&n) => n,
            None => break,
        };
        group_count += 1;
        eliminate_connected(next, pipes, &mut remaining);
    }
    group_count
}

fn eliminate_connected(node: usize, pipes: &HashMap<usize, HashSet<usize>>, remaining: &mut HashSet<usize>) {
    remaining.remove(&node);
    for &next in pipes[&node].iter() {
        if remaining.contains(&next) {
            eliminate_connected(next, pipes, remaining);
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let pipes = parse_pipes(input);

    let part1 = count_group(0, &pipes);
    println!("Part 1 = {}", part1);

    let part2 = count_all_groups(&pipes);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_group() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let pipes = parse_pipes(input);
        let count = count_group(0, &pipes);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_count_all_groups() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let pipes = parse_pipes(input);
        let count = count_all_groups(&pipes);
        assert_eq!(count, 2);
    }
}
