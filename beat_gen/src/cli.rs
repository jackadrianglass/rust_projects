extern crate clap;
use clap::{App, Arg};

use crate::generator::benny::{gen_funk_groove, gen_jazz_groove};
use crate::display::ascii::display_groove;
use crate::display::gscribe::gen_groovescribe_link;

#[derive(Debug)]
struct Args {
    bars: i32, 
    style: String,
    // filters: Vec<Filter>,
}

pub fn run_cli() {
    let args = parse_args();

    let groove = match args.style.as_str() {
        "funk" => gen_funk_groove(args.bars),
        "jazz" => gen_jazz_groove(args.bars),
        _ => panic!("Alien style here!"),
    };
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
                            Err(String::from("Bars must be greater than 1"))
                        } else {
                            Ok(())
                        }
                    }
                    Err(_) => return Err(String::from("Bars must be an integer")),
                }),
        )
        .arg(
            Arg::with_name("style")
                .default_value("funk")
                .short("s")
                .long("style")
                .validator(|val| match val.as_str() {
                    "funk" => return Ok(()),
                    "jazz" => return Ok(()),
                    _ => return Err(String::from("Only funk and jazz my good sir")),
                }),
        )
        .arg(
            Arg::with_name("filters")
                .multiple(true)
                .default_value("H?")
        )
        .get_matches();

    let bars = matches.value_of("bars").unwrap().parse().unwrap();
    let style = matches.value_of("style").unwrap();
    // let filters: Vec<_> = matches.values_of("filters").unwrap().collect();
    // let filters = filters.iter().map(|val| Filter::from_str(val, &grouping).unwrap()).collect();

    Args{
        bars,
        style: style.to_string(),
        // filters
    }
}
