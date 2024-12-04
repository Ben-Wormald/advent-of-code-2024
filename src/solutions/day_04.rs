use itertools::Itertools;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

trait GridRef<T> {
    fn safe_get(&self, index: isize) -> Option<&T>;
}

impl GridRef<Vec<char>> for Vec<Vec<char>> {
    fn safe_get(&self, index: isize) -> Option<&Vec<char>> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}

impl GridRef<char> for Vec<char> {
    fn safe_get(&self, index: isize) -> Option<&char> {
        if index < 0 {
            None
        } else {
            self.get(index as usize)
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let grid = get_grid(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            for direction in DIRECTIONS.iter() {
                let letters = (0..4)
                    .map(|step| {
                        let step_x = x as isize + step * direction.0;
                        let step_y = y as isize + step * direction.1;

                        grid.safe_get(step_y).and_then(|row| row.safe_get(step_x))
                    })
                    .collect_tuple().unwrap();

                    if let (Some('X'), Some('M'), Some('A'), Some('S')) = letters {
                        count += 1;
                    }
            }
        }
    }

    count
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = get_grid(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if *letter == 'A' {
                let axis_a = (
                    grid.safe_get(y as isize - 1).and_then(|row| row.safe_get(x as isize - 1)),
                    grid.get(y + 1).and_then(|row| row.get(x + 1)),
                );

                match axis_a {
                    (Some('M'), Some('S')) | (Some('S'), Some('M')) => {
                        let axis_b = (
                            grid.safe_get(y as isize - 1).and_then(|row| row.get(x + 1)),
                            grid.get(y + 1).and_then(|row| row.safe_get(x as isize - 1)),
                        );

                        match axis_b {
                            (Some('M'), Some('S')) | (Some('S'), Some('M')) => count += 1,
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    count
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n\
    ";

    #[test]
    fn part_one() {
        let expected = 18;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 9;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
