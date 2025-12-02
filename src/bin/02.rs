advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .filter_map(|range| range.split_once("-"))
        .filter_map(|(a, b)| {
            Some((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?))
        })
        .collect()
}

fn invalid_in_range(a: u64, b: u64) -> Vec<u64> {
    let mut invalid: Vec<u64> = vec![];
    for n in a..=b {
        let s = n.to_string();
        if s.len() % 2 == 0 {
            let (p1, p2) = s.split_at(s.len() / 2);
            if p1 == p2 { invalid.push(n); }
        }
    }
    invalid
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid: Vec<u64> = vec![];
    for range in ranges {
        invalid.append(&mut invalid_in_range(range.0, range.1));
    }
    Some(invalid.into_iter().sum())
}

fn invalid_in_range_2(a: u64, b: u64) -> Vec<u64> {
    let mut invalid: Vec<u64> = vec![];
    'main: for n in a..=b {
        let s = n.to_string();
        let len = s.len();

        for m in 1..=(len / 2) {
            if len % m != 0 { continue }
            let (pat, mut s2) = s.split_at(m);
            while s2.starts_with(&pat) {
                s2 = s2.strip_prefix(&pat).unwrap();
            }
            if s2.is_empty() {
                invalid.push(n);
                continue 'main;
            }
        }
    }
    invalid
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid: Vec<u64> = vec![];
    for range in ranges {
        invalid.append(&mut invalid_in_range_2(range.0, range.1));
    }
    Some(invalid.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
