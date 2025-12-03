use std::cmp::min;
use std::ops::Range;

advent_of_code::solution!(3);

fn max_in(numbers: &[u8], range: Range<usize>) -> (u8, usize) {
    let mut max = (0, usize::MAX);
    for i in 0..numbers.len() {
        if !range.contains(&i) { continue }
        if numbers[i] > max.0 { max = (numbers[i], i); }
        if max.0 == 9 { break }
    }
    max
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    input
        .lines()
        .for_each(|l| {
            let numbers = l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();
            let (n1, i1) = max_in(&numbers, 0..(numbers.len() - 1));
            let (n2, _i2) = max_in(&numbers, (i1+1)..numbers.len());
            sum += (n1 * 10 + n2) as u64;
        });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    input
        .lines()
        .for_each(|l| {
            let numbers = l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();

            let mut number = 0;
            let mut min_i = None;
            for i in 0..12 {
                let limit = numbers.len() - (12 - i);
                let m = if min_i == None { 0 } else { min_i.unwrap() + 1 };
                let (n, i0) = max_in(&numbers, min(m, limit)..(limit+1));
                number += n as u64 * 10_u64.pow(11 - i as u32);
                min_i = Some(i0);
            }

            sum += number;
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
