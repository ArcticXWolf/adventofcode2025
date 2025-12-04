use std::fmt::{Display, Write};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

advent_of_code::solution!(4);

#[derive(Debug)]
enum Tile {
    PaperRoll,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PaperRoll => f.write_char('@'),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Self::PaperRoll),
            _ => Err(()),
        }
    }
}

fn generate_map(input: &str) -> PointGrid<isize, 2, Tile> {
    let mut map = PointGrid::default();

    for (y, row) in input.trim().lines().enumerate() {
        for (x, c) in row.char_indices() {
            if let Ok(tile) = c.try_into() {
                map.insert(Point2::new(x as isize, y as isize), tile);
            }
        }
    }

    map
}

fn is_tile_accessible(map: &PointGrid<isize, 2, Tile>, position: &Point2<isize>) -> bool {
    let mut count = 0;
    for d in Point2Direction::all_with_diagonals() {
        if let Some(Tile::PaperRoll) = map.get(&(position.get_point_in_direction(d, 1))) {
            count += 1;
        }
    }
    count < 4
}

fn remove_accessible_tiles(map: &PointGrid<isize, 2, Tile>) -> (PointGrid<isize, 2, Tile>, usize) {
    let mut new_map = PointGrid::default();

    for p in map.0.keys() {
        if !is_tile_accessible(map, p) {
            new_map.insert(*p, Tile::PaperRoll);
        }
    }

    let removed_elements = map.0.len() - new_map.0.len();
    (new_map, removed_elements)
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = generate_map(input);
    let mut result = 0;
    for p in map.0.keys() {
        if is_tile_accessible(&map, p) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = generate_map(input);
    let mut result = 0;
    let mut current = 1;

    while current > 0 {
        (map, current) = remove_accessible_tiles(&map);
        result += current as u64;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
