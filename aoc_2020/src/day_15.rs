use std::collections::HashMap;

fn remember_game(init: &[usize], stop_idx: usize) -> usize {
    let mut numbers = HashMap::new();
    let mut last_num = 0;

    for idx in 1..stop_idx {
        if idx <= init.len() {
            last_num = init[idx-1];
        }
        match numbers.get_mut(&last_num) {
            Some(val) => {
                last_num = idx - *val;
                *val = idx;
            }
            None => {
                numbers.insert(last_num, idx);
                last_num = 0;
            }
        }
    }
    last_num
}

fn main() {
    let last = remember_game(&[8, 0, 17, 4, 1, 12], 2020);
    println!("The last number is {}", last);

    let last = remember_game(&[8, 0, 17, 4, 1, 12], 30000000);
    println!("The last number is {}", last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remebering_game() {
        assert_eq!(remember_game(&[0, 3, 6], 10), 0);
        assert_eq!(remember_game(&[0, 3, 6], 2020), 436);
        assert_eq!(remember_game(&[1, 3, 2], 2020), 1);
        assert_eq!(remember_game(&[2, 1, 3], 2020), 10);
        assert_eq!(remember_game(&[1, 2, 3], 2020), 27);
        assert_eq!(remember_game(&[2, 3, 1], 2020), 78);
        assert_eq!(remember_game(&[3, 2, 1], 2020), 438);
        assert_eq!(remember_game(&[3, 1, 2], 2020), 1836);
    }
}
