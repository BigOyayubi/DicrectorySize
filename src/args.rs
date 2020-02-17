use std::env;
use std::ffi::{OsString};
use std::process;
use std::io::{self, Write};

use clap::{self};
#[macro_use]
use log;

use crate::app;
use crate::logger::Logger;
use crate::Result;

pub struct Args{
    pub depth: i32,
    pub kilo: bool,
    pub include_pattern: String,
    pub exclude_pattern: String,
    pub dir_path: String,
}

impl Args{
    pub fn parse() -> Result<Args> {
        let parsed = app_parse(env::args_os())?;

        if let Err(err) = Logger::init() {
            return Err(format!("failed to initialize Logger: {}", err).into());
        }
        if parsed.is_present("trace") {
            log::set_max_level(log::LevelFilter::Trace);
        } else if parsed.is_present("debug") {
            log::set_max_level(log::LevelFilter::Debug);
        }else{
            log::set_max_level(log::LevelFilter::Warn);
        }


        let path = match parsed.value_of("path") {
            Some(p) => p,
            None => return Err("path must be set.".into()),
        };

        let kilo = parsed.is_present("kilo");

        let depth = match parsed.value_of("depth") {
            Some(p) => p,
            None => "0",
        };

        let include_pattern = match parsed.value_of("include") {
            Some(p) => p,
            None => "",
        };


        let exclude_pattern = match parsed.value_of("exclude") {
            Some(p) => p,
            None => "",
        };

        Ok(Args{
            depth: depth.parse::<i32>().unwrap(),
            kilo: kilo,
            include_pattern: include_pattern.to_string(),
            exclude_pattern: exclude_pattern.to_string(),
            dir_path: path.to_string(),
        })
    }
}

fn app_parse<I,T>(
    args: I,
) -> Result<clap::ArgMatches<'static>>
where I: IntoIterator<Item=T>,
      T: Into<OsString> + Clone
{
    let err = match app::app().get_matches_from_safe(args) {
        Ok(matches) => return Ok(matches),
        Err(err) => err,
    };

    if err.use_stderr() {
        return Err(err.into());
    }

    let _ = write!(io::stdout(), "{}", err);
    process::exit(0);
}