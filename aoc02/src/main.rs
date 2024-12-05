use std::{cmp::Ordering, error::Error, fs, str::FromStr};

#[derive(Debug, Clone)]
struct Reports {
    nums: Vec<Vec<u8>>,
}

impl FromStr for Reports {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .lines()
            .map(|l| {
                l.split(" ")
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Reports { nums })
    }
}

fn part_1_count_safe(reports: &Reports) -> usize {
    reports
        .nums
        .iter()
        .filter_map(|levels| {
            if (levels
                .windows(2)
                .map(|x| TryInto::<&[u8; 2]>::try_into(x).unwrap())
                .all(|&[x, y]| y > x)
                || levels
                    .windows(2)
                    .map(|x| TryInto::<&[u8; 2]>::try_into(x).unwrap())
                    .all(|&[x, y]| x > y))
                && (levels
                    .windows(2)
                    .map(|x| TryInto::<&[u8; 2]>::try_into(x).unwrap())
                    .all(|&[x, y]| {
                        let diff = (y as i8 - x as i8).abs();
                        diff == 1 || diff == 2 || diff == 3
                    }))
            {
                Some(levels)
            } else {
                None
            }
        })
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let reports = input.parse()?;
    println!("Part 1: {}", part_1_count_safe(&reports));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn it_passes_testcase_1() {
        let reports = TEST_INPUT.parse::<Reports>().unwrap();
        assert_eq!(2, part_1_count_safe(&reports));
    }
}
