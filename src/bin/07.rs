use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let startposition = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();
    let splitter_positions: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .filter(|l| l.chars().any(|c| c == '^'))
        .map(|l| l.chars().positions(|c| c == '^').collect())
        .collect();

    let mut split_counters = 0;
    let mut current_beams = HashSet::new();
    current_beams.insert(startposition);
    for line in &splitter_positions {
        let mut next_beams = HashSet::new();
        for splitter in line {
            if current_beams.contains(splitter) {
                next_beams.insert(splitter - 1);
                next_beams.insert(splitter + 1);
                split_counters += 1;
            }
        }
        for beam in current_beams {
            if !line.contains(&beam) {
                next_beams.insert(beam);
            }
        }
        current_beams = next_beams;
    }
    Some(split_counters)
}

pub fn part_two(input: &str) -> Option<u64> {
    let splitter_positions: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .filter(|l| l.chars().any(|c| c == '^'))
        .map(|l| l.chars().positions(|c| c == '^').collect())
        .rev()
        .collect();
    let mut splitter_paths: Vec<Vec<u64>> = vec![vec![]];

    for (layer_idx, layer) in splitter_positions.iter().enumerate() {
        if layer_idx == 0 {
            for _ in layer {
                splitter_paths[0].push(2);
            }
        } else {
            splitter_paths.push(vec![]);
        }

        for sp in layer {
            let (mut left, mut right) = (1, 1);
            for previous_layer in (0..layer_idx).rev() {
                if let Some(previous_idx) = splitter_positions
                    .get(previous_layer)
                    .unwrap()
                    .iter()
                    .position(|&s| s == sp - 1)
                {
                    left = *splitter_paths
                        .get(previous_layer)
                        .unwrap()
                        .get(previous_idx)
                        .unwrap();
                    break;
                }
            }
            for previous_layer in (0..layer_idx).rev() {
                if let Some(previous_idx) = splitter_positions
                    .get(previous_layer)
                    .unwrap()
                    .iter()
                    .position(|&s| s == sp + 1)
                {
                    right = *splitter_paths
                        .get(previous_layer)
                        .unwrap()
                        .get(previous_idx)
                        .unwrap();
                    break;
                }
            }
            splitter_paths[layer_idx].push(left + right);
        }
    }

    Some(splitter_paths.last().unwrap().iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
