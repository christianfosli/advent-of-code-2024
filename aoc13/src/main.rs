use std::{borrow::Borrow, error::Error, fs, str::FromStr};

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

struct ClawMachineProblem {
    btn_a: Point,
    btn_b: Point,
    prize: Point,
}

impl FromStr for ClawMachineProblem {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();

        let str_a = lines_iter.next().unwrap();
        let re_caps_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?
            .captures(str_a)
            .unwrap();
        let btn_a = Point {
            x: re_caps_a[1].parse::<f64>()?,
            y: re_caps_a[2].parse::<f64>()?,
        };

        let str_b = lines_iter.next().unwrap();
        let re_caps_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?
            .captures(str_b)
            .unwrap();
        let btn_b = Point {
            x: re_caps_b[1].parse::<f64>()?,
            y: re_caps_b[2].parse::<f64>()?,
        };

        let str_prize = lines_iter.next().unwrap();
        let re_caps_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?
            .captures(str_prize)
            .unwrap();
        let prize = Point {
            x: re_caps_p[1].parse::<f64>()?,
            y: re_caps_p[2].parse::<f64>()?,
        };

        Ok(ClawMachineProblem {
            btn_a,
            btn_b,
            prize,
        })
    }
}

impl ClawMachineProblem {
    fn solve_1(&self) -> Option<f64> {
        variables! { vars:
            a (integer) <= 100;
            b (integer) <= 100;
        }

        let objective = a * 3 + b;

        let result = vars
            .minimise(&objective)
            .using(default_solver)
            .with(constraint!(
                a * self.btn_a.x + b * self.btn_b.x == self.prize.x
            ))
            .with(constraint!(
                a * self.btn_a.y + b * self.btn_b.y == self.prize.y
            ))
            .solve();

        match result {
            Ok(solution) => Some(solution.eval(&objective)),
            Err(_) => None,
        }
    }

    fn solve_2(self) -> Option<f64> {
        variables! { vars:
            a (integer);
            b (integer);
        }

        let objective = a * 3 + b;

        let result = vars
            .minimise(&objective)
            .using(default_solver)
            .with(constraint!(
                a * self.btn_a.x + b * self.btn_b.x == self.prize.x
            ))
            .with(constraint!(
                a * self.btn_a.y + b * self.btn_b.y == self.prize.y
            ))
            .solve();

        match result {
            Ok(solution) => Some(solution.eval(&objective)),
            Err(_) => None,
        }
    }
}

fn min_token_cost_p1(machines: &[ClawMachineProblem]) -> f64 {
    machines
        .into_iter()
        .filter_map(ClawMachineProblem::solve_1)
        .sum()
}

fn min_token_cost_p2(machines: &[ClawMachineProblem]) -> f64 {
    machines
        .into_iter()
        .map(|m| ClawMachineProblem {
            btn_a: m.btn_a,
            btn_b: m.btn_b,
            prize: Point {
                x: m.prize.x + 10_000_000_000_000f64,
                y: m.prize.y + 10_000_000_000_000f64,
            },
        })
        .filter_map(ClawMachineProblem::solve_2)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let machines = input
        .split("\n\n")
        .map(|s| s.parse::<ClawMachineProblem>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}", min_token_cost_p1(&machines));
    println!("Part 2: {}", min_token_cost_p2(&machines));

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
    fn test_solve() {
        let problem = ClawMachineProblem {
            btn_a: Point { x: 94.0, y: 34.0 },
            btn_b: Point { x: 22.0, y: 67.0 },
            prize: Point {
                x: 8400.0,
                y: 5400.0,
            },
        };

        assert_eq!(Some(280f64), problem.solve_1());
    }

    #[test]
    fn it_passes_testcase_1() {
        let machines = TEST_INPUT
            .split("\n\n")
            .map(|s| s.parse::<ClawMachineProblem>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(min_token_cost_p1(&machines), 480f64)
    }
}
