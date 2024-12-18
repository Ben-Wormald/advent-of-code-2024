use itertools::Itertools;

#[derive(Clone, Debug)]
struct Computer {
    registers: [usize; 3],
    program: Vec<Op>,
}
impl Computer {
    fn run(&mut self) -> Vec<usize> {
        let mut outputs = Vec::new();
        let mut ptr = 0;

        loop {
            let mut has_jumped = false;

            if let Some(op) = self.program.get(ptr) {
                match op.code {
                    OpCode::Adv => self.registers[0] /= 2_usize.pow(self.operand(op) as u32),
                    OpCode::Bxl => self.registers[1] ^= self.operand(op),
                    OpCode::Bst => self.registers[1] = self.operand(op) % 8,
                    OpCode::Jnz => if self.registers[0] != 0 {
                        ptr = self.operand(op) / 2;
                        has_jumped = true;
                    },
                    OpCode::Bxc => self.registers[1] ^= self.registers[2],
                    OpCode::Out => outputs.push(self.operand(op) % 8),
                    OpCode::Bdv => self.registers[1] = self.registers[0] / 2_usize.pow(self.operand(op) as u32),
                    OpCode::Cdv => self.registers[2] = self.registers[0] / 2_usize.pow(self.operand(op) as u32),
                }

                if !has_jumped {
                    ptr += 1;
                }
            } else {
                break;
            }
        }

        outputs
    }

    fn operand(&self, op: &Op) -> usize {
        match op.code {
            OpCode::Adv => self.combo_operand(op),
            OpCode::Bxl => op.operand,
            OpCode::Bst => self.combo_operand(op),
            OpCode::Jnz => op.operand,
            OpCode::Bxc => op.operand,
            OpCode::Out => self.combo_operand(op),
            OpCode::Bdv => self.combo_operand(op),
            OpCode::Cdv => self.combo_operand(op),
        }
    }

    fn combo_operand(&self, op: &Op) -> usize {
        match op.operand {
            0..=3 => op.operand,
            4..=6 => self.registers[op.operand - 4],
            _ => panic!("invalid operand"),
        }
    }
}

#[derive(Clone, Debug)]
struct Op {
    code: OpCode,
    operand: usize,
}

#[derive(Clone, Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl OpCode {
    fn from(code: &str) -> OpCode {
        match code {
            "0" => OpCode::Adv,
            "1" => OpCode::Bxl,
            "2" => OpCode::Bst,
            "3" => OpCode::Jnz,
            "4" => OpCode::Bxc,
            "5" => OpCode::Out,
            "6" => OpCode::Bdv,
            "7" => OpCode::Cdv,
            _ => panic!("unrecognised op code"),
        }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let mut computer = get_computer(input);
    let outputs = computer.run();
    outputs.into_iter().map(|output| output.to_string()).join(",")
}

pub fn solve_part_two(input: &str) -> String {
    let computer = get_computer(input);
    let mut init_a = 0;

    loop {
        if init_a % 100_000 == 0 {
            println!("{init_a}");
        }

        let mut computer = computer.clone();
        computer.registers[0] = init_a;

        let outputs = computer.run();
        let output = outputs.into_iter().map(|output| output.to_string()).join(",");

        let target_output = input.split("Program: ").last().unwrap().trim();

        if output == target_output {
            break init_a.to_string();
        }

        init_a += 1;
    }
}

fn get_computer(input: &str) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let registers = registers
        .lines()
        .map(|register| register
            .split(": ")
            .last().unwrap()
            .parse().unwrap()
        )
        .collect::<Vec<usize>>()
        .try_into().unwrap();

    let program = program
        .replace("Program: ", "")
        .trim()
        .split(",")
        .chunks(2)
        .into_iter()
        .map(|op| {
            let (code, operand) = op.collect_tuple().unwrap();

            let code = OpCode::from(code);
            let operand = operand.parse().unwrap();

            Op { code, operand }
        })
        .collect();

    Computer { registers, program }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0\n\
    ";

    #[test]
    fn part_one() {
        let expected = "4,6,3,5,6,3,5,2,1,0";

        assert_eq!(solve_part_one(INPUT_ONE), expected);
    }

    const INPUT_TWO: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0\n\
    ";

    #[test]
    fn part_two() {
        let expected = "117440";

        assert_eq!(solve_part_two(INPUT_TWO), expected);
    }
}
