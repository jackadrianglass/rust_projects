use std::fs;

#[derive(PartialEq, PartialOrd, Debug)]
struct Policy {
    character: char,
    lower: i32,
    upper: i32,
    password: String
}

impl Policy {
    fn from_str(value: &str) -> Self {
        let split = value.split(" ");
        let split = split.collect::<Vec<&str>>();
        let bounds = split[0].split("-").collect::<Vec<&str>>();

        Policy {
            character: split[1].replace(":", "").chars().nth(0).unwrap(),
            lower: bounds[0].parse::<i32>().unwrap(),
            upper: bounds[1].parse::<i32>().unwrap(),
            password: split[2].to_owned()
        }
    }
}

fn policy1_matches(policy: &Policy) -> bool {
    let mut times = 0;
    for c in policy.password.chars() {
        if c == policy.character{
            times += 1;
        }
    }
    return times >= policy.lower && times <= policy.upper;
}

fn num_policy1_matches(policies: &[Policy]) -> i32 {
    let mut num = 0;
    for policy in policies {
        if policy1_matches(policy) {
            num += 1;
        }
    }
    return num;
}

fn policy2_matches(policy: &Policy) -> bool {
    let c : Vec<char> = policy.password.chars().collect();

    let upper = (policy.upper - 1) as usize;
    let lower = (policy.lower - 1) as usize;
    let character = policy.character;

    if lower < policy.password.len() && upper < policy.password.len() {
        return (c[lower] == character && c[upper] != character)
                || (c[lower] != character && c[upper] == character);
    }
    return false;
}

fn num_policy2_matches(policies: &[Policy]) -> i32 {
    let mut num = 0;
    for policy in policies {
        if policy2_matches(policy) {
            num += 1;
        }
    }
    return num;
}

fn main() {
    println!("Reading list.txt");

    let contents = fs::read_to_string("day_2.txt")
                    .expect("Something went wrong reading the file");

    let policies = contents.lines().map(|line| Policy::from_str(line)).collect::<Vec<Policy>>();
    // println!("{:?}", policies);
    let num_valid = num_policy1_matches(&policies);
    println!("The result for first is {}", num_valid);

    let num_valid = num_policy2_matches(&policies);
    println!("The result for second is {}", num_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_policy() {
        assert_eq!(Policy::from_str("1-3 a: blah"), Policy{character: 'a', lower: 1, upper: 3, password: String::from("blah")});
    }

    #[test]
    fn does_policy_match() {
        let policy = Policy::from_str("1-3 a: blah");
        assert!(policy1_matches(&policy));
    }

    #[test]
    fn policy_doesnt_match() {
        let policy = Policy::from_str("2-4 c: crap");
        assert!(!policy1_matches(&policy));
    }

    #[test]
    fn policies_matches() {
        let policies = [
            Policy::from_str("1-3 a: banana"),
            Policy::from_str("1-3 a: shoot"),
            Policy::from_str("1-3 c: cccccccc"),
        ];

        assert_eq!(num_policy1_matches(&policies), 1);
    }
}
