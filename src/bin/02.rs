advent_of_code::solution!(2);

fn amount_digits(number: u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

fn is_part1_silly_pattern(number: u64) -> bool {
    if !amount_digits(number).is_multiple_of(2) {
        return false;
    }
    is_silly_pattern(number, amount_digits(number) / 2)
}

fn is_part2_silly_pattern(number: u64) -> bool {
    for i in 1..(amount_digits(number) / 2 + 1) {
        if is_silly_pattern(number, i) {
            return true;
        }
    }
    false
}

fn is_silly_pattern(number: u64, pattern_length: u32) -> bool {
    if !amount_digits(number).is_multiple_of(pattern_length) {
        return false;
    }

    let amount_repetitions = amount_digits(number) / pattern_length;
    let base = 10_u64.pow(pattern_length);

    let mut acc = number;
    let comparator = acc % base;
    acc /= base;

    for _ in 1..amount_repetitions {
        if acc % base != comparator {
            return false;
        }
        acc /= base;
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<_> = input
        .trim()
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .collect();
    let mut counter = 0;
    for r in ranges {
        for i in r {
            if is_part1_silly_pattern(i) {
                counter += i;
            }
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<_> = input
        .trim()
        .split(",")
        .map(|s| s.split_once("-").unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
        .collect();
    let mut counter = 0;
    for r in ranges {
        for i in r {
            if is_part2_silly_pattern(i) {
                counter += i;
            }
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_silly_pattern() {
        assert!(is_silly_pattern(99, 1));
        assert!(is_silly_pattern(1010, 2));
        assert!(is_silly_pattern(64646464, 2));
        assert!(is_silly_pattern(123123, 3));
        assert!(!is_silly_pattern(123123, 2));
        assert!(!is_silly_pattern(101, 2));
    }
}
