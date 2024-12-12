use std::collections::HashMap;

const COUNT: usize = 25;
const COUNT_TWO: usize = 75;
const MULTIPLIER: usize = 2024;

type Cache = HashMap<(usize, usize), usize>;

pub fn solve_part_one(input: &str) -> usize {
    let stones = get_initial_stones(input);
    let mut cache: Cache = HashMap::new();

    stones.into_iter().map(|stone| get_count(stone, COUNT, &mut cache)).sum()
}

pub fn solve_part_two(input: &str) -> usize {
    let stones = get_initial_stones(input);
    let mut cache: Cache = HashMap::new();

    stones.into_iter().map(|stone| get_count(stone, COUNT_TWO, &mut cache)).sum()
}

fn get_initial_stones(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|stone| stone.parse().unwrap()).collect()
}

fn get_count(stone: usize, depth: usize, cache: &mut Cache) -> usize {
    if let Some(cached_count) = cache.get(&(stone, depth)) {
        return *cached_count;
    }

    let new_stones = if stone == 0 {
        vec!(1)
    } else {
        let stone_string = stone.to_string();

        if stone_string.len() % 2 == 0 {
            let (stone_a, stone_b) = stone_string.split_at(stone_string.len() / 2);
            vec!(stone_a.parse().unwrap(), stone_b.parse().unwrap())
        } else {
            vec!(stone * MULTIPLIER)
        }
    };

    let count = if depth == 1 {
        new_stones.len()
    } else {
        new_stones.into_iter().map(|stone| get_count(stone, depth - 1, cache)).sum()
    };

    cache.insert((stone, depth), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17\n";

    #[test]
    fn part_one() {
        let expected = 55312;

        assert_eq!(solve_part_one(INPUT), expected);
    }
}
