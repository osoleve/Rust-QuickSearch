# Rust-QuickSearch
Yet Another In-Memory Name Search Engine

A blazing fast proper name search engine that scales well to anything your memory can hold. 
Uses token indexing as a pre-clustering heuristic, so it might work for things other than names but no promises.

Uses Jaro-Winkler to score, but also includes an implementation of ngram Jaccard that can be used.

## Usage
Feed `QuickSearch::new()` a list of names (`&str`), then use `find()` to search in those names!

## Example
Running the crate should yield the following:
```
Building token index...
Searching for Spencer Q. Peanut in 100000 names
[
    (
        "Spencer Kent",
        0.9208333333333333,
    ),
    (
        "Spencer Evans",
        0.9067307692307692,
    ),
    (
        "Spencer Tran",
        0.893560606060606,
    ),
    (
        "Spencer Carpenter",
        0.8923642533936651,
    ),
    (
        "Spencer Leon",
        0.8916666666666667,
    ),
]
```
