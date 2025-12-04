advent_of_code::solution!(3);

fn generate_banks(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn find_biggest_digit_string(bank: &[u32], depth: usize) -> u64 {
    if depth == 0 {
        return 0;
    }

    let max_digit = *bank.iter().rev().skip(depth - 1).max().unwrap();
    let pos_of_max_digit = bank.iter().position(|&d| d == max_digit).unwrap();
    find_biggest_digit_string(&bank[(pos_of_max_digit + 1)..], depth - 1)
        + 10_u64.pow(depth as u32 - 1) * max_digit as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let banks = generate_banks(input);
    for b in banks.iter() {
        result += find_biggest_digit_string(b, 2);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let banks = generate_banks(input);
    for b in banks.iter() {
        result += find_biggest_digit_string(b, 12);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
