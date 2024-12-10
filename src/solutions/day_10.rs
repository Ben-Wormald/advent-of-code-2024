use std::{collections::HashSet, ops::Add};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord(isize, isize);
impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord(x as isize, y as isize)
    }

    fn x(&self) -> usize {
        self.0 as usize
    }

    fn y(&self) -> usize {
        self.1 as usize
    }
}
impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }    
}

pub fn solve_part_one(input: &str) -> usize {
    let map = get_map(input);
    let mut sum = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                sum += get_score(&map, Coord::new(x, y));
            }
        }
    }

    sum
}

pub fn solve_part_two(input: &str) -> usize {
    let map = get_map(input);
    let mut sum = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if *height == 0 {
                sum += get_distinct_score(&map, Coord::new(x, y));
            }
        }
    }

    sum
}

fn get_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|row| row
            .chars()
            .map(|height| height.to_string().parse().unwrap())
            .collect()
        )
        .collect()
}

fn get_score(map: &[Vec<u8>], trailhead: Coord) -> usize {
    let mut current_ends = HashSet::new();
    current_ends.insert(trailhead);

    for height in 1..=9 {
        let mut new_ends = HashSet::new();

        for end in current_ends.iter() {
            for adj in get_adjacent(map, end).into_iter() {
                if map[adj.y()][adj.x()] == height {
                    new_ends.insert(adj);
                }
            }
        }

        current_ends = new_ends;
    }

    current_ends.len()
}

fn get_distinct_score(map: &[Vec<u8>], trailhead: Coord) -> usize {
    let mut current_ends = vec![trailhead];

    for height in 1..=9 {
        let mut new_ends = Vec::new();

        for end in current_ends.iter() {
            for adj in get_adjacent(map, end).into_iter() {
                if map[adj.y()][adj.x()] == height {
                    new_ends.push(adj);
                }
            }
        }

        current_ends = new_ends;
    }

    current_ends.len()
}

fn get_adjacent(map: &[Vec<u8>], coord: &Coord) -> Vec<Coord> {
    let mut adjacent = Vec::new();

    if coord.x() > 0 {
        adjacent.push(coord + &Coord(-1, 0));
    }
    if coord.y() > 0 {
        adjacent.push(coord + &Coord(0, -1));
    }
    if (coord.x()) < map[coord.y()].len() - 1 {
        adjacent.push(coord + &Coord(1, 0));
    }
    if (coord.y()) < map.len() - 1 {
        adjacent.push(coord + &Coord(0, 1));
    }
    
    adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n\
    ";

    #[test]
    fn part_one() {
        let expected = 36;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 81;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
