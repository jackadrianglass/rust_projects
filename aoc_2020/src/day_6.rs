use std::fs;
use std::collections::HashMap;

fn increment_rsp(answers: &mut HashMap<char, i32>, rsp: &str) {
    let mut to_add: Vec<char> = rsp.chars().collect();
    to_add.sort();
    to_add.dedup();

    for c in to_add {
        if !c.is_alphanumeric() { continue; }
        match answers.get_mut(&c) {
            Some(val) => {*val += 1;}
            None => {answers.insert(c, 1);}
        }
    }
}

fn increment_by_consensus(answers: &mut HashMap<char,i32>, rsp: &str) {
    let lines: Vec<&str> = rsp.lines().collect();
    let num_users = lines.len();

    let to_add: Vec<char> = rsp.chars().collect();
    let mut map = HashMap::new();

    for c in to_add {
        if !c.is_alphanumeric() {continue;}
        match map.get_mut(&c) {
            Some(val) => {*val += 1;}
            None => {map.insert(c, 1);}
        }       
    }

    for (key, val) in map.iter() {
        if *val != num_users { continue; }
        match answers.get_mut(&key) {
            Some(val) => {*val += 1;}
            None => {answers.insert(*key, 1);}
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day_6.txt")
        .expect("Something went wrong reading the file");

    let passport_str: Vec<&str> = contents.split("\r\n\r\n").collect();
    let mut answers = HashMap::new();

    passport_str.iter().for_each(|val| increment_rsp(&mut answers, val));
    let sum = answers.values().fold(0, |sum, val| sum + val);
    println!("Sum of all the answers is {}", sum);

    let mut consensus = HashMap::new();
    passport_str.iter().for_each(|val| increment_by_consensus(&mut consensus, val));
    let sum = consensus.values().fold(0, |sum, val| sum + val);
    println!("Sum of all the consensus answers is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_rsp() {
        let mut map = HashMap::new();
        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 1);
        expected.insert('c', 1);

        increment_rsp(&mut map, "aabbcc");
        assert_eq!(map, expected);
    }

    #[test]
    fn test_increment_rsp1() {
        let mut map = HashMap::new();
        let mut expected = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 1);
        expected.insert('c', 1);

        increment_rsp(&mut map, "aa\nbb\ncc");
        increment_rsp(&mut map, "aa\r\naa");
        assert_eq!(map, expected);
    }

    #[test]
    fn test_consensus() {
        let mut map = HashMap::new();
        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 1);
        expected.insert('c', 1);

        increment_by_consensus(&mut map, "abc\nabc");
    }
}
