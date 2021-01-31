use std::fs;

#[derive(Debug, PartialEq)]
struct Instruction {
    name: String,
    value: i32
}

fn parse_instructions(contents: &str) -> Vec<Instruction> {
    contents.lines().map(|value| {
        let split: Vec<&str> = value.split(' ').collect();
        let inst = String::from(split[0]);
        let val: i32 = split[1].parse().unwrap();
        Instruction{name: inst, value: val}
    }).collect()
}

fn swap_inst(inst: &mut Instruction) {
    match inst.name.as_str() {
        "nop" => {inst.name = String::from("jmp");}
        "jmp" => {inst.name = String::from("nop");}
        _ => {panic!("Unable to swap instruction");}
    }
}

fn accumulator_inf(insts: &mut Vec<Instruction>) -> i32 {
    let mut idx: i32 = 0;
    let mut acc = 0;

    let mut visited = Vec::new();

    let mut fix_candidates = Vec::new();
    let mut fix_attempts = Vec::new();

    let mut fix_idx: i32 = -1;

    loop {
        if idx as usize >= insts.len() {break;}

        if visited.contains(&idx) {
            if fix_idx != -1 {
                // undo prev fix
                swap_inst(&mut insts[fix_idx as usize]);
            }

            // find new fix idx
            let prev = fix_idx;
            for val in fix_candidates.iter().rev() {
                if !fix_attempts.contains(val) {
                    fix_idx = *val;
                }
            }
            if prev == fix_idx {panic!("Can't figure out a suitable fix");}

            // reset
            idx = 0;
            acc = 0;
            visited = Vec::new();

            // add new fix
            fix_attempts.push(fix_idx);
            swap_inst(&mut insts[fix_idx as usize]);
            // println!("New attempt");
            continue;
        }

        visited.push(idx);

        let inst = &insts[idx as usize];
        // println!("{} {} ({} {})", idx, acc, inst.name, inst.value);
        match inst.name.as_str() {
            "nop" => {
                if !fix_candidates.contains(&idx) {
                    fix_candidates.push(idx);
                }
                idx += 1;
            }
            "acc" => {
                idx += 1;
                acc += inst.value;
            }
            "jmp" => {
                if !fix_candidates.contains(&idx) {
                    fix_candidates.push(idx);
                }
                idx += inst.value;
            }
            _ => {panic!("Unknown instruction");}
        }
    }

    acc
}

fn main() {
    let contents = fs::read_to_string("day_8.txt")
                    .expect("Something went wrong reading input file");

    let mut instructions = parse_instructions(&contents);
    let result = accumulator_inf(&mut instructions);
    println!("Max value of the accumulator is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "nop +0\nacc +1\njmp -4";
        let expected = vec![
            Instruction{name: String::from("nop"), value: 0},
            Instruction{name: String::from("acc"), value: 1},
            Instruction{name: String::from("jmp"), value: -4}
        ];

        assert_eq!(parse_instructions(input), expected);
    }

    // #[test]
    // fn test_accumulator() {
    //     let mut input = vec![
    //         Instruction{name: String::from("nop"), value: 0},
    //         Instruction{name: String::from("jmp"), value: -1},
    //     ];

    //     assert_eq!(accumulator_inf(&mut input), 0);
    // }

    #[test]
    fn test_accumulator2() {
        let mut input = parse_instructions("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6");
        // assert_eq!(accumulator_inf(&mut input, false), 5);
        assert_eq!(accumulator_inf(&mut input), 8);
    }
}
