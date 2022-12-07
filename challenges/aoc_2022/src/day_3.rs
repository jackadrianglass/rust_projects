#![allow(dead_code)]

fn char_value(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        (c as i32) - 97 + 1
    }
    else if c >= 'A' && c <= 'Z' {
        (c as i32) - 65 + 27
    } else {
        panic!("Unexpected value {}", c)
    }
}

fn part_one(lines: &[&str]) -> i32 {
    lines.iter().map(|l| {
        let (h1, h2) = l.split_at(l.len() / 2);
        let c = h1.chars().find(|c| h2.contains(*c)).unwrap();
        char_value(c)
    }).sum()
}

// part two

fn part_two(lines: &Vec<&str>) -> i32 {
    lines.chunks(3).map(|p| {
        let c = p[0].chars().find(|c| p[1].contains(*c) && p[2].contains(*c)).unwrap();
        char_value(c)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STRING: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!(157, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let contents = fs::read_to_string("day_3.txt").unwrap();
        let lines: Vec<_> = contents.lines().collect();
        assert_eq!(8298, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!(70, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let contents = fs::read_to_string("day_3.txt").unwrap();
        let lines: Vec<_> = contents.lines().collect();
        assert_eq!(2708, part_two(&lines));
    }
}
