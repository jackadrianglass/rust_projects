use std::fs;

fn use_all_adapters(values: &[i32]) -> i32 {
    let mut prev = values[0];
    let mut diff_by_3 = 1; //always have this 3 above highest
    let mut diff_by_1 = 0;

    for curr in &values[..] {
        let diff = curr - prev;
        if diff == 3 {
            diff_by_3 += 1;
        }
        else if diff == 1 {
            diff_by_1 += 1;
        }
        else if diff > 3 || diff < 0 {
            panic!("Invalid diff. Make sure list is sorted");
        }

        prev = *curr;
    }
    diff_by_1 * diff_by_3
}

fn count_permutations(values: &[i32]) -> i64 {
    let mut combinations = Vec::with_capacity(values.len());
    for idx in 0..values.len() {
        let val = values[idx];
        let mut comb = 0;
        for upper in idx + 1..values.len() {
            let diff = values[upper] - val;
            if diff < 0 {panic!("values needs to be sorted");}
            else if diff <= 3 {comb += 1;}
            else {break;}
        }
        if comb == 0 {continue;}
        combinations.push(comb);
        // println!("idx:{} val:{} comb:{}'", idx, val, comb);
    }
    combinations.push(1);

    // println!("\nStart lookup");
    let mut lookup = vec![1; combinations.len()];
    for idx in (0..combinations.len() - 2).rev() {
        let comb = combinations[idx];
        match comb {
            1 => {lookup[idx] = lookup[idx + 1];}
            2 => {lookup[idx] = lookup[idx + 1] + lookup[idx + 2];}
            3 => {lookup[idx] = lookup[idx + 1] + lookup[idx + 2] + lookup[idx + 3];}
            value => {panic!("Unexpected value idx{}: {}", idx, value);}
        }
        // println!("idx:{} comb:{} lookup:{}", idx, comb, lookup[idx]);
    }
    lookup[0]
}

fn main() {
    let contents = fs::read_to_string("day_10.txt")
                    .expect("Unable to read file");
    println!("Hello World!");
    let mut values: Vec<i32> = contents.lines().map(|val| val.parse().unwrap()).collect();
    values.push(0);
    values.sort();

    let prod = use_all_adapters(&values);
    println!("The product of the 1 diffs and 3 diffs is {}", prod);

    let comb = count_permutations(&values);
    println!("The number of combination is {}", comb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_all_adaptors() {
        let input = [0, 1, 2, 5];
        assert_eq!(use_all_adapters(&input), 4);
    }

    #[test]
    fn test_short_all_adaptors() {
        let mut input = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(use_all_adapters(&input), 35);
    }

    #[test]
    fn test_long_all_adaptors() {
        let mut input = vec![0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24,
                            23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
                            8, 17, 7, 9, 4, 2, 34, 10, 3];
        input.sort();
        assert_eq!(use_all_adapters(&input), 220);
    }

    #[test]
    fn test_basic_comb() {
        let input = [0, 1, 2, 5];
        assert_eq!(count_permutations(&input), 2);
    }

    #[test]
    fn test_other_basic_comb() {
        let input = [0, 1, 2, 3, 4, 5];
        assert_eq!(count_permutations(&input), 13);
    }

    #[test]
    fn test_short_comb() {
        let mut input = vec![0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(count_permutations(&input), 8);
    }

    #[test]
    fn test_long_comb() {
        let mut input = vec![0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24,
                            23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
                            8, 17, 7, 9, 4, 2, 34, 10, 3];
        input.sort();
        assert_eq!(count_permutations(&input), 19208);
    }
}
