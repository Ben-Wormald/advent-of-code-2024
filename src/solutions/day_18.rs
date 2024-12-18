use std::{cmp, collections::{HashMap, HashSet}};

struct Grid<'a> {
    size: usize,
    blocked: &'a [Coord],
}
impl Grid<'_> {
    fn get_shortest_path(&self) -> Option<usize> {
        let mut nodes = HashMap::new();
        let mut unvisited = HashSet::new();

        for x in 0..=self.size {
            for y in 0..=self.size {
                let node = Coord::new(x, y);

                let distance = if node == Coord::new(0, 0) {
                    Some(0)
                } else {
                    None
                };

                nodes.insert(node, distance);
                unvisited.insert(node);
            }
        }

        loop {
            let current = unvisited.iter()
                .cloned()
                .map(|node| (node, nodes.get(&node).unwrap()))
                .filter_map(|(node, distance)| distance.map(|distance| (node, distance)))
                .min_by(|a, b| a.1.cmp(&b.1));

            if let Some(current) = current {
                if current.0 == Coord::new(self.size, self.size) {
                    break Some(current.1);
                }

                let neighbours = self.get_neighbours(&current.0);
                for neighbour in neighbours.into_iter() {
                    let distance = current.1 + 1;

                    if nodes.get(&neighbour).unwrap().is_none_or(|d| d > distance) {
                        nodes.insert(neighbour, Some(distance));
                    }
                }

                unvisited.remove(&current.0);
            } else {
                break None;
            }
        }
    }

    fn get_neighbours(&self, coord: &Coord) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        if coord.x > 0 {
            neighbours.push(Coord { x: coord.x - 1, y: coord.y });
        }

        if coord.y > 0 {
            neighbours.push(Coord { x: coord.x, y: coord.y - 1 });
        }

        if coord.x < self.size as isize {
            neighbours.push(Coord { x: coord.x + 1, y: coord.y });
        }

        if coord.y < self.size as isize {
            neighbours.push(Coord { x: coord.x, y: coord.y + 1 });
        }

        neighbours.into_iter().filter(|n| !self.blocked.contains(n)).collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    fn from(coord: (&str, &str)) -> Coord {
        Coord {
            x: coord.0.parse().unwrap(),
            y: coord.1.parse().unwrap(),
        }
    }

    fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x as isize,
            y: y as isize,
        }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let grid_size = 70;
    let bytes = 1024;
    get_shortest_path(input, grid_size, bytes).to_string()
}

fn get_shortest_path(input: &str, grid_size: usize, bytes: usize) -> usize {
    let coords = get_coords(input);
    let blocked = &coords[..bytes];

    let grid = Grid { size: grid_size, blocked };

    grid.get_shortest_path().unwrap()
}

pub fn solve_part_two(input: &str) -> String {
    let grid_size = 70;
    get_first_blocker(input, grid_size)
}

fn get_first_blocker(input: &str, grid_size: usize) -> String {
    let coords = get_coords(input);

    let mut d = coords.len() / 2;
    let mut n = d;
    let mut is_blocked_at = HashMap::new();

    loop {
        d = cmp::max(d / 2, 1);

        let blocked = &coords[..=n];
        let grid = Grid { size: grid_size, blocked };

        if grid.get_shortest_path().is_some() {
            if is_blocked_at.get(&(n + 1)).is_some_and(|b| *b) {
                let coord = coords[n + 1];
                break format!("{},{}", coord.x, coord.y);
            }

            is_blocked_at.insert(n, false);
            n += d;
        } else {
            if is_blocked_at.get(&(n - 1)).is_some_and(|b| !b) {
                let coord = coords[n];
                break format!("{},{}", coord.x, coord.y);
            }

            is_blocked_at.insert(n, true);
            n -=d;
        }
    }
}

fn get_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|coord| Coord::from(coord.split_once(",").unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0\n\
    ";

    #[test]
    fn part_one() {
        let grid_size = 6;
        let bytes = 12;

        let expected = 22;

        assert_eq!(get_shortest_path(INPUT, grid_size, bytes), expected);
    }

    #[test]
    fn part_two() {
        let grid_size = 6;

        let expected = "6,1";

        assert_eq!(get_first_blocker(INPUT, grid_size), expected);
    }
}
