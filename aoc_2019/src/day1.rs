use std::fs;

fn get_mass_1(module: i32) -> i32 {
    module / 3 - 2
}

fn get_mass_2(module: i32) -> i32 {
    let fuel = module / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + get_mass_2(fuel)
}

fn main() {
    let contents = fs::read_to_string("day1.txt").expect("Uh Oh!");
    let lines: Vec<&str> = contents.split("\r\n").collect();
    let input: Vec<i32> = lines.iter().map(|val| val.parse().unwrap()).collect();
    let result = input.iter().fold(0, |acc, val| acc + get_mass_1(*val));
    println!("Result is {}", result);
    let result = input.iter().fold(0, |acc, val| acc + get_mass_2(*val));
    println!("Result is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mass_1() {
        assert_eq!(get_mass_1(12), 2);
        assert_eq!(get_mass_1(14), 2);
        assert_eq!(get_mass_1(1969), 654);
        assert_eq!(get_mass_1(100756), 33583);
    }

    #[test]
    fn test_get_mass_2() {
        assert_eq!(get_mass_1(12), 2);
        assert_eq!(get_mass_1(14), 2);
        assert_eq!(get_mass_2(1969), 966);
        assert_eq!(get_mass_2(100756), 50346);
    }
}
