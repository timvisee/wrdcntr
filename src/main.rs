extern crate clap;
extern crate rayon;

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{Arg, App};
use rayon::prelude::*;

/// A sorted and unsorted map type
pub type U<'a> = HashMap<&'a str, usize>;
pub type S<'a> = BTreeMap<&'a str, usize>;

/// Application entrypoint.
fn main() {
    // Handle CLI arguments
    let matches = App::new("wrdcntr")
        .version("0.1")
        .author("Tim Visee <timvisee@gmail.com>")
        .about("Counts words, FAST!")
        .arg(Arg::with_name("FILE")
            .help("The file to count words in")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("stats")
            .long("stats")
            .short("s")
            .help("Show some extra word counting stats"))
        .arg(Arg::with_name("no-output")
            .long("no-output")
            .short("n")
            .help("Do not print the result"))
        .get_matches();

    // Open the file and collect all it's lines
    let path = matches.value_of("FILE").expect("no file selected");
    let file = BufReader::new(File::open(path).expect("failed to open file"));
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();

    // Count words: iterate lines, split words, fold count, reduce and sort
    let result: S = lines.par_iter()
        .map(|line|
            line.split_terminator(|c: char| !c.is_alphanumeric())
                .fold(U::new(), fold_wc)
        )
        .reduce(|| U::new(), reduce_map)
        .into_iter()
        .collect();

    // Print the results, unless specified otherwise
    if !matches.is_present("no-output") {
        result.iter().for_each(|(w, c)| println!("{}: {}", w, c));
    }

    // Print some extra word counting stats
    if matches.is_present("stats") {
        let total: usize = result.par_iter().map(|(_, c)| c).sum();
        println!("> unique words: {}", result.len());
        println!("> total words: {}", total);
    }
}

/// Count a word and put it in a map.
fn fold_wc<'a>(mut map: U<'a>, word: &'a str) -> U<'a> {
    {
        *map.entry(word).or_insert(0) += 1
    }
    map
}

/// Reduce and sum a map.
fn reduce_map<'a>(mut map: U<'a>, other: U<'a>) -> U<'a> {
    other.iter().for_each(|(key, value)|
        *map.entry(key).or_insert(0) += value
    );
    map
}
