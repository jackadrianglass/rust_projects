#![allow(dead_code)]
use super::*;

//Ordered so that they mirror how binary maps out
static TERNARY: [char; 8] = ['X', 'S', 'R', 'U', 'Q', 'V', 'T', 'W'];
static QUATERNARY: [char; 16] = [
    'P', 'D', 'C', 'G', 'B', 'J', 'F', 'L', 'A', 'H', 'I', 'M', 'E', 'N', 'K', 'O',
];

#[derive(Debug, PartialEq)]
pub enum Inst {
    HighHat,
    Snare,
    Bass,
}

#[derive(Debug, PartialEq)]
pub enum Filter {
    Random { inst: Inst },
    Include { inst: Inst, values: Vec<i8> },
    Exclude { inst: Inst, values: Vec<i8> },
    Sequence { inst: Inst, values: Vec<i8> },
}

impl Filter {
    pub fn from_str(src: &str, grouping: &Grouping) -> Result<Self, ()> {
        let inst = match src.chars().nth(0) {
            Some('H') => Inst::HighHat,
            Some('S') => Inst::Snare,
            Some('B') => Inst::Bass,
            Some(_) => return Err(()),
            None => return Err(()),
        };
        let values: Vec<i8> = src[2..]
            .chars()
            .map(|val| get_hex(&val, grouping).unwrap())
            .collect();

        return match src.chars().nth(1) {
            Some('+') => Ok(Filter::Include { inst, values }),
            Some('-') => Ok(Filter::Exclude { inst, values }),
            Some('~') => Ok(Filter::Sequence { inst, values }),
            Some('?') => Ok(Filter::Random { inst }),
            Some(_) | None => Err(()),
        };
    }
}

pub fn get_hex(letter: &char, division: &Grouping) -> Result<i8, ()> {
    match division {
        Grouping::Ternary => match TERNARY.iter().position(|val| *val == *letter) {
            Some(val) => {
                return Ok(val as i8);
            }
            None => {
                return Err(());
            }
        },
        Grouping::Quaternary => match QUATERNARY.iter().position(|val| *val == *letter) {
            Some(val) => {
                return Ok(val as i8);
            }
            None => {
                return Err(());
            }
        },
    }
}

pub fn gen_groove(style: &Style, bars: i32, gen_type: &GenerationType) -> Groove {
    match style {
        Style::Funk => gen_funk_groove(bars, gen_type),
        Style::Jazz => gen_jazz_groove(bars, gen_type),
    }
}

fn gen_funk_groove(bars: i32, gen_type: &GenerationType) -> Groove {
    let structure = Structure {
        time_signature: TimeSignature {
            number: 4,
            divisions: 4,
        },
        grouping: Grouping::Quaternary,
        bars,
    };
    let hh_pattern = "H~".to_string() + &"IIII".repeat(bars as usize);
    let snare_pattern = "H~".to_string() + &"PAPA".repeat(bars as usize);
    let Filter::Sequence{values: high_hat, ..} = Filter::from_str(&hh_pattern, &structure.grouping).unwrap() else {
        panic!("Ya done goofed the high hat sequence");
    };
    let Filter::Sequence{values: snare, ..} = Filter::from_str(&snare_pattern, &structure.grouping).unwrap() else {
        panic!("Ya done goofed the snare sequence");
    };

    let bass = gen_sequence(&structure, gen_type);

    Groove {
        structure,
        high_hat,
        snare,
        bass,
    }
}

fn gen_jazz_groove(bars: i32, gen_type: &GenerationType) -> Groove {
    let structure = Structure {
        time_signature: TimeSignature {
            number: 4,
            divisions: 4,
        },
        grouping: Grouping::Ternary,
        bars,
    };
    let hh_pattern = "H~".to_string() + &"QVQV".repeat(bars as usize);
    let Filter::Sequence{values: high_hat, ..} = Filter::from_str(&hh_pattern, &structure.grouping).unwrap() else {
        panic!("Ya done goofed the high hat sequence");
    };

    let snare = gen_sequence(&structure, gen_type);
    let bass = gen_sequence(&structure, gen_type);

    Groove {
        structure,
        high_hat,
        snare,
        bass,
    }
}

fn gen_sequence(structure: &Structure, gen_type: &GenerationType) -> Vec<i8> {
    match gen_type {
        GenerationType::Letters => gen_letter_sequence(structure),
        GenerationType::OddGroupings => gen_letter_sequence(structure),
    }
}

fn gen_letter_sequence(structure: &Structure) -> Vec<i8> {
    let length = structure.length();
    let mut result = Vec::with_capacity(length);
    for _ in 0..length {
        result.push(structure.grouping.get_rand_num());
    }
    result
}

fn gen_odd_grouping_sequence(structure: &Structure) -> Vec<i8> {
    let mut groupings: Vec<i8> = vec![3, 5, 7];
    let ticks = structure.ticks();

    let mut seq: Vec<i8> = Vec::new();
    let mut rng = thread_rng();

    loop {
        seq.push(groupings[rng.gen_range(0..groupings.len()) as usize]);
        let sum: i8 = seq.iter().sum();
        let remaining = ticks as i8 - sum;

        if remaining == 0 {
            break;
        }

        groupings = groupings
            .iter()
            .filter(|v| v >= &&remaining)
            .map(|v| *v)
            .collect();

        if groupings.is_empty() {
            seq.push(remaining);
            break;
        }
    }

    populate_beat_sequence(structure, &seq)
}

fn populate_beat_sequence(structure: &Structure, seq: &[i8]) -> Vec<i8> {
    let mut result = vec![0; structure.length()];
    let mut sum: i8 = 0;
    let bit_init = 1 << (structure.grouping.num_beats() - 1);

    for val in seq.iter() {
        let idx = sum as usize / structure.grouping.num_beats();
        let bit = bit_init >> sum % (structure.grouping.num_beats() as i8);

        sum += *val;

        result[idx] |= bit;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hex_quaternary() {
        assert_eq!(get_hex(&'A', &Grouping::Quaternary), Ok(0x8));
        assert_eq!(get_hex(&'B', &Grouping::Quaternary), Ok(0x4));
        assert_eq!(get_hex(&'C', &Grouping::Quaternary), Ok(0x2));
        assert_eq!(get_hex(&'D', &Grouping::Quaternary), Ok(0x1));
        assert_eq!(get_hex(&'E', &Grouping::Quaternary), Ok(0xC));
        assert_eq!(get_hex(&'F', &Grouping::Quaternary), Ok(0x6));
        assert_eq!(get_hex(&'G', &Grouping::Quaternary), Ok(0x3));
        assert_eq!(get_hex(&'H', &Grouping::Quaternary), Ok(0x9));
        assert_eq!(get_hex(&'I', &Grouping::Quaternary), Ok(0xa));
        assert_eq!(get_hex(&'J', &Grouping::Quaternary), Ok(0x5));
        assert_eq!(get_hex(&'K', &Grouping::Quaternary), Ok(0xe));
        assert_eq!(get_hex(&'L', &Grouping::Quaternary), Ok(0x7));
        assert_eq!(get_hex(&'M', &Grouping::Quaternary), Ok(0xb));
        assert_eq!(get_hex(&'N', &Grouping::Quaternary), Ok(0xd));
        assert_eq!(get_hex(&'O', &Grouping::Quaternary), Ok(0xf));
        assert_eq!(get_hex(&'P', &Grouping::Quaternary), Ok(0x0));
    }

    #[test]
    fn test_get_hex_ternary() {
        assert_eq!(get_hex(&'Q', &Grouping::Ternary), Ok(0x4));
        assert_eq!(get_hex(&'R', &Grouping::Ternary), Ok(0x2));
        assert_eq!(get_hex(&'S', &Grouping::Ternary), Ok(0x1));
        assert_eq!(get_hex(&'T', &Grouping::Ternary), Ok(0x6));
        assert_eq!(get_hex(&'U', &Grouping::Ternary), Ok(0x3));
        assert_eq!(get_hex(&'V', &Grouping::Ternary), Ok(0x5));
        assert_eq!(get_hex(&'W', &Grouping::Ternary), Ok(0x7));
        assert_eq!(get_hex(&'X', &Grouping::Ternary), Ok(0x0));
    }

    #[test]
    fn test_get_filter() {
        assert_eq!(
            Ok(Filter::Include {
                inst: Inst::HighHat,
                values: vec![0x0, 0x1]
            }),
            Filter::from_str("H+PD", &Grouping::Quaternary)
        );
    }

    #[test]
    fn test_populate_beat_sequence_quaternary() {
        let seq = vec![7, 3, 3, 3];
        let structure = Structure {
            time_signature: TimeSignature {
                number: 4,
                divisions: 4,
            },
            grouping: Grouping::Quaternary,
            bars: 1,
        };

        let result = populate_beat_sequence(&structure, &seq);
        assert_eq!(result, vec![0b1000, 0b0001, 0b0010, 0b0100]);
    }

    #[test]
    fn test_populate_beat_sequence_ternary() {
        let seq = vec![5, 3, 3, 1];
        let structure = Structure {
            time_signature: TimeSignature {
                number: 4,
                divisions: 4,
            },
            grouping: Grouping::Ternary,
            bars: 1,
        };

        let result = populate_beat_sequence(&structure, &seq);
        assert_eq!(result, vec![0b100, 0b001, 0b001, 0b001]);
    }
}
