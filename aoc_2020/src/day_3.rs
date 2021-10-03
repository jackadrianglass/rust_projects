use std::fs;

struct Slope {
    right: usize,
    down: usize
}

fn num_trees_hit(slope: &Slope, map: &Vec<&str>) -> i64 {
    let mut trees_hit = 0;
    let mut line = 0;
    let mut idx = 0;
    while line < map.len() {
        if idx >= map[line].len(){
            idx -= map[line].len();
        }

        if map[line].chars().nth(idx).unwrap() == '#' {
            trees_hit += 1;
        }

        idx += slope.right;
        line += slope.down;
    }
    trees_hit
}

fn main() {
    println!("Reading list.txt");

    let contents = fs::read_to_string("day_3.txt")
                    .expect("Something went wrong reading the file");

    let map: Vec<&str> = contents.lines().collect();
    let slopes = [
        Slope{right: 1, down: 1},
        Slope{right: 3, down: 1},
        Slope{right: 5, down: 1},
        Slope{right: 7, down: 1},
        Slope{right: 1, down: 2},
    ];

    let mut result = Vec::new();

    for slope in slopes.iter() {
        result.push(num_trees_hit(&slope, &map));
    }

    println!("Each number of tree hit is {:?}", result);

    let answer = result.iter().fold(1, |prod, val| prod * val);
    println!("The result is {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_trees_hit() {
        let trees = vec![
            "..##....",
            "#...#...",
            ".#....#."
        ];

        let slope = Slope{right: 3, down: 1};
        assert_eq!(num_trees_hit(&slope, &trees), 1);
    }
}
