extern crate rand;
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("d_roll <# dice to roll>d<N sided die>");
    }

    for arg in args.iter() {
        let words: Vec<&str> = arg.split('d').collect();
        if words.len() != 2 {
            continue;
        }

        let num = words[0].parse::<i32>().unwrap();
        let die = words[1].parse::<i32>().unwrap();

        for roll in 0..num {
            println!("Roll {}d{} is {}", roll + 1, die, rand::thread_rng().gen_range(1, die));
        }
        println!("---");
    }
}
