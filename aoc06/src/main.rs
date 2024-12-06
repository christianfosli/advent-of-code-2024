use std::{error::Error, fs};

use rotate_enum::RotateEnum;

#[derive(Copy, Clone, Debug, PartialEq, RotateEnum)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn predict_guard_exit(map: &str) -> usize {
    let width = map.lines().next().map(|l| l.len()).unwrap_or(0);
    let height = map.lines().count();

    let mut map = map
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<_>>();

    let guard = map
        .iter()
        .enumerate()
        .find(|&(_i, &c)| c == '<' || c == '^' || c == '>' || c == 'v')
        .map(|(i, &c)| match c {
            '^' => (Direction::Up, i),
            '>' => (Direction::Right, i),
            'v' => (Direction::Down, i),
            '<' => (Direction::Left, i),
            _ => unreachable!(),
        });

    if let Some((mut guard_dir, mut guard_ix)) = guard {
        loop {
            map[guard_ix] = 'X';

            match guard_dir {
                Direction::Up => {
                    if guard_ix < width {
                        break;
                    }
                    if map[guard_ix - width] == '#' {
                        guard_dir = guard_dir.next()
                    } else {
                        guard_ix -= width;
                    }
                }
                Direction::Right => {
                    if guard_ix % width == width - 1 {
                        break;
                    }
                    if map[guard_ix + 1] == '#' {
                        guard_dir = guard_dir.next();
                    } else {
                        guard_ix += 1;
                    }
                }
                Direction::Down => {
                    if guard_ix >= width * (height - 1) {
                        break;
                    }
                    if map[guard_ix + width] == '#' {
                        guard_dir = guard_dir.next();
                    } else {
                        guard_ix += width;
                    }
                }
                Direction::Left => {
                    if guard_ix % width == 0 {
                        break;
                    }
                    if map[guard_ix - 1] == '#' {
                        guard_dir = guard_dir.next();
                    } else {
                        guard_ix -= 1;
                    }
                }
            }
        }
    }

    map.iter().filter(|&&c| c == 'X').count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", predict_guard_exit(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn it_passes_testcase_1() {
        assert_eq!(41, predict_guard_exit(TEST_INPUT));
    }
}
