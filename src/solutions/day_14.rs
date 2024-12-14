use itertools::Itertools;
use std::ops::{Add, Mul, Rem};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
const BOUNDS: Vector = Vector { x: WIDTH, y: HEIGHT };

#[derive(Debug)]
struct Robot {
    p: Vector,
    v: Vector,
}
impl Robot {
    fn new(p: Vector, v: Vector) -> Robot {
        Robot { p, v }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vector {
    x: isize,
    y: isize,
}
impl Vector {
    fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }
}
impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Rem for Vector {
    type Output = Vector;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut robots = get_robots(input);
    let seconds = 100;
    
    for robot in robots.iter_mut() {
        robot.p = (robot.p + (robot.v * seconds)) % BOUNDS;
    }

    get_safety_factor(&robots)
}

pub fn solve_part_two(input: &str) -> usize {
    let mut robots = get_robots(input);
    
    let mut n = 0;
    loop {
        for robot in robots.iter_mut() {
            robot.p = (robot.p + robot.v) % BOUNDS;
        }
        n += 1;

        if robots.iter().map(|r| r.p).all_unique() {
            print_grid(&robots);
            break;
        }
    }
    
    n
}

fn get_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|robot| {
            let (p, v) = robot.split_whitespace().collect_tuple().unwrap();

            let (px, py) = p
                .replace("p=", "")
                .split(",")
                .map(|p| p.parse().unwrap())
                .collect_tuple().unwrap();

            let (vx, vy) = v
                .replace("v=", "")
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect_tuple().unwrap();

            Robot::new(
                Vector::new(px, py),
                Vector::new(vx, vy),
            )
        })
        .collect()
}

fn get_safety_factor(robots: &[Robot]) -> usize {
    let mid_x = BOUNDS.x / 2;
    let mid_y = BOUNDS.y / 2;

    let quadrants = [
        (0..mid_x, 0..mid_y),
        (0..mid_x, (mid_y + 1)..BOUNDS.y),
        ((mid_x + 1)..BOUNDS.x, 0..mid_y),
        ((mid_x + 1)..BOUNDS.x, (mid_y + 1)..BOUNDS.y),
    ];
    
    quadrants.into_iter().map(|quadrant| {
        robots.iter().filter(|robot| {
            quadrant.0.contains(&robot.p.x) && quadrant.1.contains(&robot.p.y)
        }).count()
    }).product()
}

fn print_grid(robots: &[Robot]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let robot_count = robots.iter().filter(|r| r.p == Vector::new(x, y)).count();

            let output = if robot_count == 0 {
                String::from("-")
            } else {
                robot_count.to_string()
            };
            print!("{output}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3\n\
    ";

    #[test]
    fn part_one() {
        let expected = 21;

        assert_eq!(solve_part_one(INPUT), expected);
    }
}
