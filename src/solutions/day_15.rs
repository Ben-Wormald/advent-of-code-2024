struct Map {
    map: Vec<Vec<Cell>>,
    boxes: Vec<Coord>,
    robot: Coord,
}
impl Map {
    fn get_cell(&self, coord: &Coord) -> Cell {
        self.map[coord.y as usize][coord.x as usize]
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Space,
    Wall,
}

#[derive(Clone, Copy, PartialEq)]
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

    fn move_one(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn get_gps(&self) -> usize {
        (self.x + 100 * self.y) as usize
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new(direction: char) -> Direction {
        match direction {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("bad direction"),
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let (mut map, directions) = get_map(input);
    
    for direction in directions.into_iter() {
        let mut pos = map.robot;
        let mut boxes_to_move = Vec::new();

        let can_move = loop {
            pos.move_one(&direction);

            if map.boxes.contains(&pos) {
                boxes_to_move.push(pos);
            } else {
                match map.get_cell(&pos) {
                    Cell::Space => break true,
                    Cell::Wall => break false,
                };
            }
        };

        if can_move {
            for box_to_move in boxes_to_move.into_iter() {
                map.boxes.iter_mut()
                    .find(|b| **b == box_to_move).unwrap()
                    .move_one(&direction);
            }

            map.robot.move_one(&direction);
        }
    }

    map.boxes.iter().map(Coord::get_gps).sum()
}

pub fn solve_part_two(_input: &str) -> usize {
    todo!()
}

fn get_map(input: &str) -> (Map, Vec<Direction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let mut boxes = Vec::new();
    let mut robot = None;

    let map = map
        .lines()
        .enumerate()
        .map(|(y, row)| row
            .chars()
            .enumerate()
            .map(|(x, cell)| match cell {
                '#' => Cell::Wall,
                '.' => Cell::Space,
                'O' => {
                    boxes.push(Coord::new(x, y));
                    Cell::Space
                },
                '@' => {
                    robot = Some(Coord::new(x, y));
                    Cell::Space
                },
                _ => panic!("bad map cell"),
            })
            .collect()
        )
        .collect();

    let robot = robot.expect("no robot found");

    let map = Map {
        map,
        boxes,
        robot,
    };

    let directions = directions
        .lines()
        .flat_map(|line| line
            .chars()
            .map(Direction::new)
        )
        .collect();

    (map, directions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
    ";

    #[test]
    fn part_one() {
        let expected = 10092;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 9021;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
