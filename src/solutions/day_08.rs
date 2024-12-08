use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};

use itertools::Itertools;

#[derive(Debug)]
struct Map {
    bounds: (isize, isize),
    antennae: HashMap<char, Vec<Coord>>,
}
impl Map {
    fn get_antinodes(&self) -> HashSet<Coord> {
        let mut antinodes = HashSet::new();

        for (_frequency, coords) in self.antennae.iter() {
            for (coord_a, coord_b) in coords.iter().tuple_combinations() {
                let delta = coord_a - coord_b;

                let antinode_a = coord_a + &delta;
                let antinode_b = coord_b - &delta;

                if self.is_within(&antinode_a) {
                    antinodes.insert(antinode_a);
                }
                if self.is_within(&antinode_b) {
                    antinodes.insert(antinode_b);
                }
            }
        }

        antinodes
    }

    fn get_antinodes_with_resonance(&self) -> HashSet<Coord> {
        let mut antinodes = HashSet::new();

        for (_frequency, coords) in self.antennae.iter() {
            for (coord_a, coord_b) in coords.iter().tuple_combinations() {
                let delta = coord_a - coord_b;

                antinodes.insert(coord_a.clone());
                antinodes.insert(coord_b.clone());

                let mut next_antinode = coord_a.clone();

                loop {
                    next_antinode = &next_antinode + &delta;
                    if self.is_within(&next_antinode) {
                        antinodes.insert(next_antinode.clone());
                    } else {
                        break;
                    }
                }

                let mut next_antinode = coord_b.clone();

                loop {
                    next_antinode = &next_antinode - &delta;
                    if self.is_within(&next_antinode) {
                        antinodes.insert(next_antinode.clone());
                    } else {
                        break;
                    }
                }
            }
        }

        antinodes
    }

    fn is_within(&self, coord: &Coord) -> bool {
        coord.0 >= 0 && coord.1 >= 0 && coord.0 < self.bounds.0 && coord.1 < self.bounds.1
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord(isize, isize);
impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }    
}
impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let map = get_map(input);
    let antinodes = map.get_antinodes();
    antinodes.len()
}

pub fn solve_part_two(input: &str) -> usize {
    let map = get_map(input);
    let antinodes = map.get_antinodes_with_resonance();
    antinodes.len()
}

fn get_map(input: &str) -> Map {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let bounds = (map[0].len() as isize, map.len() as isize);
    let mut antennae = HashMap::new();

    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell != '.' {
                let coords: &mut Vec<Coord> = antennae.entry(cell).or_default();
                coords.push(Coord(x as isize, y as isize));
            }
        }
    }

    Map { bounds, antennae }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n\
    ";

    #[test]
    fn part_one() {
        let expected = 14;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 34;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
