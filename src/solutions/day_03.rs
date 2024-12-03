use regex::Regex;

pub fn solve_part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for (_, [operand_a, operand_b]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += operand_a.parse::<usize>().unwrap() * operand_b.parse::<usize>().unwrap();
    }

    sum
}

pub fn solve_part_two(input: &str) -> usize {
    let re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for index in 0..input.len() {
        if input[index..].starts_with("do()") {
            enabled = true;
        }

        if input[index..].starts_with("don't()") {
            enabled = false;
        }

        if enabled {
            for (_, [operand_a, operand_b]) in re.captures_iter(&input[index..]).map(|c| c.extract()) {
                sum += operand_a.parse::<usize>().unwrap() * operand_b.parse::<usize>().unwrap();
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    #[test]
    fn part_one() {
        let expected = 161;
        
        assert_eq!(solve_part_one(INPUT_ONE), expected);
    }

    const INPUT_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_two() {
        let expected = 48;

        assert_eq!(solve_part_two(INPUT_TWO), expected);
    }
}
