#![allow(dead_code)]

fn parse_stacks(lines: &[&str]) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for line in lines.iter() {
        if line.is_empty() || line.contains('1') {
            break;
        }
        for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
            if idx + 1 > stacks.len() {
                stacks.push(Vec::new());
            }
            if c == ' ' {
                continue;
            } else {
                stacks[idx].insert(0, c);
            }
        }
    }
    stacks
}

fn parse_instructions(lines: &[&str]) -> Vec<(usize, usize, usize)> {
    let raw_inst = lines.split(|l| l.is_empty()).skip(1).next().unwrap();
    raw_inst
        .iter()
        .map(|l| l.split(' '))
        .map(|mut v| {
            let _ = v.next();
            let a = v.next().unwrap().parse().unwrap();
            let _ = v.next();
            let b: usize = v.next().unwrap().parse().unwrap();
            let _ = v.next();
            let c: usize = v.next().unwrap().parse().unwrap();
            (a, b - 1, c - 1)
        })
        .collect()
}

fn part_one(lines: &[&str]) -> String {
    let mut stacks = parse_stacks(lines);
    let insts = parse_instructions(lines);

    for (quantity, from, to) in insts.iter() {
        for _ in 0..*quantity {
            let item = stacks[*from].pop().unwrap();
            stacks[*to].push(item);
        }
    }
    stacks.iter().fold(String::new(), |acc, v| {
        format!("{}{}", acc, v.last().unwrap())
    })
}


fn part_two(lines: &[&str]) -> String {
    let mut stacks = parse_stacks(lines);
    let insts = parse_instructions(lines);

    for (quantity, from, to) in insts.iter() {
        let mut tmp = Vec::with_capacity(*quantity);
        for _ in 0..*quantity {
            tmp.push(stacks[*from].pop().unwrap());
        }
        stacks[*to].extend(tmp.iter().rev());
    }
    stacks.iter().fold(String::new(), |acc, v| {
        format!("{}{}", acc, v.last().unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STRING: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_stack() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(expected, parse_stacks(&lines));
    }

    #[test]
    fn test_parse_instructions() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        let expected = vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)];
        assert_eq!(expected, parse_instructions(&lines));
    }

    #[test]
    fn test_part_one() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!("CMZ", part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_5.txt").unwrap();
        let lines: Vec<_> = content.lines().collect();
        assert_eq!("FWSHSPJWM", part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines: Vec<_> = TEST_STRING.lines().collect();
        assert_eq!("MCD", part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_5.txt").unwrap();
        let lines: Vec<_> = content.lines().collect();
        assert_eq!("PWPWHGFZS", part_two(&lines));
    }
}
