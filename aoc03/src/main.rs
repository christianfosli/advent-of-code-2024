use std::{error::Error, fs};

use regex::Regex;

fn sum_valid_muls_1(s: &str) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    Ok(re
        .captures_iter(s)
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .sum())
}

fn sum_valid_muls_2(s: &str) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)")?;
    Ok(re
        .captures_iter(s)
        .fold((true, 0), |(is_enabled, sum), cap| {
            match (&cap[0], is_enabled) {
                ("do()", _) => (true, sum),
                ("don't()", _) => (false, sum),
                (op, true) if op.starts_with("mul") => (
                    is_enabled,
                    sum + cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap(),
                ),
                (op, false) if op.starts_with("mul") => (is_enabled, sum),
                _ => unreachable!(),
            }
        })
        .1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {:?}", sum_valid_muls_1(&input));
    println!("Part 2: {:?}", sum_valid_muls_2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_testcase_1() {
        let corrupted_mem =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, sum_valid_muls_1(corrupted_mem).unwrap());
    }

    #[test]
    fn it_passes_testcase_2() {
        let corrupted_mem =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, sum_valid_muls_2(corrupted_mem).unwrap());
    }
}
