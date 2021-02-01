use std::fs;

fn first_not_sum(container: &[i64], preamble: usize) -> i64 {
    for idx in preamble..container.len() {
        let val = &container[idx];
        let mut found_pair = false;
        for lower in idx-preamble..idx {
            let lower_val = &container[lower];
            for upper in lower + 1..idx {
                let upper_val = &container[upper];
                if lower_val + upper_val == *val {
                    found_pair = true;
                    break;
                }
            }
            if found_pair {break;}
        }
        if !found_pair {return *val;}
    }
    panic!("unable to find value");
}

fn contiguous_sum_of(container: &[i64], sum_val: i64) -> i64 {
    for idx in 0..container.len() {
        let mut lowest = container[idx];
        let mut highest = container[idx];
        let mut sum = container[idx];
        for upper in idx+1..container.len() {
            let val = container[upper];
            sum += val;
            if val < lowest {
                lowest = val;
            } else if val > highest {
                highest = val;
            }

            if sum == sum_val {
                return lowest + highest;
            } else if sum > sum_val {
                break;
            }
        }
    }
    panic!("Value not found");
}

fn main() {
    let contents = fs::read_to_string("day_9.txt")
                    .expect("Unable to read input file");

    let input: Vec<i64> = contents.lines().map(|val| val.parse().unwrap()).collect();
    let first = first_not_sum(&input, 25);
    println!("First that doesn't match the rule is {}", first);

    let continguous = contiguous_sum_of(&input, first);
    println!("The encryption weakness is {}", continguous);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let input = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
                                102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(first_not_sum(&input, 5), 127);
    }

    #[test]
    fn test_contiguous() {
        let input = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95,
                                102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(contiguous_sum_of(&input, 127), 62);
    }
}
