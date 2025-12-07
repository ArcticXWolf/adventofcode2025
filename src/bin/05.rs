use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut ids = vec![];
    let (ranges_str, ids_str) = input.trim().split_once("\n\n").unwrap();

    for l in ranges_str.trim().lines() {
        let (start, end) = l.split_once("-").unwrap();
        ranges.push(start.parse().unwrap()..=end.parse().unwrap());
    }
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    for l in ids_str.trim().lines() {
        ids.push(l.parse().unwrap());
    }

    (ranges, ids)
}

fn count_fresh(ranges: Vec<RangeInclusive<u64>>, ids: Vec<u64>) -> u64 {
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count() as u64
}

fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged_ranges = vec![];
    let mut current_range = ranges.first().unwrap().clone();
    for r in ranges.iter().skip(1) {
        if current_range.end() < r.start() {
            // no overlap
            merged_ranges.push(current_range);
            current_range = r.clone();
            continue;
        }
        if r.end() <= current_range.end() {
            // completely contained
            continue;
        }
        current_range = *current_range.start()..=*r.end();
    }
    merged_ranges.push(current_range);

    merged_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ranges, ids) = parse_input(input);
    ranges = merge_ranges(ranges);
    Some(count_fresh(ranges, ids))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);
    ranges = merge_ranges(ranges);
    Some(ranges.iter().map(|r| r.end() - r.start() + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
