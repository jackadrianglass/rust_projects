#![allow(dead_code)]

fn contains(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    first[0] <= second[0] && first[1] >= second[1]
}

fn part_one(lines: &[&str]) -> i32 {
    lines.iter().map(|l| {
        l.split(",").map(|p| p.split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
    }).map(|l| if contains(&l[0], &l[1]) || contains(&l[1], &l[0]) { 1 } else { 0 }).sum()
}

fn overlaps(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    (a[0] >= b[0] && a[0] <= b[1])
    || (b[0] >= a[0] && b[0] <= a[1])
}

fn part_two(lines: &[&str]) -> i32 {
    lines.iter().map(|l| {
        l.split(",").map(|p| p.split("-").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
    }).map(|l| if overlaps(&l[0], &l[1]) { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STRING: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!(2, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_4.txt").unwrap();
        let lines: Vec<_> = content.lines().collect();
        assert_eq!(513, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!(4, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_4.txt").unwrap();
        let lines: Vec<_> = content.lines().collect();
        assert_eq!(878, part_two(&lines));
    }
}
