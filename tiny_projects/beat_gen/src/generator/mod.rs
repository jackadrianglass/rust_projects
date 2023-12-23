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


#[derive(Debug, Clone)]
pub enum GenerationType {
    Letters,
    OddGroupings,
}

impl GenerationType {
    pub fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "letters" => Ok(Self::Letters),
            "odd" => Ok(Self::OddGroupings),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum Style {
    Jazz,
    Funk,
}

impl Style {
    pub fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "jazz" => Ok(Self::Jazz),
            "funk" => Ok(Self::Funk),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub time_signature: TimeSignature,
    pub grouping: Grouping,
    pub bars: i32,
}

impl Structure {
    pub fn length(&self) -> usize {
        (self.time_signature.number * self.bars) as usize
    }

    pub fn ticks(&self) -> usize {
        (self.time_signature.number * self.time_signature.divisions * self.bars) as usize
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Grouping {
    Ternary,
    Quaternary,
}

impl Grouping {
    pub fn from_str(src: &str) -> Result<Self, ()> {
        match src {
            "ternary" | "3" => Ok(Grouping::Ternary),
            "quaternary" | "4" => Ok(Grouping::Quaternary),
            _ => Err(()),
        }
    }

    pub fn max_val(self) -> i8 {
        match self {
            Grouping::Ternary => 8,
            Grouping::Quaternary => 16,
        }
    }

    pub fn num_beats(self) -> usize {
        match self {
            Grouping::Ternary => 3,
            Grouping::Quaternary => 4,
        }
    }

    pub fn get_rand_num(self) -> i8 {
        let mut rng = thread_rng();
        match self {
            Grouping::Ternary => rng.gen_range(0..self.max_val()),
            Grouping::Quaternary => rng.gen_range(0..self.max_val()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
