trait Report {
    fn is_valid(&self) -> bool;
    fn without(&self, index: usize) -> Self;
}

impl Report for Vec<usize> {
    fn is_valid(&self) -> bool {
        let mut reversed = self.clone();
        reversed.reverse();

        let is_sorted = self.is_sorted() || reversed.is_sorted();

        let is_valid = self.windows(2).all(|pair| {
            let diff = (pair[0] as isize - pair[1] as isize).abs();
            (1..=3).contains(&diff)
        });

        is_sorted && is_valid
    }

    fn without(&self, index: usize) -> Self {
        let mut new = self.clone();
        new.remove(index);
        new
    }
}

pub fn solve_part_one(input: &str) -> usize {
    get_reports(input)
        .filter(|report| report.is_valid())
        .count()
}

pub fn solve_part_two(input: &str) -> usize {
    get_reports(input)
        .filter(|report| {
            let mut is_valid = report.is_valid();

            for index in 0..report.len() {
                let report_without = report.without(index);
                if report_without.is_valid() {
                    is_valid = true;
                }
            }

            is_valid
        })
        .count()
}

fn get_reports(input: &str) -> impl Iterator<Item = Vec<usize>> + use<'_> {
    input.lines()
        .map(|report| report
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect::<Vec<usize>>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
    ";

    #[test]
    fn part_one() {
        let expected = 2;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 4;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
