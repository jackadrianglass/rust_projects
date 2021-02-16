use std::time;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum State{
    F,  //floor
    E,  //empty
    O   //occupied
}

fn to_state_mat(contents: &str) -> Vec<Vec<State>>
{
    let mut seats = Vec::new();
    for line in contents.lines() {
        seats.push(line.chars().map(|val|
            match val {
                '.' => {State::F}
                'L' => {State::E}
                '#' => {State::O}
                _ => {panic!("Unexpected character")}
            }
        ).collect());
    }
    seats
}

#[allow(dead_code)]
fn is_swappable(seats: &Vec<Vec<State>>, row: i32, col: i32) -> bool
{
    let val = &seats[col as usize][row as usize];
    match val {
        State::F => {return false;}
        State::E => {
            let width = seats[0].len() as i32;
            let height = seats.len() as i32;
            for i in col-1..col+2{
                if i < 0 || i >= height {
                    continue;
                }
                for j in row-1..row+2{
                    if j < 0 || j >= width {
                        continue;
                    }
                    if seats[i as usize][j as usize] == State::O {
                        return false;
                    }
                }
            }
        }
        State::O => {
            let width = seats[0].len() as i32;
            let height = seats.len() as i32;
            let mut occupied = 0;
            for i in col-1..col+2{
                if i < 0 || i >= height {
                    continue;
                }
                for j in row-1..row+2{
                    if j < 0 || j >= width || (i == col && j == row){
                        continue;
                    }
                    if seats[i as usize][j as usize] == State::O {
                        occupied += 1;
                    }
                }
            }

            if occupied < 4 {
                return false;
            }
        }
    }
    true
}

fn occupied_seats(seats: &Vec<Vec<State>>) -> i32 {
    let mut copy1 = seats.clone();
    let mut copy2 = seats.clone();
    let mut which_copy = 1;
    loop {
        let (check, change) = match which_copy {
            1 => {which_copy = 2; (&mut copy1, &mut copy2)}
            2 => {which_copy = 1; (&mut copy2, &mut copy1)}
            _ => {panic!("This can't happen");}
        };

        let mut changed = false;

        for col in 0..check.len() {
            for row in 0..check[0].len() {
                if is_swappable2(check, row as i32, col as i32) {
                    // println!("[{}, {}]", col, row);
                    changed = true;
                    change[col][row] = match check[col][row] {
                        State::E => {State::O}
                        State::O => {State::E}
                        State::F => {panic!("Definitely shouldn't happen");}
                    }
                }
                else {
                    change[col][row] = check[col][row];
                }
            }
        }

        // for row in check {
        //     println!("{:?}", row);
        // }
        // println!("------------------");
        if !changed {
            break;
        }
    }
    let mut sum = 0;
    for row in copy1 {
        sum += row.iter().fold(0, |sum, val| if *val == State::O {sum + 1}else{sum});
    }
    sum
}

// Part 2

#[derive(Debug)]
struct Idx {
    row: i32,
    col: i32
}

fn o_in_dir(seats: &Vec<Vec<State>>, pos: Idx, dir: Idx) -> bool {
    let height = seats.len() as i32;
    let width = seats[0].len() as i32;

    let mut idx = Idx{row: pos.row + dir.row, col: pos.col + dir.col};
    loop{
        if idx.row < 0 || idx.row >= width || idx.col < 0 || idx.col >= height {
            break;
        }
        match seats[idx.col as usize][idx.row as usize] {
            State::O => {return true;}
            State::E => {return false;}
            _ => {}
        }

        idx.row += dir.row;
        idx.col += dir.col;
    }
    false
}

fn is_swappable2(seats: &Vec<Vec<State>>, row: i32, col: i32) -> bool
{
    let val = &seats[col as usize][row as usize];
    match val {
        State::F => {return false;}
        State::E => {
            for i in -1..2{
                for j in -1..2{
                    if j == 0 && i == 0 {
                        continue;
                    }
                    if o_in_dir(&seats, Idx{row: row, col: col}, Idx{row: j, col: i}) {
                        return false;
                    }
                }
            }
        }
        State::O => {
            let mut occupied = 0;
            for i in -1..2{
                for j in -1..2{
                    if j == 0 && i == 0 {
                        continue;
                    }
                    if o_in_dir(&seats, Idx{row: row, col: col}, Idx{row: j, col: i}) {
                        occupied += 1;
                    }
                }
            }

            if occupied < 5 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let contents = fs::read_to_string("day_11.txt")
                    .expect("Unable to read file");

    let now = time::Instant::now();
    let seats = to_state_mat(&contents);
    let count = occupied_seats(&seats);
    println!("{:?} took {:?}", count, now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let contents = "#.#\nL.#\nLLL";
        let expect = vec![
            vec![State::O, State::F, State::O],
            vec![State::E, State::F, State::O],
            vec![State::E, State::E, State::E],
        ];

        assert_eq!(to_state_mat(&contents), expect);
    }

    #[test]
    fn test_is_swappable() {
        let contents = r"#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.LL.L#
#.##.LL.LL
#.##LLL.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
        let input = to_state_mat(&contents);

        assert!(!is_swappable(&input, 0, 0));
        assert!(!is_swappable(&input, 0, 1));
        assert!(is_swappable(&input, 2, 1));
        assert!(is_swappable(&input, 5, 4));
    }

    #[test]
    fn test_stabilized_count(){
        let contents = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let input = to_state_mat(&contents);

        assert_eq!(occupied_seats(&input), 26);
    }

    #[test]
    fn test_o_in_dir() {
        let contents = r"#.##.L#.##
#L###LL.L#
#.L.L..L..
#L##.LL.L#
#.##.LL.LL
#.##LLL.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##";
        let input = to_state_mat(&contents);

        assert!(!o_in_dir(&input, Idx{row: 0, col:2}, Idx{row: 1, col: 0}));
        assert!(o_in_dir(&input, Idx{row: 0, col:1}, Idx{row: 1, col: 1}));
    }


}
