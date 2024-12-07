use std::{error::Error, fs};

fn solve_1(res: usize, nums: &[usize]) -> Option<Vec<char>> {
    let init_acc = vec![(nums[0], vec![])];
    let combinations = nums.iter().skip(1).fold(init_acc, |acc, &el| {
        acc.into_iter()
            .flat_map(|(num, ops)| {
                let add = (
                    num + el,
                    ops.iter().copied().chain(['+']).collect::<Vec<_>>(),
                );
                let mul = (
                    num * el,
                    ops.iter().copied().chain(['*']).collect::<Vec<_>>(),
                );
                vec![add, mul]
            })
            .collect::<Vec<_>>()
    });
    combinations
        .into_iter()
        .find_map(|(num, ops)| if num == res { Some(ops) } else { None })
}

fn sum_calibration_results_1(puzzle: &str) -> usize {
    puzzle
        .lines()
        .map(|l| {
            let (res, nums) = l.split_once(":").unwrap();
            let res = res.parse::<usize>().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (res, nums)
        })
        .filter_map(|(res, nums)| solve_1(res, &nums).map(|_| res))
        .sum()
}

fn solve_2(res: usize, nums: &[usize]) -> Option<Vec<char>> {
    let init_acc = vec![(nums[0], vec![])];
    let combinations = nums.iter().skip(1).fold(init_acc, |acc, &el| {
        acc.into_iter()
            .flat_map(|(num, ops)| {
                let add = (
                    num + el,
                    ops.iter().copied().chain(['+']).collect::<Vec<_>>(),
                );
                let mul = (
                    num * el,
                    ops.iter().copied().chain(['*']).collect::<Vec<_>>(),
                );
                let cat = (
                    format!("{num}{el}").parse::<usize>().unwrap(),
                    ops.iter().copied().chain(['|']).collect::<Vec<_>>(),
                );
                vec![add, mul, cat]
            })
            .collect::<Vec<_>>()
    });
    combinations
        .into_iter()
        .find_map(|(num, ops)| if num == res { Some(ops) } else { None })
}

fn sum_calibration_results_2(puzzle: &str) -> usize {
    puzzle
        .lines()
        .map(|l| {
            let (res, nums) = l.split_once(":").unwrap();
            let res = res.parse::<usize>().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (res, nums)
        })
        .filter_map(|(res, nums)| solve_2(res, &nums).map(|_| res))
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", sum_calibration_results_1(&input));
    println!("Part 2: {}", sum_calibration_results_2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_solve_1() {
        assert_eq!(Some(vec!['*']), solve_1(190, &[10, 19]));
        assert_eq!(None, solve_1(156, &[15, 6]));
    }

    #[test]
    fn it_passes_testcase_1() {
        assert_eq!(3749, sum_calibration_results_1(TEST_INPUT));
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(Some(vec!['|']), solve_2(156, &[15, 6]));
        assert_eq!(Some(vec!['|', '+']), solve_2(192, &[17, 8, 14]));
    }

    #[test]
    fn it_passes_testcase_2() {
        assert_eq!(11387, sum_calibration_results_2(TEST_INPUT));
    }
}
