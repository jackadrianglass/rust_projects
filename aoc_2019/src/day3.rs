#[derive(Debug, PartialEq, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Clone for Pos {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn from_char(value: char) -> Self {
        match value {
            'U' => Dir::Up,
            'R' => Dir::Right,
            'D' => Dir::Down,
            'L' => Dir::Left,
            _ => panic!("Unexpected character {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    dir: Dir,
    amount: i32,
}

impl Move {
    fn from_str(value: &str) -> Self {
        Self {
            dir: Dir::from_char(value.chars().nth(0).unwrap()),
            amount: value[1..].parse().unwrap(),
        }
    }
}

fn make_wire(moves: &[Move]) -> Vec<Pos> {
    let mut result = vec![Pos { x: 0, y: 0 }];
    let mut current = Pos { x: 0, y: 0 };
    for m in moves {
        match m.dir {
            Dir::Up => current.y += m.amount,
            Dir::Right => current.x += m.amount,
            Dir::Down => current.y -= m.amount,
            Dir::Left => current.x -= m.amount,
        }
        result.push(current.clone());
    }
    result
}

fn main() {
    println!("Hello bitches!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moves() {
        let input = ["R8", "U5", "L5", "D3"];
        let expected = vec![
            Move {
                dir: Dir::Right,
                amount: 8,
            },
            Move {
                dir: Dir::Up,
                amount: 5,
            },
            Move {
                dir: Dir::Left,
                amount: 5,
            },
            Move {
                dir: Dir::Down,
                amount: 3,
            },
        ];
        let result: Vec<Move> = input.iter().map(|val| Move::from_str(val)).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_wire() {
        let input = ["R8", "U5", "L5", "D3"];
        let input: Vec<Move> = input.iter().map(|val| Move::from_str(val)).collect();
        let expect = vec![
            Pos { x: 0, y: 0 },
            Pos { x: 8, y: 0 },
            Pos { x: 8, y: 5 },
            Pos { x: 3, y: 5 },
            Pos { x: 3, y: 2 },
        ];
        assert_eq!(expect, make_wire(&input));
    }
}
