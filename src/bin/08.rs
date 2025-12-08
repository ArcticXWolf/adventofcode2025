#![feature(iter_map_windows)]
use std::{collections::HashSet, hash::Hash};

use advent_of_code::algebra_helpers::Point3;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Edge {
    a: Point3<isize>,
    b: Point3<isize>,
}

impl Edge {
    fn distance(&self) -> u64 {
        (self.a - self.b).length_euclid_squared() as u64
    }

    fn target(&self, source: Point3<isize>) -> Option<Point3<isize>> {
        if source == self.a {
            Some(self.b)
        } else if source == self.b {
            Some(self.a)
        } else {
            None
        }
    }
}

impl Eq for Edge {}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.a.min_componentwise(self.b).hash(state);
        self.a.max_componentwise(self.b).hash(state);
    }
}

fn generate_map(input: &str) -> Vec<Point3<isize>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut parts = l.splitn(3, ',');
            Point3::new(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn generate_edges(map: &[Point3<isize>]) -> Vec<Edge> {
    let mut edges: HashSet<Edge> = HashSet::new();

    for &a in map {
        for &b in map {
            if a == b {
                continue;
            }
            edges.insert(Edge { a, b });
        }
    }
    edges
        .into_iter()
        .sorted_by_key(|e| e.distance())
        .collect::<Vec<Edge>>()
}

fn find_group_sizes(map: &[Point3<isize>], edges: &[Edge]) -> Vec<u64> {
    let mut visited_nodes = HashSet::new();
    let mut group_sizes = vec![];

    for starting_point in map {
        if visited_nodes.contains(starting_point) {
            continue;
        }

        let mut current_group_size = 0;
        let mut stack = vec![*starting_point];
        while let Some(current_node) = stack.pop() {
            if visited_nodes.contains(&current_node) {
                continue;
            }

            visited_nodes.insert(current_node);
            current_group_size += 1;

            for target in edges.iter().filter_map(|e| e.target(current_node)) {
                if visited_nodes.contains(&target) {
                    continue;
                }
                stack.push(target);
            }
        }
        group_sizes.push(current_group_size);
    }
    group_sizes
}

fn binary_search(input: &str) -> Edge {
    let map = generate_map(input);
    let edges = generate_edges(&map);
    let (mut min, mut max) = (0, edges.len() - 1);
    while min <= max {
        let cur = (max - min) / 2 + min;
        let group_sizes = find_group_sizes(
            &map,
            &edges.iter().take(cur).cloned().collect::<Vec<Edge>>(),
        );
        if group_sizes.len() == 1 {
            max = cur - 1;
        } else {
            min = cur + 1;
        }
    }
    println!(
        "Edges: {} {:?} {} {:?}",
        min,
        edges.get(min).unwrap(),
        max,
        edges.get(max).unwrap()
    );

    *edges.get(max).unwrap()
}

fn _part_one(input: &str, size: usize) -> Option<u64> {
    let map = generate_map(input);
    let edges = generate_edges(&map);
    let subset_edges = edges.into_iter().take(size).collect::<Vec<Edge>>();
    let mut group_sizes = find_group_sizes(&map, &subset_edges);
    group_sizes.sort();
    group_sizes.reverse();

    Some(group_sizes.first().unwrap() * group_sizes.get(1).unwrap() * group_sizes.get(2).unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    _part_one(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let edge = binary_search(input);

    Some(edge.a.0[0] as u64 * edge.b.0[0] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
