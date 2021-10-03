use std::fs;

/***********************************************************************
* Structs
************************************************************************/
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


/***********************************************************************
* The actual logic
************************************************************************/
fn build_from_str(all_moves: &str) -> Vec<Pos> {
    let move_str: Vec<&str> = all_moves.trim().split(",").collect();
    let moves: Vec<Move> = move_str.iter().map(|val| Move::from_str(val)).collect();
    make_wire(&moves)
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

fn in_between(a: i32, points: (i32, i32)) -> bool {
    if points.0 < points.1 {
        return points.0 <= a && a <= points.1;
    } else {
        return points.1 <= a && a <= points.0;
    }
}

fn get_intersection(line1: (&Pos, &Pos), line2: (&Pos, &Pos)) -> Option<Pos> {
    let ((a, b), (c, d)) = if line1.0.y == line1.1.y {
        (line1, line2)
    } else {
        (line2, line1)
    };

    if in_between(c.x, (a.x, b.x)) && in_between(a.y, (c.y, d.y)) {
        return Some(Pos{x: c.x, y: a.y});
    }
    None
}

fn get_intersections(wire1: &[Pos], wire2: &[Pos]) -> Vec<Pos> {
    let mut result = Vec::new();
    let origin = Pos{x: 0, y: 0};
    for outer in 0..wire1.len()-1 {
        let line1 = (&wire1[outer], &wire1[outer + 1]);
        for inner in 0..wire2.len()-1 {
            let line2 = (&wire2[inner], &wire2[inner + 1]);
            if let Some(pos) = get_intersection(line1, line2) {
                if pos != origin {
                    result.push(pos);
                }
            }
        }
    }
    result
}

fn get_manhatten_distance(intersections: &[Pos]) -> i32 {
    let mut result = i32::max_value();
    for item in intersections {
        let distance = item.x.abs() + item.y.abs();
        if distance < result {
            result = distance;
        }
    }
    result
}

/***********************************************************************
* Main
************************************************************************/
fn main() {
    let contents = fs::read_to_string("day3.txt").expect("Uh Oh!");
    let contents: Vec<&str> = contents.trim_end().lines().collect();
    let wire1 = build_from_str(contents[0]);
    let wire2 = build_from_str(contents[1]);

    let intersections = get_intersections(&wire1, &wire2);
    let distance = get_manhatten_distance(&intersections);
    println!("{} is the manhatten distance", distance);
}

/***********************************************************************
* Tests
************************************************************************/
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

    #[test]
    fn test_get_intersection() {
        assert_eq!(None, get_intersection((&Pos{x:-1, y:2}, &Pos{x:1, y:2}), (&Pos{x:0, y:-1}, &Pos{x:0, y:1})));
        assert_eq!(Some(Pos{x: 0, y: 0}), get_intersection((&Pos{x:-1, y:0}, &Pos{x:1, y:0}), (&Pos{x:0, y:-1}, &Pos{x:0, y:1})));
    }

    #[test]
    fn test_get_intersections() {
        let wire1 = build_from_str("R8,U5,L5,D3");
        let wire2 = build_from_str("U7,R6,D4,L4");
        let expect = vec![Pos{x: 6, y: 5}, Pos{x: 3, y: 3}];

        assert_eq!(expect, get_intersections(&wire1, &wire2));
    }

    #[test]
    fn test_get_manhatten_distance() {
        let wire1 = build_from_str("R8,U5,L5,D3");
        let wire2 = build_from_str("U7,R6,D4,L4"); 
        let intersections = get_intersections(&wire1, &wire2);

        assert_eq!(6, get_manhatten_distance(&intersections));

        let wire1 = build_from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = build_from_str("U62,R66,U55,R34,D71,R55,D58,R83"); 
        let intersections = get_intersections(&wire1, &wire2);

        assert_eq!(159, get_manhatten_distance(&intersections));

        let wire1 = build_from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = build_from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"); 
        let intersections = get_intersections(&wire1, &wire2);

        assert_eq!(135, get_manhatten_distance(&intersections));
    }
}
