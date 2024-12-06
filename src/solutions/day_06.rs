use std::{collections::HashSet, ops::Add};

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    guard_coord: Coord,
    guard_direction: Direction,
    previous_states: HashSet<(Coord, Direction)>,
}
impl Grid {
    fn new(cells: Vec<Vec<Cell>>, guard_coord: Coord, guard_direction: Direction) -> Grid {
        Grid {
            cells,
            guard_coord,
            guard_direction,
            previous_states: HashSet::new(),
        }
    }
    
    fn step(&mut self) -> State {
        let next_coord = self.guard_coord + match self.guard_direction {
            Direction::Up => Coord(0, -1),
            Direction::Down => Coord(0, 1),
            Direction::Left => Coord(-1, 0),
            Direction::Right => Coord(1, 0),
        };

        let moved_out = match self.get_cell(next_coord) {
            Some(Cell::Unvisited) | Some(Cell::Visited) => {
                self.guard_coord = next_coord;
                self.cells[next_coord.1 as usize][next_coord.0 as usize] = Cell::Visited;
                false
            },
            Some(Cell::Obstacle) => {
                self.guard_direction = self.guard_direction.turn();
                false
            },
            None => true,
        };

        if moved_out {
            State::Out
        } else if self.previous_states.contains(&(self.guard_coord, self.guard_direction)) {
            State::Loop
        } else {
            self.previous_states.insert((self.guard_coord, self.guard_direction));
            State::In
        }
    }

    fn get_cell(&self, coord: Coord) -> Option<&Cell> {
        if coord.0 < 0 || coord.1 < 0 {
            return None;
        }

        self.cells.get(coord.1 as usize).and_then(|row| row.get(coord.0 as usize))
    }

    fn count_visited(&self) -> usize {
        self.cells
            .iter()
            .fold(0, |sum, row| sum + row
                .iter()
                .fold(0, |sum, cell| match cell {
                    Cell::Visited => sum + 1,
                    _ => sum,
                })
            )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);
impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }    
}

#[derive(Clone, PartialEq)]
enum Cell {
    Unvisited,
    Visited,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn from(cell: char) -> Direction {
        match cell {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(PartialEq)]
enum State {
    In,
    Out,
    Loop
}

pub fn solve_part_one(input: &str) -> usize {
    let mut grid = get_grid(input);

    loop {
        if grid.step() == State::Out {
            break;
        }
    }

    grid.count_visited()
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = get_grid(input);

    let mut obstacle_count = 0;

    for (y, row) in grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if Coord(x as isize, y as isize) == grid.guard_coord || *cell == Cell::Obstacle {
                continue;
            }

            let mut modified_grid = grid.clone();
            modified_grid.cells[y][x] = Cell::Obstacle;

            loop {
                match modified_grid.step() {
                    State::Out => break,
                    State::Loop => {
                        obstacle_count += 1;
                        break;
                    },
                    State::In => (),
                }
            }
        }
    }

    obstacle_count
}

fn get_grid(input: &str) -> Grid {
    let mut guard_coord: Option<Coord> = None;
    let mut guard_direction: Option<Direction> = None;

    let cells = input
        .lines()
        .enumerate()
        .map(|(y, row)| row
            .chars()
            .enumerate()
            .map(|(x, cell)| match cell {
                '.' => Cell::Unvisited,
                '^' | 'v' | '<' | '>' => {
                    guard_coord = Some(Coord(x as isize, y as isize));
                    guard_direction = Some(Direction::from(cell));
                    Cell::Visited
                },
                '#' => Cell::Obstacle,
                _ => unreachable!(),
            })
            .collect()
        )
        .collect();

    Grid::new(cells, guard_coord.unwrap(), guard_direction.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n\
    ";

    #[test]
    fn part_one() {
        let expected = 41;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 6;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
