mod beat {
    extern crate rand;
    use rand::{Rng, thread_rng};

    #[derive(Clone, Copy, Debug)]
    pub enum Division {
        Binary,
        Ternary,
        Quaternary
    }

    //Ordered so that they mirror how binary maps out
    static BINARY : [char; 4] = ['D', 'B', 'A', 'C'];
    static TERNARY: [char; 8] = ['X', 'S', 'R', 'U', 'Q', 'V', 'T', 'W'];
    static QUATERNARY: [char; 16] = ['P', 'D', 'C', 'G', 'B', 'J', 'F', 'L',
                                        'A', 'H', 'I', 'M', 'E', 'N', 'K', 'O'];

    pub fn max_val_of(division: Division) -> usize {
        match division {
            Division::Binary => {
                BINARY.len()
            }
            Division::Ternary => {
                TERNARY.len()
            }
            Division::Quaternary => {
                QUATERNARY.len()
            }
        }
    }

    pub fn get_rand_num(division: Division) -> usize {
        let mut rng = thread_rng();
        match division {
            Division::Binary => {
                return rng.gen_range(0..BINARY.len());
            }
            Division::Ternary => {
                return rng.gen_range(0..TERNARY.len());
            }
            Division::Quaternary => {
                return rng.gen_range(0..QUATERNARY.len());
            }
        }
    }

    pub fn get_idx(letter: char, division: Division) -> Result<usize,()> {
        match division {
            Division::Binary => {
                match BINARY.iter().position(|val| *val == letter) {
                    Some(val) => {return Ok(val);}
                    None => {return Err(());}
                }
            }
            Division::Ternary => {
                match TERNARY.iter().position(|val| *val == letter) {
                    Some(val) => {return Ok(val);}
                    None => {return Err(());}
                }
            }
            Division::Quaternary => {
                match QUATERNARY.iter().position(|val| *val == letter) {
                    Some(val) => {return Ok(val);}
                    None => {return Err(());}
                }
            }
        }
    }

    pub fn gen_sequence(division: Division, length: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(length);
        for _ in 0..length {
            result.push(get_rand_num(division));
        }
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_rand_num() {
            for _ in 0..5 {
                let val = get_rand_num(Division::Binary);
                assert!(val < 4);
                let val = get_rand_num(Division::Ternary);
                assert!(val < 8);
                let val = get_rand_num(Division::Quaternary);
                assert!(val < 16);
            }
        }

        #[test]
        fn test_get_idx_quaternary() {
            assert_eq!(get_idx('A', Division::Quaternary), Ok(0x8));
            assert_eq!(get_idx('B', Division::Quaternary), Ok(0x4));
            assert_eq!(get_idx('C', Division::Quaternary), Ok(0x2));
            assert_eq!(get_idx('D', Division::Quaternary), Ok(0x1));
            assert_eq!(get_idx('E', Division::Quaternary), Ok(0xC));
            assert_eq!(get_idx('F', Division::Quaternary), Ok(0x6));
            assert_eq!(get_idx('G', Division::Quaternary), Ok(0x3));
            assert_eq!(get_idx('H', Division::Quaternary), Ok(0x9));
            assert_eq!(get_idx('I', Division::Quaternary), Ok(0xa));
            assert_eq!(get_idx('J', Division::Quaternary), Ok(0x5));
            assert_eq!(get_idx('K', Division::Quaternary), Ok(0xe));
            assert_eq!(get_idx('L', Division::Quaternary), Ok(0x7));
            assert_eq!(get_idx('M', Division::Quaternary), Ok(0xb));
            assert_eq!(get_idx('N', Division::Quaternary), Ok(0xd));
            assert_eq!(get_idx('O', Division::Quaternary), Ok(0xf));
            assert_eq!(get_idx('P', Division::Quaternary), Ok(0x0));
        }

        #[test]
        fn test_get_idx_ternary() {
            assert_eq!(get_idx('Q', Division::Ternary), Ok(0x4));
            assert_eq!(get_idx('R', Division::Ternary), Ok(0x2));
            assert_eq!(get_idx('S', Division::Ternary), Ok(0x1));
            assert_eq!(get_idx('T', Division::Ternary), Ok(0x6));
            assert_eq!(get_idx('U', Division::Ternary), Ok(0x3));
            assert_eq!(get_idx('V', Division::Ternary), Ok(0x5));
            assert_eq!(get_idx('W', Division::Ternary), Ok(0x7));
            assert_eq!(get_idx('X', Division::Ternary), Ok(0x0));
        }

        #[test]
        fn test_get_idx_binary() {
            assert_eq!(get_idx('A', Division::Binary), Ok(0x2));
            assert_eq!(get_idx('B', Division::Binary), Ok(0x1));
            assert_eq!(get_idx('C', Division::Binary), Ok(0x3));
            assert_eq!(get_idx('D', Division::Binary), Ok(0x0));

        }
    }
}
