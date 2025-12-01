advent_of_code::solution!(1);

#[derive(Debug)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (op, numstr) = value.split_at(1);
        let value = numstr.parse::<i32>().map_err(|_| ())?;

        match op {
            "L" => Ok(Self::Left(value)),
            "R" => Ok(Self::Right(value)),
            _ => Err(()),
        }
    }
}

impl Instruction {
    fn apply_and_return_visited_zeros(&self, current: i32, max: i32) -> (i32, i32) {
        match self {
            Self::Left(x) => (
                (current - x).rem_euclid(max),
                (x / max) // zeros due to full rotations
                    + (current != 0 && (current - x).rem_euclid(max) > current) as i32 // zeros due to overflowing
                    + ((current - x).rem_euclid(max) == 0) as i32, // zeros due to stopping on 0
            ),
            Self::Right(x) => (
                (current + x).rem_euclid(max),
                (x / max)  // zeros due to full rotations
                    + ((current + x).rem_euclid(max) < current) as i32, // zeros due to overflowing
                                                                        // no zeros due to stopping on 0, because they are already counted as overflows
            ),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect();
    let mut dial = 50;
    let mut counter = 0;

    for i in instructions {
        (dial, _) = i.apply_and_return_visited_zeros(dial, 100);
        if dial == 0 {
            counter += 1;
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect();
    let mut dial = 50;
    let mut counter = 0;

    for i in instructions {
        let overflows;
        (dial, overflows) = i.apply_and_return_visited_zeros(dial, 100);
        counter += overflows;
    }

    Some(counter as u64)
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
        assert_eq!(result, Some(6));
    }
}
