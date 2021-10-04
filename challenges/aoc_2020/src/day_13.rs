extern crate regex;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct EarlyBus {
    id: i32,
    wait_time: i32
}

fn earliest_bus(input: &Data) -> EarlyBus {
    let mut earliest = EarlyBus{id: 0, wait_time: 1000};
    for bus in input.buses.iter() {
        let wait_time = ((input.depart / bus) + 1) * bus - input.depart;
        if wait_time <= earliest.wait_time {
            earliest.wait_time = wait_time;
            earliest.id = *bus;
        }
    }
    earliest
}

#[derive(Debug, PartialEq)]
struct Data {
    depart: i32,
    buses: Vec<i32>
}

impl Data {
    fn from_str(input: &str) -> Data {
        let re = Regex::new(r",|,?x,?").expect("Failed to construct regex");
        let lines: Vec<&str> = input.lines().collect();
        let depart: i32 = lines[0].parse().unwrap();
        let buses = re.split(&lines[1])
                        .filter(|val| *val != "")
                        .map(|val| val.parse().unwrap())
                        .collect();
        Data{depart, buses}
    }
}

//part 2
#[derive(Debug, PartialEq)]
struct Bus {
    id: i64,
    idx: i64,
}

fn parse_buses(input: &str) -> Vec<Bus> {
    let lines: Vec<&str> = input.lines().collect();
    let mut buses = Vec::new();
    for (idx, bus) in lines[1].split(',').enumerate() {
        match bus {
            "x" => {}
            value => {buses.push(Bus{id: value.parse().unwrap(), idx: idx as i64});}
        }
    }
    buses
}

fn find_time(buses: &[Bus]) -> i64 {
    let increment = buses[0].id;
    let mut time = increment;
    println!("{:?}", buses);
    loop {
        if buses.iter().fold(true, |valid, val| ((time + val.idx) % val.id == 0) && valid) {
            break;
        } else {
            time += increment;
        }
    }
    time
}

fn main() {
    let contents = fs::read_to_string("day_13.txt").expect("Unable to read file");
    let input = Data::from_str(&contents);
    let earliest = earliest_bus(&input);
    println!("Earliest bus is {:?} with a product of {}", earliest, earliest.id * earliest.wait_time);

    let input = parse_buses(&contents);
    //This will take forever
    let earliest = find_time(&input);
    println!("Value of time is {}", earliest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let expect = Data{depart: 939, buses: vec![7, 13, 59, 31, 19]};
        assert_eq!(Data::from_str(&input), expect);
    }

    #[test]
    fn test_earliest_bus() {
        let input = Data::from_str("939\n7,13,x,x,59,x,31,19");
        assert_eq!(earliest_bus(&input), EarlyBus{id: 59, wait_time: 5});
    }

    #[test]
    fn test_parse2() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let expect = vec![
            Bus{id: 7, idx: 0},
            Bus{id: 13, idx: 1},
            Bus{id: 59, idx: 4},
            Bus{id: 31, idx: 6},
            Bus{id: 19, idx: 7},
        ];

        assert_eq!(parse_buses(&input), expect);
    }

    #[test]
    fn test_time_thingy() {
        let input = parse_buses("123\n17,x,13,19");
        assert_eq!(find_time(&input), 3417);
        let input = parse_buses("939\n67,7,59,61");
        assert_eq!(find_time(&input), 754018);
        let input = parse_buses("939\n67,x,7,59,61");
        assert_eq!(find_time(&input), 779210);
        let input = parse_buses("939\n67,7,x,59,61");
        assert_eq!(find_time(&input), 1261476);
        let input = parse_buses("939\n1789,37,47,1889");
        assert_eq!(find_time(&input), 1202161486);
    }
}
