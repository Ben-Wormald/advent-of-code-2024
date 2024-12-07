use itertools::{repeat_n, Itertools};

#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
    Concat,
}
impl Op {
    fn ops() -> [Op; 2] {
        [Op::Add, Op::Multiply]
    }

    fn ops_all() -> [Op; 3] {
        [Op::Add, Op::Multiply, Op::Concat]
    }
}

trait Equation {
    fn has_solution(&self, ops: &[Op]) -> bool;
    fn evaluate(&self, ops: &[&Op]) -> usize;
}
impl Equation for (usize, Vec<usize>) {
    fn has_solution(&self, ops: &[Op]) -> bool {
        let op_count = self.1.len() - 1;
        let combinations = repeat_n(ops.iter(), op_count).multi_cartesian_product();
        let mut solution_found = false;

        for combination in combinations {
            if self.evaluate(&combination) == self.0 {
                solution_found = true;
                break;
            }
        }

        solution_found
    }

    fn evaluate(&self, ops: &[&Op]) -> usize {
        let mut op_index = 0;
        self.1.iter().copied().reduce(|result, operand| {
            let result = match ops[op_index] {
                Op::Add => result + operand,
                Op::Multiply => result * operand,
                Op::Concat => format!("{}{}", result, operand).parse().unwrap(),
            };

            op_index += 1;
            result
        }).unwrap()
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let equations = get_equations(input);
    let ops = Op::ops();

    equations.into_iter().fold(0, |sum, equation| {
        if equation.has_solution(&ops) {
            sum + equation.0
        } else {
            sum
        }
    })
}

pub fn solve_part_two(input: &str) -> usize {
    let equations = get_equations(input);
    let ops = Op::ops_all();

    equations.into_iter().fold(0, |sum, equation| {
        if equation.has_solution(&ops) {
            sum + equation.0
        } else {
            sum
        }
    })
}

fn get_equations(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|equation| {
            let (result, operands) = equation.split(": ").collect_tuple().unwrap();

            let result = result.parse().unwrap();
            let operands = operands
                .split_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect();

            (result, operands)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n\
    ";

    #[test]
    fn part_one() {
        let expected = 3749;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 11387;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
