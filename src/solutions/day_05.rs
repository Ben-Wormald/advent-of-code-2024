use itertools::Itertools;
use std::cmp::Ordering;

struct Page {
    number: usize,
    rules: Vec<(usize, usize)>,
}
impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rules.contains(&(self.number, other.number)) {
            Ordering::Less
        } else if self.rules.contains(&(other.number, self.number)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
impl Eq for Page {}

pub fn solve_part_one(input: &str) -> usize {
    let updates = get_updates(input);

    updates.into_iter().fold(0, |sum, update| {
        if update.is_sorted() {
            sum + get_middle_page(&update)
        } else {
            sum
        }
    })
}

pub fn solve_part_two(input: &str) -> usize {
    let updates = get_updates(input);

    updates.into_iter().fold(0, |sum, update| {
        if !update.is_sorted() {
            let mut update = update;
            update.sort();
            sum + get_middle_page(&update)
        } else {
            sum
        }
    })
}

fn get_updates(input: &str) -> Vec<Vec<Page>> {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();

    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|rule| rule
            .split("|")
            .map(|page| page.parse().unwrap())
            .collect_tuple().unwrap()
        )
        .collect();

    let updates = updates
        .lines()
        .map(|update| {
            let pages = update
                .split(",")
                .map(|page| page.parse().unwrap())
                .collect::<Vec<usize>>();

            pages.iter()
                .map(|page| Page {
                    number: *page,
                    rules: get_matching_rules(&pages, &rules),
                })
                .collect()
        })
        .collect();

    updates
}

fn get_matching_rules(update: &[usize], rules: &[(usize, usize)]) -> Vec<(usize, usize)> {
    rules
        .iter()
        .copied()
        .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
        .collect()
}

fn get_middle_page(update: &[Page]) -> usize {
    update.get(update.len() / 2).unwrap().number
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
    ";

    #[test]
    fn part_one() {
        let expected = 143;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 123;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
