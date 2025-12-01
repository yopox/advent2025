use std::cmp::min;

advent_of_code::solution!(1);

const STARTING: u64 = 50;

fn parse_line(line: &str) -> (bool, u64) {
    let (left, number) = line.split_at(1);
    (left == "L", number.parse::<u64>().ok().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = STARTING;
    let mut zeroes = 0;

    for line in input.lines() {
        let (left, number) = parse_line(line);
        current = if left { current + 100 - number % 100 } else { current + number } % 100;
        if current == 0 { zeroes += 1 }
    }
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current = STARTING;
    let mut zeroes = 0;

    for line in input.lines() {
        let (left, mut number) = parse_line(line);

        while number > 0 {
            let temp = min(number, 99);
            if left {
                if current != 0 && current <= temp { zeroes += 1; }
                current = (current + 100 - temp) % 100
            } else {
                if current + temp >= 100 { zeroes += 1; }
                current = (current + temp) % 100
            }
            number = number - temp;
        }
    }
    Some(zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
