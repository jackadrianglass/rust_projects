fn parse_cycles(lines: &[&str]) -> Vec<i32> {
    let mut x_reg = 1;
    let mut values = Vec::with_capacity(350);
    values.push(x_reg);

    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "noop" => values.push(x_reg),
            "addx" => {
                let value: i32 = parts[1].parse().unwrap();
                values.push(x_reg);
                values.push(x_reg);
                x_reg += value;
            }
            v => panic!("Unknown instruction {}", v),
        }
    }
    values
}

fn part_one(lines: &[&str]) -> i32 {
    let values = parse_cycles(lines);
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|v| v * values[*v as usize])
        .sum()
}

fn part_two(lines: &[&str]) -> String {
    let values = parse_cycles(lines);
    let mut result = values[1..]
        .chunks(40)
        .map(|c| {
            c.iter()
                .enumerate()
                .map(|v| {
                    if (v.1 - v.0 as i32).abs() > 1 {
                        "."
                    } else {
                        "#"
                    }
                })
                .collect::<String>()
        })
        .fold(String::new(), |acc, v| acc + &v + "\n");
    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TINY_TEST_STR: &str = "noop
addx 3
addx -5";

    const TEST_STR: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_one() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(13140, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_10.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(12740, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(expected, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_10.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        let expected = "###..###..###...##..###...##...##..####.
#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....
#..#.###..#..#.#..#.#..#.#..#.#....###..
###..#..#.###..####.###..####.#.##.#....
#.#..#..#.#....#..#.#.#..#..#.#..#.#....
#..#.###..#....#..#.#..#.#..#..###.#....";

        assert_eq!(expected, part_two(&lines));
    }
}
