use std::fs;

fn is_correct(rules: &[(&str, &str)], update: &[&str]) -> bool {
    rules.iter().all(|&(r1, r2)| {
        if update.contains(&r1) && update.contains(&r2) {
            update.iter().position(|&x| x == r1) < update.iter().position(|&x| x == r2)
        } else {
            true
        }
    })
}

fn sum_corr_ord_1(s: &str) -> usize {
    let (rules, updates) = s.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .collect::<Vec<_>>();

    let updates = updates
        .lines()
        .map(|l| l.split(',').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    updates
        .into_iter()
        .filter_map(|u| {
            if is_correct(&rules, &u) {
                Some(u[u.len() / 2].parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", sum_corr_ord_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn it_passes_testcase_1() {
        assert_eq!(143, sum_corr_ord_1(TEST_INPUT));
    }
}
