extern crate regex;
use regex::Regex;

use std::fs;
use std::collections::HashMap;

fn policy_from_str(string: &str) -> (String, Vec<(String, i32)>) {
    let re = Regex::new(r" |, |\. |\.").unwrap();
    let tokens: Vec<&str> = re.split(string).collect();

    let mut bagname = String::new(); 
    let mut quantity = 0;
    let mut other_bag = String::new();

    let mut contents = Vec::new();
    for token in tokens {
        match token {
            "" => {continue;}
            "bag" | "bags" => {
                if quantity != 0 {
                    if other_bag != "no other" {
                        contents.push((other_bag.clone(), quantity));
                    }
                    other_bag = String::new();
                }
            }
            "contain" => {}
            value => {
                match value.parse::<i32>() {
                    Ok(value) => {quantity = value;}
                    Err(_) => {
                        if quantity == 0 {
                            if bagname == "" {
                                bagname = String::from(value);
                            } else {
                                bagname.push_str(" ");
                                bagname.push_str(value);
                            }
                        } else {
                            if other_bag == "" {
                                other_bag = String::from(value);
                            } else {
                                other_bag.push_str(" ");
                                other_bag.push_str(value);
                            }
                        }
                    }
                }
            }
        }
    }
    (bagname, contents)
}

fn num_bags_can_fit(query: &str, conaining_bag: &str, bag_listing: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut total = 0;
    match bag_listing.get(conaining_bag) {
        Some(bags) => {
            for (bag, quantity) in bags {
                if query == bag {
                    total += quantity;
                } else {
                    total += quantity * num_bags_can_fit(query, bag, bag_listing)
                }
            }
        }
        None => {}
    }
    total
}

fn bag_contains(query: &str, bag_listing: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut total = 0;
    match bag_listing.get(query) {
        Some(bags) => {
            for (bag, quantity) in bags {
                total += quantity;
                total += quantity * bag_contains(bag, bag_listing);
            }
        }
        None => {}
    }
    total
}

fn main() {
    let contents = fs::read_to_string("day_7.txt")
        .expect("Something went wrong reading the file");

    let mut bag_listing = HashMap::new();

    for line in contents.lines() {
        let (key, value) = policy_from_str(line);
        bag_listing.insert(key, value);
    }

    let mut can_contain = 0;
    for bag in bag_listing.keys() {
        if num_bags_can_fit("shiny gold", bag, &bag_listing) > 0 {
            can_contain += 1;
        }
    }
    println!("There are {} bags that can contain a shiny gold bag", can_contain);

    let num_bags = bag_contains("shiny gold", &bag_listing);
    println!("The shiny gold bag contains {} other bags", num_bags);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_policy() {
        let expected = ( 
            String::from("light red"),
            vec![
                (String::from("bright white"), 1),
                (String::from("muted yellow"), 2)
            ]
        );

        assert_eq!(policy_from_str("light red bags contain 1 bright white bag, 2 muted yellow bags."), expected);
    }

    #[test]
    fn test_bag_fitting() {
        let policies = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let mut map = HashMap::new();
        for line in policies.lines() {
            let (key, value) = policy_from_str(line);
            map.insert(key, value);
        }

        assert_eq!(num_bags_can_fit("bright white", "dotted black", &map), 0);
        assert_eq!(num_bags_can_fit("shiny gold", "bright white", &map), 1);
        assert_eq!(num_bags_can_fit("shiny gold", "muted yellow", &map), 2);
        assert_eq!(num_bags_can_fit("shiny gold", "dark orange", &map), 11);
        assert_eq!(num_bags_can_fit("shiny gold", "light red", &map), 5);
    }

    #[test]
    fn test_bag_containment() {
        let policies = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let mut map = HashMap::new();
        for line in policies.lines() {
            let (key, value) = policy_from_str(line);
            map.insert(key, value);
        }

        assert_eq!(bag_contains("dark violet", &map), 0);
        assert_eq!(bag_contains("dark blue", &map), 2);
        assert_eq!(bag_contains("dark green", &map), 6);
        assert_eq!(bag_contains("shiny gold", &map), 126);
    }
}
