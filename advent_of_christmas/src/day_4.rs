use std::fs;
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Default, PartialEq, Eq)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: i32,
    hcl: String,
    ecl: String,
    pid: i64,
    cid: i32
}

impl Passport {
    fn is_parsable(src: &str) -> bool {
        src.contains("byr") &&
        src.contains("iyr") &&
        src.contains("eyr") &&
        src.contains("hgt") &&
        src.contains("hcl") &&
        src.contains("ecl") &&
        src.contains("pid")
    }

    fn from_str(src: &str) -> Result<Self, ()>{
        if !Passport::is_parsable(src) { return Err(()); }

        let items: Vec<&str> = src.split_ascii_whitespace().collect();

        let mut pp = Passport{..Default::default()};
        for item in items {
            let split: Vec<&str> = item.split(":").collect();
            match split[0] {
                "byr" => {
                    pp.byr = split[1].parse().unwrap();
                    if pp.byr < 1920 || pp.byr > 2002 {
                        return Err(());
                    }
                }
                "iyr" => {
                    pp.iyr = split[1].parse().unwrap();
                    if pp.iyr < 2010 || pp.iyr > 2020 {
                        return Err(());
                    }
                }
                "eyr" => {
                    pp.eyr = split[1].parse().unwrap();
                    if pp.eyr < 2020 || pp.eyr > 2030 {
                        return Err(());
                    }
                }
                "hgt" => {
                    if split[1].contains("cm") {
                        let values: Vec<&str> = split[1].split("cm").collect();
                        pp.hgt = values[0].parse().unwrap();

                        if pp.hgt < 150 || pp.hgt > 193 {
                            return Err(());
                        }
                    }
                    else if split[1].contains("in") {
                        let values: Vec<&str> = split[1].split("in").collect();
                        pp.hgt = values[0].parse().unwrap();

                        if pp.hgt < 59 || pp.hgt > 76 {
                            return Err(());
                        }
                    }
                    else {
                        return Err(());
                    }
                }
                "hcl" => {
                    if split[1].len() != 7 {
                        return Err(());
                    }
                    if split[1].chars().nth(0).unwrap() != '#' {
                        return Err(());
                    }
                    for c in split[1].chars() {
                        if !c.is_alphanumeric() && c != '#' {
                            return Err(());
                        }
                    }
                    pp.hcl = split[1].to_owned();
                }
                "ecl" => {
                    pp.ecl = split[1].to_owned();
                    if pp.ecl != "amb"
                            && pp.ecl != "blu"
                            && pp.ecl != "brn"
                            && pp.ecl != "gry"
                            && pp.ecl != "grn"
                            && pp.ecl != "hzl"
                            && pp.ecl != "oth" {
                        return Err(());
                    }
                }
                "pid" => {
                    if split[1].len() != 9 { return Err(()); }
                    match split[1].parse() {
                        Ok(value) => {
                            pp.pid = value;
                            if pp.pid < 1 || pp.pid > 999999999 {
                                return Err(());
                            }
                        }
                        Err(_) => {
                            return Err(());
                        }
                    }
                    
                }
                "cid" => {
                    pp.cid = match split[1].parse() {
                        Ok(value) => {value}
                        Err(_) => {0}
                    };
                }
                &_ => {}
            }
        }
        Ok(pp)
    }
}

fn main() {
    let contents = fs::read_to_string("day_4.txt")
                    .expect("Something went wrong reading the file");
    
    let passport_str: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut passports = Vec::new();
    for string in passport_str {
        match Passport::from_str(string) {
            Ok(value) => {passports.push(value);}
            Err(_) => {}
        }
    }
    println!("{}", passports.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Passport::is_parsable("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"));
    }

    #[test]
    fn test_invalid() {
        assert!(!Passport::is_parsable("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929m"));
    }

    #[test]
    fn test_parse_valid() {
        assert_eq!(Passport::from_str("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f").unwrap(),
            Passport{pid: 087499704, hgt: 74, ecl: "grn".to_owned(), iyr:2012, eyr: 2030, byr: 1980, hcl:"#623a2f".to_owned(), cid: 0});

        assert_eq!(Passport::from_str("eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm").unwrap(),
            Passport{pid: 896056539, hgt: 165, ecl: "blu".to_owned(), iyr:2014, eyr: 2029, byr: 1989, hcl:"#a97842".to_owned(), cid: 129});

        assert_eq!(Passport::from_str("hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022").unwrap(),
            Passport{pid: 545766238, hgt: 164, ecl: "hzl".to_owned(), iyr:2015, eyr: 2022, byr: 2001, hcl:"#888785".to_owned(), cid: 88});
    }

    #[test]
    fn test_parse_invalid() {
        assert_eq!(Passport::from_str("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"), Err(()));
        assert_eq!(Passport::from_str("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:19466"), Err(()));
        assert_eq!(Passport::from_str("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"), Err(()));
        assert_eq!(Passport::from_str("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"), Err(()));
    }
}
