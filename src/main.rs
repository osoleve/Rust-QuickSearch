use quicksearch::quicksearch::QuickSearch;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let names = lines_from_file(r"C:\Users\Andy\Dropbox\Programming\Rust\quicksearch\names.txt");

    println!("Building token index...");

    let qs = QuickSearch::new(&names);

    let name = "Joanna R. Smith";
    println!("Searching for {name} in {} names", names.len());

    let results = if let Some(results) = qs.find(name) {
        results
    } else {
        todo!()
    };

    println!("{:#?}", &results[0..5]);
}
