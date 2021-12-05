extern crate clap;
use clap::{App, Arg};

use crate::generator::{Grouping, TimeSignature, Structure};
use crate::generator::benny::{Filter, gen_groove};
use crate::display::ascii::display_groove;
use crate::display::gscribe::gen_groovescribe_link;

#[derive(Debug)]
struct Args {
    structure: Structure,
    filters: Vec<Filter>,
}

pub fn run_cli() {
    let args = parse_args();
    println!("{:?}", args);

    let groove = gen_groove(args.structure);
    display_groove(&groove);
    gen_groovescribe_link(&groove);
    
}

fn parse_args() -> Args {
    let matches = App::new("Beat")
        .author("Jack Glass <jackadrianglass@gmail.com>")
        .about("Generates randomized drum beats and stickings")
        .arg(
            Arg::with_name("bars")
                .short("b")
                .long("bars")
                .default_value("1")
                .help("Number of bars to generate")
                .validator(|val| match val.parse::<i32>() {
                    Ok(val) => {
                        if val < 1 {
                            return Err(String::from("Bars must be greater than 1"));
                        } else {
                            return Ok(());
                        }
                    }
                    Err(_) => return Err(String::from("Bars must be an integer")),
                }),
        )
        .arg(
            Arg::with_name("grouping")
                .default_value("4")
                .short("g")
                .long("grouping"),
        )
        .arg(
            Arg::with_name("time_signature")
                .default_value("4/4")
                .short("t")
                .long("time_signature"),
        )
        .arg(
            Arg::with_name("filters")
                .multiple(true)
                .default_value("H?")
        )
        .get_matches();

    let bars = matches.value_of("bars").unwrap().parse().unwrap();
    let grouping = Grouping::from_str(matches.value_of("grouping").unwrap())
        .expect("No matching grouping found");
    let time_signature =
        TimeSignature::from_str(matches.value_of("time_signature").unwrap()).unwrap();

    if time_signature.divisions != 4 {
        println!("Only supports quarter note subdivisions");
        std::process::exit(0);
    }

    let filters: Vec<_> = matches.values_of("filters").unwrap().collect();
    let filters = filters.iter().map(|val| Filter::from_str(val, &grouping).unwrap()).collect();

    Args{
        structure: Structure {
            time_signature,
            bars,
            grouping,
        },
        filters
    }
}
