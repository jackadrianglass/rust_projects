use std::collections::HashSet;

fn update(head: &(i32, i32), tail: &mut(i32, i32)) {
    let x_diff: i32 = head.0 - tail.0;
    let y_diff: i32 = head.1 - tail.1;
    if x_diff.abs() > 1 && y_diff.abs() > 1 {
        if x_diff.is_positive() {
            tail.0 = head.0 - 1;
        } else {
            tail.0 = head.0 + 1;
        }

        if y_diff.is_positive() {
            tail.1 = head.1 - 1;
        } else {
            tail.1 = head.1 + 1;
        }
    } else if x_diff.abs() > 1 {
        if x_diff.is_positive() {
            tail.0 = head.0 - 1;
        } else {
            tail.0 = head.0 + 1;
        }
        if head.1 != tail.1 {
            tail.1 = head.1;
        }
    } else if y_diff.abs() > 1 {
        if y_diff.is_positive() {
            tail.1 = head.1 - 1;
        } else {
            tail.1 = head.1 + 1;
        }
        if head.0 != tail.0 {
            tail.0 = head.0;
        }
    }
}

fn part_one(lines: &[&str]) -> i32 {
    let mut positions = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    positions.insert((0, 0));

    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<_>>();
        for _ in 0..parts[1].parse().unwrap() {
            match parts[0] {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                i => panic!("Unknown direction {}", i),
            }
            update(&head, &mut tail);
            positions.insert(tail);
        }
    }
    positions.len() as i32
}

fn part_two(lines: &[&str]) -> i32 {
    let mut unique_pos = HashSet::new();
    let mut snake = Vec::new();
    for _ in 0..10 {
        snake.push((0, 0));
    }
    unique_pos.insert((0, 0));

    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<_>>();
        for _ in 0..parts[1].parse().unwrap() {
            match parts[0] {
                "R" => snake[0].0 += 1,
                "L" => snake[0].0 -= 1,
                "U" => snake[0].1 += 1,
                "D" => snake[0].1 -= 1,
                i => panic!("Unknown direction {}", i),
            }
            for idx in 0..snake.len() - 1 {
                update(&snake[idx].clone(), &mut snake[idx+1]);
            }
            unique_pos.insert(*snake.last().unwrap());
        }
        //println!("{line}");
        //println!("{snake:?}");
    }
    unique_pos.len() as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STR: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_STR_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_update() {
        let mut values = vec![
            // no movement cases
            ((0, 0), (0, 0), (0, 0)),
            ((1, 0), (0, 0), (0, 0)),
            ((0, -1), (0, 0), (0, 0)),

            // one dimension
            ((-2, 0), (0, 0), (-1, 0)),
            ((0, 2), (0, 0), (0, 1)),

            // diagonals
            ((1, 2), (0, 0), (1, 1)),
            ((-2, 1), (0, 0), (-1, 1)),
            ((2, 2), (0, 0), (1, 1)),
        ];
        for (head, mut tail, expect) in values.iter_mut() {
            update(&head, &mut tail);
            assert_eq!(tail, *expect);
        }
    }

    #[test]
    fn test_part_one() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(13, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_9.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(5858, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines = TEST_STR_2.lines().collect::<Vec<_>>();
        assert_eq!(36, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        // TODO number is too low. Something is going wrong with the update
        let content = fs::read_to_string("day_9.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(2602, part_two(&lines));
    }
}
