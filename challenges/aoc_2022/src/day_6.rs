fn find_start(line: &str, window_size: usize) -> usize {
    let chars = line.chars().collect::<Vec<char>>();
    for (idx, slice) in chars.windows(window_size).enumerate() {
        let mut unique = true;
        for (idx, c) in slice.iter().enumerate() {
            if slice.iter().skip(idx + 1).any(|v| v == c) {
                unique = false;
                break;
            }
        };
        if unique { return idx + window_size; }
    }
    0
}

fn part_one(line: &str) -> usize {
    find_start(line, 4)
}

fn part_two(line: &str) -> usize {
    find_start(line, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        assert_eq!(7, part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_one("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_6.txt").unwrap();
        assert_eq!(1920, part_one(&content));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(19, part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part_two("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_6.txt").unwrap();
        assert_eq!(2334, part_two(&content));
    }
}
