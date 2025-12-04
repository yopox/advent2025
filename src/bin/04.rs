use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> HashSet<(i16, i16)> {
    let mut set = HashSet::new();
    input.lines().enumerate()
        .for_each(|(y, line)| for (x, char) in line.chars().enumerate() {
            if char == '@' {
                set.insert((x as i16, y as i16));
            }

        });
    set
}

fn adjacent_4(x: i16, y: i16, set: &HashSet<(i16, i16)>) -> bool {
    let mut adjacent = 0;
    let pos = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ];

    for p in pos {
        if set.contains(&p) { adjacent += 1; }
        if adjacent == 4 { return true }
    }

    adjacent >= 4
}

pub fn part_one(input: &str) -> Option<usize> {
    let rolls = parse_input(input);
    Some(rolls.iter().filter(|pos| !adjacent_4(pos.0, pos.1, &rolls)).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rolls = parse_input(input);
    let mut removed = 0;
    let mut removed_some = true;

    while removed_some {
        let to_remove = rolls.iter()
            .filter(|pos| !adjacent_4(pos.0, pos.1, &rolls))
            .map(|pos| *pos)
            .collect::<Vec<(i16, i16)>>();

        removed += to_remove.len();
        removed_some = !to_remove.is_empty();

        to_remove.iter().for_each(|pos| { rolls.remove(pos); });
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
