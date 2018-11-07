#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_tz;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

mod parser;

use clap::{App, AppSettings, Arg};
use parser::Parser;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::process;

fn run<P: Parser>(filename: Option<&str>, p: P) -> Result<bool, &str> {
    let stdin = io::stdin();

    let reader: Box<dyn BufRead> = match filename {
        Some(name) => {
            let file = File::open(name).expect("Error opening file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", p.parse(&content)),
            Err(err) => {
                eprintln!("{}: {}", "Exited while reading lines", err.description());
                return Err("Exited while reading lines");
            }
        }
    }

    return Ok(true);
}

fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about("tztail (TimeZoneTAIL) allows you to view logs in the timezone you want")
        .arg(Arg::with_name("FILE").help("File to tail"))
        .arg(
            Arg::with_name("timezone")
                .long("timezone")
                .short("t")
                .value_name("TIMEZONE")
                .required(true)
                .takes_value(true)
                .help("Sets the timezone in which output should be printed"),
        ).arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("FORMAT")
                .required(false)
                .takes_value(true)
                .help("Custom format for parsing dates"),
        );

    let matches = app.get_matches();
    let timezone = matches.value_of("timezone").expect("Please pass timezone");
    let custom_format = matches.value_of("format");
    let filename = matches.value_of("FILE");

    let result = match custom_format {
        Some(format) => run(filename, parser::new_fixedformatutcparser(timezone, format)),
        None => run(filename, parser::new_utcparser(timezone)),
    };

    match result {
        Err(error) => {
            eprintln!("{}: {}", "Exited non-successfully", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
