use std::fs;

fn find2020(numbers: &[i32]) -> i32 {
    for outer in 0..numbers.len() {
        for inner in outer+1..numbers.len() {
            if numbers[outer] + numbers[inner] == 2020 {
                return numbers[outer] * numbers[inner];
            }
        }
    }
    return 0;
}

fn find2020_3num(numbers: &[i32]) -> i32 {
    for out in 0..numbers.len() - 2 {
        for mid in out+1..numbers.len() - 1 {
            for inner in mid+1..numbers.len() {
                if numbers[out] + numbers[mid] + numbers[inner] == 2020 {
                    return numbers[out] * numbers[mid] * numbers[inner];
                }
            }
        }
    }
    return 0;
}

fn main() {
    println!("Reading list.txt");

    let contents = fs::read_to_string("day_1.txt")
                    .expect("Something went wrong reading the file");

    let mut nums = Vec::new();

    for line in contents.lines() {
        nums.push(line.parse::<i32>().unwrap());
    }

    let result = find2020(&nums);
    println!("The result for 2 numbers is {}", result);

    let result = find2020_3num(&nums);
    println!("The result for 3 numbers is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_list() {
        let list = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find2020(&list), 514579);
    }

    #[test]
    fn test_small_list_for_3() {
        let list = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find2020_3num(&list), 241861950);
    }
}
