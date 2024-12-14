use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};

struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

#[allow(non_snake_case)]
pub fn solve_part_one(input: &str) -> usize {
    get_machines(input).fold(0, |sum, machine| {
        let A = Matrix2::new(
            machine.a.0, machine.b.0,
            machine.a.1, machine.b.1,
        );
        let B = Matrix2x1::new(
            machine.prize.0,
            machine.prize.1,
        );

        let X = A.try_inverse().unwrap() * B;

        if is_int(X[0]) && is_int(X[1]) {
            sum + 3 * (X[0].round() as usize) + (X[1].round() as usize)
        } else {
            sum
        }
    })
}

#[allow(non_snake_case)]
pub fn solve_part_two(input: &str) -> usize {
    get_machines(input).fold(0, |sum, machine| {
        let px = machine.prize.0 + 10000000000000.0;
        let py = machine.prize.1 + 10000000000000.0;
        let ax = machine.a.0;
        let ay = machine.a.1;
        let bx = machine.b.0;
        let by = machine.b.1;

        let b = (py * ax - px * ay) / (by * ax - bx * ay);
        let a = ( px - b * bx) / ax;

        let a_rounded = a.round();
        let b_rounded = b.round();

        if a_rounded * ax + b_rounded * bx == px && a_rounded * ay + b_rounded * by == py {
            sum + 3 * (a_rounded as usize) + (b_rounded as usize)
        } else {
            sum
        }
    })
}

fn get_machines(input: &str) -> impl Iterator<Item = Machine> + use<'_> {
    input
        .split("\n\n")
        .map(|machine| {
            let (a, b, prize) = machine.lines().collect_tuple().unwrap();

            let a = a
                .replace("Button A: X+", "")
                .split(", Y+")
                .map(|d| d.parse().unwrap())
                .collect_tuple().unwrap();

            let b = b
                .replace("Button B: X+", "")
                .split(", Y+")
                .map(|d| d.parse().unwrap())
                .collect_tuple().unwrap();

            let prize = prize
                .replace("Prize: X=", "")
                .split(", Y=")
                .map(|d| d.parse().unwrap())
                .collect_tuple().unwrap();

            Machine { a, b, prize }
        })
}

fn is_int(n: f64) -> bool {
    let fract = n.fract();
    fract < 1e-10 || (fract - 1.0).abs() < 1e-10
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n\
    ";

    #[test]
    fn part_one() {
        let expected = 480;

        assert_eq!(solve_part_one(INPUT), expected);
    }
}
