use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{prelude::*, BufReader},
    path::Path,
};

use unicode_segmentation::UnicodeSegmentation;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
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

pub fn string_profile(s: &str) -> String {
    let chars = s.graphemes(true).collect::<Vec<&str>>();
    let mut char_map = HashMap::<&str, usize>::new();
    let mut profile = vec![];
    let mut i = 0;

    for c in chars {
        if let Some(index) = char_map.get(c) {
            profile.push(format!("{}", *index))
        } else {
            char_map.insert(c, i);
            profile.push(format!("{}", i));
            i += 1;
        }
    }
    profile.join("")
}
