use itertools::Itertools;

pub fn solve_part_one(input: &str) -> usize {
    let mut pairs = get_pairs(input);
    
    pairs.0.sort();
    pairs.1.sort();

    pairs.0.into_iter()
        .zip(pairs.1)
        .fold(0, |sum, pair| {
            let difference = (pair.0 as isize - pair.1 as isize).abs();
            sum + difference
        }) as usize
}

pub fn solve_part_two(input: &str) -> usize {
    let pairs = get_pairs(input);

    pairs.0.iter()
        .fold(0, |sum, id_a| {
            let match_count = pairs.1.iter().filter(|id_b| *id_b == id_a).count();
            sum + id_a * match_count
        })
}

fn get_pairs(input: &str) -> (Vec<usize>, Vec<usize>) {
    let pairs: Vec<(usize, usize)> = input
        .lines()
        .map(|pair| pair
            .split_whitespace()
            .map(|id| id.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap()
        ).collect();

    pairs.into_iter().unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3\n\
    ";

    #[test]
    fn part_one() {
        let expected = 11;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 31;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
