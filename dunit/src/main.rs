use std::{io, env};
use clap::{Arg, App};

fn main() {
    let args = env::args();
    let matches = App::new("Dunit Unit Conversion Tool")
        .version("0.1")
        .author("Dheeraj Shenoy")
        .about("A simple command-line application for unit conversion")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .takes_value(true)
            .help("input string containing the unit conversion query"))
        .get_matches();

    if let Some(input) = matches.value_of("input") {
        println!("{}", input);
    }
}
