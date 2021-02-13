extern crate clap;
use clap::{Arg, App};

pub fn run_cli() {
    let matches = App::new("Beat")
                    .author("Jack Glass <jackadrianglass@gmail.com>")
                    .about("Generates randomized drum beats and stickings")
                    .arg(Arg::with_name("bars")
                            .index(1)
                            .takes_value(true)
                            .help("Number of bars to generate")
                            .validator(|val| match val.parse::<i32>(){
                                Ok(val) => {if val < 1 {
                                    return Err(String::from("Bars must be greater than 1"));
                                }else {
                                    return Ok(())
                                }}
                                Err(_) => {return Err(String::from("Bars must be an integer"))}
                            }))
                    .arg(Arg::with_name("subdivision")
                            .default_value("16")
                            .short("s")
                            .long("subdivision"))
                    .get_matches();
    
    println!("{:?}", matches);
    let bars: usize = matches.value_of("bars").unwrap().parse().unwrap();

    let subdivision: usize = matches.value_of("subdivision").unwrap()
                                .parse().expect("Subdivision must be a positive integer");
    println!("{} {}", bars, subdivision);
}
