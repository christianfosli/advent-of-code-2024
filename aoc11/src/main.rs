trait UsizeExt {
    fn num_digits(&self) -> u32;
}

impl UsizeExt for usize {
    fn num_digits(&self) -> u32 {
        self.checked_ilog10().unwrap_or(0) + 1
    }
}

fn transform_stones_1(stones: &[usize], num_blinks: u32) -> Vec<usize> {
    let transformed = stones
        .into_iter()
        .flat_map(|&n| match n {
            0 => vec![1],
            n if n.num_digits() % 2 == 0 => {
                let num_digits = n.num_digits();
                let left = n / 10usize.pow(num_digits / 2);
                let right = n % 10usize.pow(num_digits / 2);
                vec![left, right]
            }
            _ => vec![n * 2024],
        })
        .collect::<Vec<_>>();

    if num_blinks == 1 {
        transformed
    } else {
        transform_stones_1(&transformed, num_blinks - 1)
    }
}

fn main() {
    let input = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188];
    println!("Part 1: {}", transform_stones_1(&input, 25).len());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: [usize; 2] = [125, 17];

    #[test]
    fn it_passes_testcase_1() {
        assert_eq!(vec![253000, 1, 7], transform_stones_1(&TEST_INPUT, 1));

        assert_eq!(
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ],
            transform_stones_1(&TEST_INPUT, 6)
        );

        assert_eq!(55312, transform_stones_1(&TEST_INPUT, 25).len());
    }
}
