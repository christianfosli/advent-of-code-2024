use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let (l, r) = input
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut acc_l, mut acc_r), line| {
            let (el_l, el_r) = line.split_once("   ").unwrap();
            acc_l.push(el_l.parse::<usize>().unwrap());
            acc_r.push(el_r.parse::<usize>().unwrap());
            (acc_l, acc_r)
        });
    println!("Part 1: {}", part_1_distance(&l, &r));
    println!("Part 2: {}", part_2_similarity_score(&l, &r));
    Ok(())
}

fn part_1_distance(l: &[usize], r: &[usize]) -> usize {
    let mut l = l.to_vec();
    l.sort();
    let mut r = r.to_vec();
    r.sort();

    r.iter()
        .zip(l.iter())
        .map(|(&el_r, &el_l)| (el_r as isize - el_l as isize).unsigned_abs())
        .sum()
}

fn part_2_similarity_score(l: &[usize], r: &[usize]) -> usize {
    l.iter().copied().fold(0, |acc, el| {
        acc + el * r.iter().filter(|&&r_el| r_el == el).count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_testcase_1() {
        let l = vec![3, 4, 2, 1, 3, 3];
        let r = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(11, part_1_distance(&l, &r));
    }

    #[test]
    fn it_passes_testcase_2() {
        let l = vec![3, 4, 2, 1, 3, 3];
        let r = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(31, part_2_similarity_score(&l, &r));
    }
}
