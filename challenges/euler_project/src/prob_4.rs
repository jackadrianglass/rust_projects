/*
Problem 4
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn main() {
    println!("Largest 2 digit {}", largest_palindrom(2));
    println!("Largest 3 digit {}", largest_palindrom(3));
    println!("Largest 4 digit {}", largest_palindrom(4));
}

fn largest_palindrom(num_digits: u32) -> i32 {
    let max_num: i32 = 10i32.pow(num_digits);
    let min_num: i32 = 10i32.pow(num_digits - 1);
    let mut max = 0;

    for x in (min_num..max_num).rev() {
        for y in (min_num..max_num).rev() {
            let prod = x * y;
            if prod < max {
                break;
            }
            if is_palindrom(prod) {
                max = prod;
            }
        }
    }

    max
}

fn is_palindrom(num: i32) -> bool {
    let num: String = num.to_string();
    let rnum: String = num.chars().rev().collect();
    num == rnum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrom() {
        assert!(true == is_palindrom(909));
    }

    #[test]
    fn test_is_not_palindrom() {
        assert!(false == is_palindrom(1234));
    }

    #[test]
    fn test_largest_2_digit_prod_palin() {
        assert_eq!(9009, largest_palindrom(2));
    }
}
