use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);
impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord(x as isize, y as isize)
    }

    fn is_adjacent(&self, other: &Coord) -> bool {
        *self + Coord(-1, 0) == *other
            || *self + Coord(1, 0) == *other
            || *self + Coord(0, -1) == *other
            || *self + Coord(0, 1) == *other
    }
}
impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }    
}

pub fn solve_part_one(input: &str) -> usize {
    let map = get_map(input);
    let regions = get_regions(map);

    regions.iter().map(get_price).sum()
}

pub fn solve_part_two(input: &str) -> usize {
    let map = get_map(input);
    let regions = get_regions(map);

    regions.iter().map(get_discounted_price).sum()
}

fn get_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|row| row
            .chars()
            .collect()
        )
        .collect()
}

fn get_regions(map: Vec<Vec<char>>) -> Vec<(char, Vec<Coord>)> {
    let mut regions: Vec<(char, Vec<Coord>)> = Vec::new();

    for (y, row) in map.into_iter().enumerate() {
        for (x, plot_type) in row.into_iter().enumerate() {
            let plot = Coord::new(x, y);

            let mut connected_regions: Vec<&mut (char, Vec<Coord>)> = regions
                .iter_mut()
                .filter(|region|
                    region.0 == plot_type && region.1.iter().any(|p| p.is_adjacent(&plot))
                )
                .collect();

            if !connected_regions.is_empty() {
                let (new_region, old_regions) = connected_regions.split_first_mut().unwrap();

                new_region.1.push(plot);

                for old_region in old_regions.iter_mut() {
                    new_region.1.append(&mut old_region.1);
                }

            } else {
                regions.push((plot_type, vec!(plot)));
            }
        }
    }

    regions
}

fn get_price(region: &(char, Vec<Coord>)) -> usize {
    let (_plot_type, region) = region;

    let area = region.len();

    let perimeter = region.iter().fold(0, |sum, plot| {
        let neighbours = region.iter().filter(|p| p.is_adjacent(plot)).count();
        sum + (4 - neighbours)
    });

    area * perimeter
}

#[derive(PartialEq)]
enum Side {
    A,
    B,
}

fn get_discounted_price(region: &(char, Vec<Coord>)) -> usize {
    let (_plot_type, region) = region;

    let area = region.len();

    if area == 0 {
        return 0;
    }

    let min_x = region.iter().min_by(|plot_a, plot_b| plot_a.0.cmp(&plot_b.0)).unwrap().0 - 1;
    let min_y = region.iter().min_by(|plot_a, plot_b| plot_a.1.cmp(&plot_b.1)).unwrap().1 - 1;

    let max_x = region.iter().max_by(|plot_a, plot_b| plot_a.0.cmp(&plot_b.0)).unwrap().0 + 1;
    let max_y = region.iter().max_by(|plot_a, plot_b| plot_a.1.cmp(&plot_b.1)).unwrap().1 + 1;

    let mut sides = 0;

    for x in min_x..max_x {
        let mut side = None;

        for y in min_y..max_y {
            let is_plot_a = region.iter().any(|plot| *plot == Coord(x, y));
            let is_plot_b = region.iter().any(|plot| *plot == Coord(x + 1, y));

            let new_side = match (is_plot_a, is_plot_b) {
                (true, false) => Some(Side::A),
                (false, true) => Some(Side::B),
                _ => None,
            };

            if side.is_some() && side != new_side {
                sides += 1;
            }

            side = new_side;
        }

        if side.is_some() {
            sides += 1;
        }
    }

    for y in min_y..max_y {
        let mut side = None;

        for x in min_x..max_x {
            let is_plot_a = region.iter().any(|plot| *plot == Coord(x, y));
            let is_plot_b = region.iter().any(|plot| *plot == Coord(x, y + 1));

            let new_side = match (is_plot_a, is_plot_b) {
                (true, false) => Some(Side::A),
                (false, true) => Some(Side::B),
                _ => None,
            };

            if side.is_some() && side != new_side {
                sides += 1;
            }

            side = new_side;
        }

        if side.is_some() {
            sides += 1;
        }
    }

    area * sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE\n\
    ";

    #[test]
    fn part_one() {
        let expected = 1930;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 1206;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
