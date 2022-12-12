fn parse_grid(lines: &[&str]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in lines.iter() {
        result.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    result
}

fn is_visible(grid: &[Vec<i32>], i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1 {
        return true;
    }
    let t = grid[i][j];
    grid.iter().skip(i + 1).all(|v| v[j] < t)
        || grid.iter().rev().skip(grid.len() - i).all(|v| v[j] < t)
        || grid[i].iter().skip(j + 1).all(|v| *v < t)
        || grid[i].iter().rev().skip(grid.len() - j).all(|v| *v < t)
}

fn part_one(lines: &[&str]) -> i32 {
    let grid = parse_grid(lines);
    let mut visible = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_visible(&grid, i, j) {
                visible += 1;
            } else {
            }
        }
    }
    visible
}

fn scenic_score(grid: &[Vec<i32>], i: usize, j: usize) -> i32 {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1 {
        return 0;
    }
    let t = grid[i][j];

    let mut r = grid.iter().skip(i + 1).take_while(|v| v[j] < t).count();
    r = if r >= grid.len() - i - 1 { r } else { r + 1 };

    let mut l = grid.iter().rev().skip(grid.len() - i).take_while(|v| v[j] < t).count();
    l = if l >= i { l } else { l + 1 };

    let mut d = grid[i].iter().skip(j + 1).take_while(|v| **v < t).count();
    d = if d >= grid[i].len() - j - 1 { d } else { d + 1 };

    let mut u = grid[i].iter().rev().skip(grid.len() - j).take_while(|v| **v < t).count();
    u = if u >= j { u } else { u + 1 };

    (r * l * d * u) as i32
}

fn part_two(lines: &[&str]) -> i32 {
    let grid = parse_grid(lines);
    grid.iter()
        .enumerate()
        .map(|o| {
            o.1.iter()
                .enumerate()
                .map(|i| scenic_score(&grid, o.0, i.0))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STR: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(21, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_8.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(1820, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(8, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_8.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(385112, part_two(&lines));
    }
}
