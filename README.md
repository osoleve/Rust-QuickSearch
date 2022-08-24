# Rust-QuickSearch
Yet Another In-Memory Name Search Engine

A blazing fast proper name search engine that scales well to anything your memory can hold.
Uses token indexing as a pre-clustering heuristic, so it might work for things other than names but no promises.
Essentially a Rust port of https://github.com/osoleve/Name-QuickSearch

Uses Jaro-Winkler to score, but also includes implementations of Jaro, ngram Jaccard, and the symmetric Damerau-Levenshtein norm that can be used.

Interface is a WIP but it works.

## Usage
Feed `QuickSearch::new()` a list of names (`&str`), then use `find()` to search in those names!

## Example
Running the crate should yield the following (approximately, since sort is unstable):
```
Building token index...
Searching for John Q. Peanut in 100000 names
[
    (
        "John Dean",
        0.9008547008547009,
    ),
    (
        "John Bean",
        0.9008547008547009,
    ),
    (
        "John Cantu",
        0.8962393162393162,
    ),
    (
        "John Eaton",
        0.8962393162393162,
    ),
    (
        "John Pearson",
        0.8884615384615385,
    ),
]
```
