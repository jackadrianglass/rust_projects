pub mod benny;

extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Groove {
    pub structure: Structure,
    pub high_hat: Vec<i8>,
    pub snare: Vec<i8>,
    pub bass: Vec<i8>
}


#[derive(Debug)]
pub struct Structure {
    pub time_signature: TimeSignature,
    pub grouping: Grouping,
    pub bars: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Grouping {
    Binary,
    Ternary,
    Quaternary,
}

impl Grouping {
    pub fn from_str(src: &str) -> Result<Self, ()> {
        match src {
            "binary" | "2" => Ok(Grouping::Binary),
            "ternary" | "3" => Ok(Grouping::Ternary),
            "quaternary" | "4" => Ok(Grouping::Quaternary),
            _ => Err(()),
        }
    }

    pub fn max_val(self) -> i8 {
        match self {
            Grouping::Binary => 4,
            Grouping::Ternary => 8,
            Grouping::Quaternary => 16,
        }
    }

    pub fn num_beats(self) -> usize {
        match self {
            Grouping::Binary => 2,
            Grouping::Ternary => 3,
            Grouping::Quaternary => 4,
        }
    }

    pub fn get_rand_num(self) -> i8 {
        let mut rng = thread_rng();
        match self {
            Grouping::Binary => rng.gen_range(0..self.max_val()),
            Grouping::Ternary => rng.gen_range(0..self.max_val()),
            Grouping::Quaternary => rng.gen_range(0..self.max_val()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TimeSignature {
    pub number: i32,
    pub divisions: i32
}

impl TimeSignature {
    pub fn from_str(src: &str) -> Result<Self, &str> {
        if !src.contains("/") {
            return Err("Does not contain a /");
        }
        let parts: Vec<&str> = src.split("/").collect();
        let number = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => {return Err("Unable to parse top number of time signature");}
        };
        let divisions = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => {return Err("Unable to parse bottom number of time signature");}
        };
        Ok(TimeSignature{number, divisions})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rand_num() {
        for _ in 0..5 {
            let val = Grouping::get_rand_num(Grouping::Binary);
            assert!(val < 4);
            let val = Grouping::get_rand_num(Grouping::Ternary);
            assert!(val < 8);
            let val = Grouping::get_rand_num(Grouping::Quaternary);
            assert!(val < 16);
        }
    }

    #[test]
    fn test_create_time_signature() {
        assert_eq!(TimeSignature::from_str("4/4"), Ok(TimeSignature{number: 4, divisions: 4}));
        match TimeSignature::from_str("44") {
            Ok(val) => assert!(false, "Parsed a time signature without a /: {:?}", val),
            Err(_) => {}
        }
        match TimeSignature::from_str("4a/4") {
            Ok(val) => assert!(false, "Parsed a time signature with a non integer value: {:?}", val),
            Err(_) => {}
        }
    }
}
