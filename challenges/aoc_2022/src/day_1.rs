#![allow(dead_code)]

fn get_each_calories(lines: &[&str]) -> Vec<i32> {
    lines
        .split(|a| a.is_empty())
        .map(|i| i.iter().map(|v| v.parse::<i32>().unwrap()).sum())
        .collect()
}

fn part_one(lines: &[&str]) -> i32 {
    let all = get_each_calories(lines);
    *all.iter().max().unwrap()
}

fn part_two(lines: &[&str]) -> i32 {
    let mut all = get_each_calories(lines);
    all.sort();
    all.iter().rev().take(3).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(24000, part_one(&lines));
    }

    #[test]
    fn test_part_one_the_run() {
        let input = fs::read_to_string("day_1.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(70116, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(45000, part_two(&lines));
    }

    #[test]
    fn test_part_two_the_run() {
        let input = fs::read_to_string("day_1.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(206582, part_two(&lines));
    }
}
