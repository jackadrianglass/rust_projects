extern crate regex;
use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum Inst {
    Mask{
        clear: u64,
        set: u64,
        wild: Vec<u64>
    },
    Write{ 
        addr: u64,
        val: u64
    }
}

impl Inst {
    fn from_str(content: &str) -> Inst {
        if content.contains("mask = ") {
            let mut clear = 0xffff_ffff_ffff_ffff;
            let mut set = 0;
            let mut wild = Vec::new();
            for (idx, c) in content[7..].chars().rev().enumerate() {
                match c {
                    'X' | 'x' => {wild.push(idx as u64);}
                    '0' => {clear &= !(1 << idx);}
                    '1' => {set |= 1 << idx;}
                    val => {println!("Unexpected character {} in {}", val, content); panic!();}
                }
            }
            return Inst::Mask{clear, set, wild};
        } else {
            let re = Regex::new(r"mem\[|\] = ").expect("Unable to construct regex");
            let values: Vec<&str> = re.split(content).filter(|val| *val != "").collect();

            let addr = values[0].parse().unwrap();
            let val = values[1].parse().unwrap();
            return Inst::Write{addr, val};
        }
    }

    fn parse_lines(content: &str) -> Vec<Inst> {
        content.lines().map(|val| Inst::from_str(val)).collect()
    }
}

fn init_program(insts: &[Inst]) -> u64 {
    let mut memory = HashMap::new();
    let mut clear_flag = 0;
    let mut set_flag = 0;
    for inst in insts.iter() {
        match inst {
            Inst::Mask {clear, set, wild: _} => {clear_flag = *clear; set_flag = *set;}
            Inst::Write {addr, val} => {
                let update = (val | set_flag) & clear_flag;
                match memory.get_mut(&addr) {
                    Some(val) => {*val = update;}
                    None => {memory.insert(addr, update);}
                }
            }
        }
    }
    memory.values().fold(0, |sum, val| sum + val)
}

fn init_program_v2(insts: &[Inst]) -> u64 {
    let mut memory = HashMap::new();
    let mut set_flag = 0;
    let mut wild_flags = &Vec::new();
    for inst in insts.iter() {
        match inst {
            Inst::Mask {clear: _, set, wild} => {
                set_flag = *set;
                wild_flags = wild;
            }
            Inst::Write {addr: address, val} => {
                let update_addrs = get_permutation(&wild_flags, set_flag, *address);
                for addr in update_addrs.iter(){
                    match memory.get_mut(addr) {
                        Some(value) => {*value = *val;}
                        None => {memory.insert(*addr, *val);}
                    }
                }
                
            }
        }
    }
    memory.values().fold(0, |sum, val| sum + val)
}

fn get_max_permutation(len: usize) -> usize {
    let mut val = 0;
    for idx in 0..len {
        val |= 1 << idx;
    }
    val
}

fn get_permutation(indices: &[u64], set: u64, addr: u64) -> Vec<u64> {
    let mut permutations = Vec::new();
    let masked_addr = addr | set;

    for wild_idx in 0..get_max_permutation(indices.len()) + 1 {
        let mut wild_set: u64 = 0;
        let mut wild_clear: u64 = 0xffff_ffff_ffff_ffff;
        for idx in 0..indices.len() {
            if ((1 << idx) & wild_idx) != 0 {
                wild_set |= 1 << indices[idx];
            } else {
                wild_clear &= !(1 << indices[idx]);
            }
        }
        permutations.push((masked_addr | wild_set) & wild_clear);
    }
    permutations
}

fn main() {
    let content = fs::read_to_string("day_14.txt").expect("Unable to read file");
    let insts = Inst::parse_lines(&content);
    let result = init_program(&insts);
    println!("Sum of the memory values is {}", result);
    let result = init_program_v2(&insts);
    println!("Sum of the memory values v2 is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Inst::from_str("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
                    Inst::Mask{clear:0xffff_ffff_ffff_fffd, set:0x40,
                        wild: vec![0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                                    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                                    30, 31, 32, 33, 34, 35]});
        assert_eq!(Inst::from_str("mem[8] = 11"),
                    Inst::Write{addr: 8, val: 11});
        assert_eq!(Inst::from_str("mem[7] = 101"),
                    Inst::Write{addr: 7, val: 101});
        assert_eq!(Inst::from_str("mask = 000000000000000000000000000000X1001X"),
                    Inst::Mask{clear: 0xffff_fff0_0000_0033, set: 0b10010, wild: vec![0, 5]});
    }

    #[test]
    fn test_parse_lines() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let expect = vec![
            Inst::Mask{clear:0xffff_ffff_ffff_fffd, set:0x40,
                wild: vec![0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                            30, 31, 32, 33, 34, 35]},
            Inst::Write{addr: 8, val: 11},
            Inst::Write{addr: 7, val: 101},
            Inst::Write{addr: 8, val: 0},
        ];
        assert_eq!(Inst::parse_lines(&input), expect);
    }

    #[test]
    fn test_init_memory() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let input = Inst::parse_lines(&input);
        assert_eq!(init_program(&input), 165);
    }

    #[test]
    fn test_get_max_permutations() {
        assert_eq!(get_max_permutation(0), 0);
        assert_eq!(get_max_permutation(1), 0x1);
        assert_eq!(get_max_permutation(2), 0x3);
    }

    #[test]
    fn test_get_permutations() {
        match Inst::from_str("mask = 000000000000000000000000000000X1001X") {
            Inst::Mask{set, clear: _, wild} => {
                let expect = vec![26, 27, 58, 59];
                assert_eq!(get_permutation(&wild, set, 42), expect);
            }
            Inst::Write{..} => {assert!(false);}
        }
    }
}
