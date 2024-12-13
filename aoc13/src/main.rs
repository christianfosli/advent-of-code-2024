use std::{error::Error, fs, str::FromStr};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Button {
    A(u32, u32),
    B(u32, u32),
}

impl Button {
    fn cost(self) -> u32 {
        match self {
            Button::A(_, _) => 1,
            Button::B(_, _) => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct ClawMachine {
    buttons: (Button, Button),
    prize: (u32, u32),
}

impl FromStr for ClawMachine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();

        let str_a = lines_iter.next().unwrap();
        let re_caps_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?
            .captures(str_a)
            .unwrap();
        let btn_a = Button::A(re_caps_a[1].parse::<u32>()?, re_caps_a[2].parse::<u32>()?);

        let str_b = lines_iter.next().unwrap();
        let re_caps_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?
            .captures(str_b)
            .unwrap();
        let btn_b = Button::B(re_caps_b[1].parse::<u32>()?, re_caps_b[2].parse::<u32>()?);

        let str_prize = lines_iter.next().unwrap();
        let re_caps_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?
            .captures(str_prize)
            .unwrap();
        let prize = (re_caps_p[1].parse::<u32>()?, re_caps_p[2].parse::<u32>()?);

        Ok(ClawMachine {
            buttons: (btn_a, btn_b),
            prize,
        })
    }
}

fn solve(machine: &ClawMachine) -> Option<usize> {
    // TODO: Use linear programming to win the price with the least number of tokens or return None if insolvable
    // See crate good_lp
    unimplemented!()
}

fn min_token_cost_p1(machines: &[ClawMachine]) -> usize {
    machines.into_iter().filter_map(solve).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let machines = input
        .split("\n\n")
        .map(|s| s.parse::<ClawMachine>())
        .collect::<Result<Vec<_>, _>>()?;
    println!("Part 1: {}", min_token_cost_p1(&machines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn it_passes_testcase_1() {
        let machines = TEST_INPUT
            .split("\n\n")
            .map(|s| s.parse::<ClawMachine>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(min_token_cost_p1(&machines), 480)
    }
}
