#![allow(dead_code)]

enum Hand {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Tie,
    Lose,
}

// part one

fn outcome_pts_1(round: &(Hand, Hand)) -> i32 {
    match round {
        (Hand::Rock, Hand::Paper) | (Hand::Paper, Hand::Scissors) | (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissors, Hand::Scissors) => 3,
        (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper) | (Hand::Rock, Hand::Scissors) => 0,
    }
}

fn shape_pts_1(hand: &Hand) -> i32 {
    match *hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn rounds_1(lines: &[& str]) -> Vec<(Hand, Hand)> {
    let mut result = Vec::new();
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent = match parts[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Unknown opponent hand {}", parts[0])
        };

        let mine = match parts[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Unknown my hand {}", parts[1])
        };
        result.push((opponent, mine));
    }
    result
}

fn part_one(lines: &[&str]) -> i32 {
    let rounds = rounds_1(lines);
    rounds.iter().map(|r| outcome_pts_1(r) + shape_pts_1(&r.1)).sum()
}

// part two

fn rounds_2(lines: &[& str]) -> Vec<(Hand, Outcome)> {
    let mut result = Vec::new();
    for line in lines.iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        let opponent = match parts[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Unknown opponent hand {}", parts[0])
        };

        let outcome = match parts[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Unknown my hand {}", parts[1])
        };
        result.push((opponent, outcome));
    }
    result
}

fn shape_pts2(round: &(Hand, Outcome)) -> i32 {
    match round {
        (Hand::Rock, Outcome::Tie) | (Hand::Paper, Outcome::Lose) | (Hand::Scissors, Outcome::Win) => 1,
        (Hand::Paper, Outcome::Tie) | (Hand::Scissors, Outcome::Lose) | (Hand::Rock, Outcome::Win) => 2,
        (Hand::Scissors, Outcome::Tie) | (Hand::Rock, Outcome::Lose) | (Hand::Paper, Outcome::Win) => 3,
    }
}

fn outcome_pts2(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Tie => 3,
        Outcome::Win => 6,
    }
}

fn part_two(lines: &[&str]) -> i32 {
    let rounds = rounds_2(lines);
    rounds.iter().map(|r| outcome_pts2(&r.1) + shape_pts2(&r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = "A Y
B X
C Z
";
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(15, part_one(&lines));
    }

    #[test]
    fn part_one_run() {
        let input = fs::read_to_string("day_2.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(12772, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let input = "A Y
B X
C Z
";
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(12, part_two(&lines));
    }


    #[test]
    fn part_two_run() {
        let input = fs::read_to_string("day_2.txt").unwrap();
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(11618, part_two(&lines));
    }
}
