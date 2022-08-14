use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[must_use]
pub fn jaccard_similarity<T>(a: &HashSet<T>, b: &HashSet<T>) -> f64
where
    T: Eq + Hash,
{
    let common_elements = a.intersection(b).count() as f64;
    let total_elements = a.union(b).count() as f64;

    common_elements / total_elements
}
