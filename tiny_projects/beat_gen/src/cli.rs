extern crate clap;
use clap::{App, Arg};

use crate::display::ascii::display_groove;
use crate::display::gscribe::gen_groovescribe_link;
use crate::generator::{
    benny::gen_groove,
    GenerationType, Style,
};

#[derive(Debug)]
struct Args {
    bars: i32,
    style: Style,
    // filters: Vec<Filter>,
    gen_type: GenerationType,
    open_link: bool,
}

pub fn run_cli() {
    let args = parse_args();

    let groove = gen_groove(&args.style, args.bars, &args.gen_type);
    display_groove(&groove);
    let url = gen_groovescribe_link(&groove);
    println!("{}", url);
    if args.open_link {
        opener::open_browser(url).unwrap();
    }
}

fn parse_args() -> Args {
    let matches = App::new("Beat")
        .author("Jack Glass <jackadrianglass@gmail.com>")
        .about("Generates randomized drum beats and stickings")
        .arg(
            Arg::with_name("open_link")
                .short("o")
                .long("open")
                .default_value("false")
            )
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
                .validator(|val| {
                    if let Ok(_) = Style::from_str(&val) {
                        Ok(())
                    } else {
                        Err(String::from("Only funk and jazz my good sir"))
                    }
                }),
        )
        .arg(Arg::with_name("filters").multiple(true).default_value("H?"))
        .arg(
            Arg::with_name("gen_type")
                .default_value("letters")
                .short("g")
                .long("gen_type")
                .validator(|val| {
                    if let Ok(_) = GenerationType::from_str(&val) {
                        Ok(())
                    } else {
                        Err(String::from("Only letters or odd my good sir"))
                    }
                }),
        )
        .get_matches();

    let bars = matches.value_of("bars").unwrap().parse().unwrap();
    let style = Style::from_str(matches.value_of("style").unwrap()).unwrap();
    let gen_type = GenerationType::from_str(&matches.value_of("gen_type").unwrap()).unwrap();
    let open_link = matches.value_of("open_link").unwrap().parse().unwrap();
    // let filters: Vec<_> = matches.values_of("filters").unwrap().collect();
    // let filters = filters.iter().map(|val| Filter::from_str(val, &grouping).unwrap()).collect();

    Args {
        bars,
        style,
        // filters
        gen_type,
        open_link,
    }
}
