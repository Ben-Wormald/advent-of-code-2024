use std::collections::{HashMap, HashSet};

const MOVE_SCORE: usize = 1;
const TURN_SCORE: usize = 1000;

#[derive(Debug)]
struct Maze {
    cells: Vec<Vec<Cell>>,
    start: Coord,
    end: Coord,
}
impl Maze {
    fn get_shortest_path(&self) -> usize {
        let mut nodes = HashMap::new();
        let mut unvisited = HashSet::new();

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Space = cell {
                    let pos = Coord::new(x, y);

                    for heading in Direction::iter() {
                        let distance = if self.start == pos && heading == Direction::E {
                            Some(0)
                        } else {
                            None
                        };

                        let node = Node { pos, heading };
    
                        nodes.insert(node, distance);
                        unvisited.insert(node);
                    }
                }
            }
        }

        loop {
            let current = unvisited.iter()
                .cloned()
                .map(|node| (node, nodes.get(&node).unwrap()))
                .filter_map(|(node, distance)| distance.map(|distance| (node, distance)))
                .min_by(|a, b| a.1.cmp(&b.1));

            if let Some(current) = current {
                if current.0.pos == self.end {
                    break current.1;
                }

                let neighbours = self.get_neighbours(&current.0);
                for neighbour in neighbours.into_iter() {
                    let distance = current.1 + neighbour.1;

                    if nodes.get(&neighbour.0).unwrap().is_none_or(|d| d > distance) {
                        nodes.insert(neighbour.0, Some(distance));
                    }
                }

                unvisited.remove(&current.0);
            } else {
                break 0;
            }
        }
    }

    fn get_neighbours(&self, current: &Node) -> Vec<(Node, usize)> {
        let mut neighbours = Vec::new();

        neighbours.push((current.left(), TURN_SCORE));
        neighbours.push((current.right(), TURN_SCORE));

        let forward = current.foward();

        if forward.pos.x >= 0 && forward.pos.y >= 0 {
            let x = forward.pos.x as usize;
            let y = forward.pos.y as usize;

            if let Some(Cell::Space) = self.cells.get(y).and_then(|row| row.get(x)) {
                neighbours.push((forward, MOVE_SCORE));
            }
        }

        neighbours
    }
}

#[derive(Debug)]
enum Cell {
    Space,
    Wall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x as isize,
            y: y as isize,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}
impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ].into_iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Node {
    pos: Coord,
    heading: Direction,
}
impl Node {
    fn foward(&self) -> Node {
        match self.heading {
            Direction::N => Node { pos: Coord { x: self.pos.x, y: self.pos.y - 1 }, heading: self.heading },
            Direction::E => Node { pos: Coord { x: self.pos.x + 1, y: self.pos.y }, heading: self.heading },
            Direction::S => Node { pos: Coord { x: self.pos.x, y: self.pos.y + 1 }, heading: self.heading },
            Direction::W => Node { pos: Coord { x: self.pos.x - 1, y: self.pos.y }, heading: self.heading },
        }
    }

    fn left(&self) -> Node {
        match self.heading {
            Direction::N => Node { pos: self.pos, heading: Direction::W },
            Direction::E => Node { pos: self.pos, heading: Direction::N },
            Direction::S => Node { pos: self.pos, heading: Direction::E },
            Direction::W => Node { pos: self.pos, heading: Direction::S },
        }
    }

    fn right(&self) -> Node {
        match self.heading {
            Direction::N => Node { pos: self.pos, heading: Direction::E },
            Direction::E => Node { pos: self.pos, heading: Direction::S },
            Direction::S => Node { pos: self.pos, heading: Direction::W },
            Direction::W => Node { pos: self.pos, heading: Direction::N },
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let maze = get_maze(input);
    maze.get_shortest_path()
}

pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

fn get_maze(input: &str) -> Maze {
    let mut start = None;
    let mut end = None;

    let cells = input
        .lines()
        .enumerate()
        .map(|(y, row)| row
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                '#' => Cell::Wall,
                '.' => Cell::Space,
                'S' => {
                    start = Some(Coord::new(x, y));
                    Cell::Space
                },
                'E' => {
                    end = Some(Coord::new(x, y));
                    Cell::Space
                },
                _ => panic!("unrecognised cell {c}"),
            })
            .collect()
        )
        .collect();

    let start = start.unwrap();
    let end = end.unwrap();

    Maze { cells, start, end }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############\n\
    ";

    #[test]
    fn part_one() {
        let expected = 7036;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 45;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
