use std::fs;

extern crate regex;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Bounds {
    upper: usize,
    lower: usize,
}

impl Bounds {
    fn contains(&self, val: usize) -> bool {
        val >= self.lower && val <= self.upper
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    name: String,
    bounds: [Bounds; 2],
}

impl Rule {
    fn parse_many(src: &str) -> Vec<Self> {
        src.lines().map(|val| Rule::from_str(&val)).collect()
    }

    fn from_str(src: &str) -> Self {
        let re = Regex::new(r": |-| or ").expect("Unable to create regex");
        let contents: Vec<&str> = re.split(src).collect();
        return Rule {
            name: String::from(contents[0]),
            bounds: [
                Bounds{lower: contents[1].parse().unwrap(), upper: contents[2].parse().unwrap()},
                Bounds{lower: contents[3].parse().unwrap(), upper: contents[4].parse().unwrap()},
            ]
        };
    }

    fn matches(&self, val: usize) -> bool {
        self.bounds.iter().any(|bound| bound.contains(val))
    }
}

fn parse_many_lines(src: &str) ->Vec<Vec<usize>> {
    let mut result = Vec::new();
    for (idx, line) in src.lines().enumerate(){
        if idx == 0 {continue;}
        result.push(parse_line(&line));
    }
    result
}
fn parse_only_second(src: &str) -> Vec<usize> {
    let lines: Vec<&str> = src.lines().collect();
    parse_line(&lines[1])
}

fn parse_line(src: &str) -> Vec<usize> {
    src.split(',').map(|val| val.parse().unwrap()).collect()
}

fn find_ticket_error_rate(rules: &[Rule], tickets: &Vec<Vec<usize>>) -> usize {
    tickets.iter().fold(0,
        |sum, val| sum + val.iter().fold(0, 
        |sum, val| {
            if rules.iter().any(|rule| rule.matches(*val)) {
                return sum;
            }
            sum + val
        }))
}

fn determine_rule_indices(rules: &[Rule], tickets: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut possibilities: Vec<Vec<usize>> = vec![(0..rules.len()).collect(); rules.len()];
    let valid : Vec<&Vec<usize>>= tickets.iter().filter(|ticket|
                                                        !ticket.iter().any(|val|
                                                        rules.iter().all(|rule| !rule.matches(*val)))).collect();

    loop {
        for ticket in &valid {
            for (idx, rule) in rules.iter().enumerate() {
                let current = &mut possibilities[idx];
                let bad_matches: Vec<(usize, &usize)> = ticket.iter().enumerate().filter(|(_, val)| !rule.matches(**val)).collect();
                for (jdx, _) in bad_matches {
                    if let Some(pos) = current.iter().position(|tmp| *tmp == jdx) {
                        current.remove(pos);
                    }
                }
                if current.len() == 1 {
                    let val = current[0];
                    for (other_idx, other) in possibilities.iter_mut().enumerate() {
                        if idx == other_idx {
                            continue;
                        }
                        if let Some(pos) = other.iter().position(|tmp| *tmp == val) {
                            other.remove(pos);
                        }
                    }
                    continue;
                }
            }
        }
        if possibilities.iter().all(|container| container.len() == 1) {
            break;
        }
    }
    possibilities.iter().map(|v| v[0]).collect()
}

fn main() {
    let contents = fs::read_to_string("day_16.txt").expect("Unable to read file.");
    let re = Regex::new("\r\n\r\n").expect("Unable to create regex");
    let contents: Vec<&str> = re.split(&contents).collect();
    
    let rules = Rule::parse_many(&contents[0]);
    let ticket = parse_only_second(&contents[1]);
    let other_tickets = parse_many_lines(&contents[2]);

    println!("ticket {:?}", ticket);

    let error_rate = find_ticket_error_rate(&rules, &other_tickets);
    println!("The scanning error rate is {}", error_rate);

    let rule_idx = determine_rule_indices(&rules, &other_tickets);
    println!("Rules idx {:?}", rule_idx);

    let result = rules.iter().enumerate().fold(1, |prod, (idx, val)|{
        if val.name.contains("departure") {
            return prod * ticket[rule_idx[idx]];
        }
        prod
    });

    println!("The product is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_test() {
        let bound = Bounds{upper: 10, lower: 5};
        assert!(bound.contains(6));
        assert!(bound.contains(5));
        assert!(bound.contains(10));
        assert!(!bound.contains(3));
        assert!(!bound.contains(11));
    }

    #[test]
    fn parse_rule() {
        let expect = Rule {
            name: String::from("class"),
            bounds: [
                Bounds{lower: 1, upper: 3},
                Bounds{lower: 5, upper: 7}
            ]
        };

        assert_eq!(Rule::from_str("class: 1-3 or 5-7"), expect);
    }

    #[test]
    fn error_rate() {
        let rules = Rule::parse_many(r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50");

        let tickets = parse_many_lines(r"nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");

        assert_eq!(find_ticket_error_rate(&rules, &tickets), 71);
    }

    #[test]
    fn determine_classes() {
        let rules = Rule::parse_many(r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50");

        let tickets = parse_many_lines(r"nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");

        let expected = vec![1, 0, 2];
        assert_eq!(determine_rule_indices(&rules, &tickets), expected);
    }
}
