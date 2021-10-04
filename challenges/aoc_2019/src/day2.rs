use std::fs;

const ADD: i32 = 1;
const MULT: i32 = 2;
const EXIT: i32 = 99;

fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    for i in (0..program.len()).step_by(4) {
        let opcode = program[i];
        if opcode == EXIT {
            break;
        }

        let src1 = program[program[i + 1] as usize];
        let src2 = program[program[i + 2] as usize];
        let dest = program[i + 3] as usize;
        match opcode {
            ADD => program[dest] = src1 + src2,
            MULT => program[dest] = src1 * src2,
            _ => panic!("Unexpected opcode {} @ {}", opcode, i),
        }
    }
    program
}

fn find_noun_verb(program: &mut Vec<i32>) -> (i32, i32) {
    for noun in 0..99 {
        for verb in 0..99 {
            program[1] = noun;
            program[2] = verb;
            let result = run_program(program.clone());
            if result[0] == 19690720 {
                return (noun, verb);
            }
        }
    }
    (0, 0)
}

fn main() {
    let contents = fs::read_to_string("day2.txt").expect("Can't find day2.txt");
    let lines: Vec<&str> = contents.split(",").collect();
    let mut program: Vec<i32> = lines.iter().map(|v| v.parse().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    let program_1 = run_program(program.clone());
    println!("Program[0] is {}", program_1[0]);

    let (noun, verb) = find_noun_verb(&mut program);
    println!("Noun {} Verb {} Result {}", noun, verb, 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_1() {
        assert_eq!(vec![2, 0, 0, 0, 99], run_program(vec![1, 0, 0, 0, 99]));
        assert_eq!(vec![2, 3, 0, 6, 99], run_program(vec![2, 3, 0, 3, 99]));
        assert_eq!(
            vec![2, 4, 4, 5, 99, 9801],
            run_program(vec![2, 4, 4, 5, 99, 0])
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }
}
