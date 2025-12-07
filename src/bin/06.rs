use std::fmt::Display;

advent_of_code::solution!(6);

#[derive(Debug, Default)]
enum Operation {
    #[default]
    Unset,
    Addition,
    Multiplication,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "+"),
            Self::Multiplication => write!(f, "*"),
            Self::Unset => write!(f, " "),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Addition),
            "*" => Ok(Self::Multiplication),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.numbers.first().unwrap())?;
        for i in self.numbers.iter().skip(1) {
            write!(f, "{} {} ", self.operation, i)?;
        }
        Ok(())
    }
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Addition => self.numbers.iter().sum(),
            Operation::Multiplication => self.numbers.iter().product(),
            Operation::Unset => unimplemented!(),
        }
    }
}

fn parse_problems_part1(input: &str) -> Vec<Problem> {
    let mut problems = vec![];
    for l in input.trim().lines() {
        for (idx, s) in l.split_whitespace().enumerate() {
            if problems.len() <= idx {
                problems.push(Problem {
                    numbers: vec![],
                    operation: Operation::Unset,
                });
            }
            if let Ok(op) = s.try_into() {
                problems.get_mut(idx).unwrap().operation = op;
            } else {
                problems
                    .get_mut(idx)
                    .unwrap()
                    .numbers
                    .push(s.parse().unwrap());
            }
        }
    }
    problems
}

fn parse_problems_part2(input: &str) -> Vec<Problem> {
    let columns = input.lines().next().unwrap().len();
    let rows = input.trim().lines().count();

    let mut problems = vec![];
    let mut current_problem = Problem::default();

    for i in 0..columns {
        let current_column = columns - i - 1;
        let mut current_str = String::new();
        for l in input.lines().take(rows - 1) {
            current_str = format!("{}{}", current_str, l.chars().nth(current_column).unwrap());
        }
        if current_str.trim().is_empty() {
            continue;
        }

        current_problem
            .numbers
            .push(current_str.trim().parse().unwrap());

        match input
            .lines()
            .nth(rows - 1)
            .unwrap()
            .chars()
            .nth(current_column)
            .unwrap()
        {
            '*' => {
                current_problem.operation = Operation::Multiplication;
                problems.push(current_problem);
                current_problem = Problem::default();
            }
            '+' => {
                current_problem.operation = Operation::Addition;
                problems.push(current_problem);
                current_problem = Problem::default();
            }
            ' ' => {}
            _ => unimplemented!(),
        }
    }

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_problems_part1(input);
    Some(problems.iter().map(|p| p.solve()).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_problems_part2(input);
    Some(problems.iter().map(|p| p.solve()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
