use std::fs;
use std::cmp;

fn get_seat_id(description: &str) -> i32 {
    let mut lower = 0;
    let mut upper = 127;

    let mut row_id = 0;

    for  c in description.chars() {
        if lower == upper {
            row_id = lower;
            lower = 0;
            upper = 7;
        }
        match c {
            'B' | 'R' => {lower = (upper + lower) / 2 + 1;}
            'F' | 'L' => {upper = (upper + lower) / 2;}
            _ => {}
        }
    }
    row_id * 8 + lower
}

fn main() {
    let contents = fs::read_to_string("./day_5.txt")
                            .expect("Unable to read file!");

    let seats: Vec<&str> = contents.split_ascii_whitespace().collect();
    let ids: Vec<i32> = seats.iter().map(|val| get_seat_id(val)).collect();

    let max = ids.iter().fold(0, |max, val| cmp::max(max, *val));
    println!("The max seat number is {}", max);

    let mut my_seat = 0;
    for seat in 1..(max - 1) {
        if !ids.contains(&seat) && ids.contains(&(seat + 1)) && ids.contains(&(seat - 1)) {
            my_seat = seat;
            break;
        }
    }

    println!("My seat number is {}", my_seat);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
    }
}
