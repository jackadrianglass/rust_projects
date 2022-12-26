#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Value {
    Old,
    Number(i64),
}

#[derive(Debug)]
struct Monkey {
    times_inspected: i64,
    items: Vec<i64>,
    operation: Operation,
    value: Value,
    test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_monkeys(lines: &[&str]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut iter = lines.iter().peekable();
    while let Some(val) = iter.next() {
        if !val.starts_with("Monkey") {
            continue;
        }

        let items_line = iter.next().unwrap();
        let op_line = iter.next().unwrap();
        let test_line = iter.next().unwrap();
        let true_line = iter.next().unwrap();
        let false_line = iter.next().unwrap();

        // parse items
        let parts = items_line.split(":").collect::<Vec<_>>();
        let items = parts[1]
            .split(",")
            .map(|v| v.trim())
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        // parse operation
        let parts = op_line.split(" ").collect::<Vec<_>>();
        let operation = match parts[parts.len() - 2] {
            "*" => Operation::Mul,
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "/" => Operation::Div,
            v => panic!("Unknown operation {}", v),
        };

        let value = if parts.last().unwrap() == &"old" {
            Value::Old
        } else {
            Value::Number(parts.last().unwrap().parse().unwrap())
        };

        let test = test_line.split(" ").last().unwrap().parse().unwrap();
        let true_monkey = true_line.split(" ").last().unwrap().parse().unwrap();
        let false_monkey = false_line.split(" ").last().unwrap().parse().unwrap();

        monkeys.push(Monkey {
            times_inspected: 0,
            items,
            operation,
            value,
            test,
            true_monkey,
            false_monkey,
        })
    }

    monkeys
}

fn part_one(lines: &[&str]) -> i64 {
    let mut monkeys = parse_monkeys(lines);
    let mut sending = Vec::new();
    for _ in 0..monkeys.len() {
        sending.push(Vec::new());
    }
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];
            while monkey.items.len() > 0 {
                monkey.times_inspected += 1;
                let mut item = monkey.items.pop().unwrap();
                let val = match monkey.value {
                    Value::Old => item,
                    Value::Number(n) => n,
                };

                item = match monkey.operation {
                    Operation::Add => item + val,
                    Operation::Div => item / val,
                    Operation::Mul => item * val,
                    Operation::Sub => item - val,
                };
                item = item / 3;
                if (item % monkey.test) as i64 == 0 {
                    sending[monkey.true_monkey].push(item);
                } else {
                    sending[monkey.false_monkey].push(item);
                }
            }

            for (idx, queue) in sending.iter_mut().enumerate() {
                while let Some(val) = queue.pop() {
                    monkeys[idx].items.push(val);
                }
            }
        }
    }
    let mut inspect = monkeys
        .iter()
        .map(|m| m.times_inspected)
        .collect::<Vec<_>>();
    inspect.sort();
    inspect.reverse();
    inspect[0] * inspect[1]
}

fn part_two(lines: &[&str]) -> i64 {
    let mut monkeys = parse_monkeys(lines);
    let mut sending = Vec::new();
    for _ in 0..monkeys.len() {
        sending.push(Vec::new());
    }
    let super_mod = monkeys.iter().map(|m| m.test).fold(1, |acc, v| acc * v);


    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];
            while monkey.items.len() > 0 {
                monkey.times_inspected += 1;
                let mut item = monkey.items.pop().unwrap();
                let val = match monkey.value {
                    Value::Old => item,
                    Value::Number(n) => n,
                };

                item = match monkey.operation {
                    Operation::Add => item + val,
                    Operation::Div => item / val,
                    Operation::Mul => item * val,
                    Operation::Sub => item - val,
                };

                item = item % super_mod;

                if item % monkey.test == 0 {
                    sending[monkey.true_monkey].push(item);
                } else {
                    sending[monkey.false_monkey].push(item);
                }
            }

            for (idx, queue) in sending.iter_mut().enumerate() {
                while let Some(val) = queue.pop() {
                    monkeys[idx].items.push(val);
                }
            }
        }
    }

    let mut inspect = monkeys
        .iter()
        .map(|m| m.times_inspected)
        .collect::<Vec<_>>();
    println!("{:?}", inspect);
    inspect.sort();
    inspect.reverse();
    inspect[0] * inspect[1]
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STR: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    #[ignore]
    fn test_part_one() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(10605, part_one(&lines));
    }

    #[test]
    #[ignore]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_11.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(62491, part_one(&lines));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(2713310158, part_two(&lines));
    }

    #[test]
    #[ignore]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_11.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(17408399184, part_two(&lines));
    }
}
