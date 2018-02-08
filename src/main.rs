extern crate clap;
extern crate glob;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{Arg, App, SubCommand};

mod list;
mod file_data;

fn main() {
    let matches = App::new("rfil")
        .version("0.1")
        .author("Tammo Ippen <tammo.ippen@posteo.de>")
        .about("fileutils with jsonl output.")
        .subcommand(SubCommand::with_name("ls")
            .about("List files in the given directory.")
            .arg(Arg::with_name("a")
                .short("a")
                .help("Display all files."))
            .arg(Arg::with_name("r")
                .short("r")
                .help("Recursively visit all directories."))
            .arg(Arg::with_name("GLOB")
                   .help("files directories or glob")
                   .multiple(true)
                   .index(1)))
        .get_matches();

    // println!("{:?}", matches);
    if let Some(ls) = matches.subcommand_matches("ls") {
        let all = ls.is_present("a");
        let recursive = ls.is_present("r");
        if ls.is_present("GLOB") {
            for glob in ls.values_of("GLOB").unwrap() {
                list::list(&glob, all, recursive);
            }
        } else {
            list::list("*", all, recursive);
        }
    }
}
