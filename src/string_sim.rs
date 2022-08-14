use core::hash::Hash;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

#[must_use]
pub fn jaro(source: &str, target: &str) -> f64 {
    let source_chars: Vec<&str> = source.graphemes(true).collect();
    let target_chars: Vec<&str> = target.graphemes(true).collect();
    _jaro(&source_chars, &target_chars)
}

#[must_use]
fn _jaro(source_chars: &[&str], target_chars: &[&str]) -> f64 {
    // ok this is gonna be inefficient since it's a translation
    // of the python rosettacode entry but i'm trying
    // TODO rewrite
    let source_len = source_chars.len();
    let target_len = target_chars.len();

    let longer_len = max(source_len, target_len);

    if longer_len == 0 {
        return 1.0;
    }

    let match_distance = (longer_len / 2) - 1;

    let mut source_matches = vec![false; source_len];
    let mut target_matches = vec![false; target_len];
    let mut matches: i32 = 0;
    let mut transpositions: i32 = 0;

    for i in 0..source_len {
        let start = if match_distance > i {
            0
        } else {
            i - match_distance
        };
        let end = min(i + match_distance + 1, target_len);

        for j in start..end {
            if target_matches[j] | (source_chars[i] != target_chars[j]) {
                continue;
            }
            source_matches[i] = true;
            target_matches[j] = true;
            matches += 1;
            break;
        }
    }

    if matches == 0 {
        return 0.0;
    }

    let mut k = 0;
    for i in 0..source_len {
        if !source_matches[i] {
            continue;
        }
        while !target_matches[k] {
            k += 1;
        }
        if source_chars[i] != target_chars[k] {
            transpositions += 1;
        }
        k += 1;
    }

    let matches = f64::from(matches);
    let transpositions = f64::from(transpositions);

    ((matches / source_len as f64)
        + (matches / target_len as f64)
        + ((matches - transpositions / 2.0) / matches))
        / 3.0
}

#[must_use]
pub fn jaro_winkler(source: &str, target: &str) -> f64 {
    let source_chars: Vec<&str> = source.graphemes(true).collect();
    let target_chars: Vec<&str> = target.graphemes(true).collect();

    let mut prefix_match = 0.0;
    for i in 0..4 {
        if source_chars[i] != target_chars[i] {
            break;
        }
        prefix_match += 1.0;
    }
    let score = _jaro(&source_chars, &target_chars);
    score + 0.1 * prefix_match * (1.0 - score)
}

#[must_use]
fn ngram_jaccard(source: &str, target: &str, ngram_width: usize) -> f64 {
    let source_chars: Vec<&str> = source.graphemes(true).collect();
    let target_chars: Vec<&str> = target.graphemes(true).collect();

    let source_bigrams: HashSet<&[&str]> = source_chars.windows(ngram_width).collect();
    let target_bigrams: HashSet<&[&str]> = target_chars.windows(ngram_width).collect();

    jaccard_similarity(&source_bigrams, &target_bigrams)
}

#[must_use]
pub fn char_jaccard(source: &str, target: &str) -> f64 {
    ngram_jaccard(source, target, 1)
}

#[must_use]
pub fn bigram_jaccard(source: &str, target: &str) -> f64 {
    ngram_jaccard(source, target, 2)
}

#[must_use]
pub fn trigram_jaccard(source: &str, target: &str) -> f64 {
    ngram_jaccard(source, target, 3)
}

#[must_use]
fn jaccard_similarity<T>(a: &HashSet<T>, b: &HashSet<T>) -> f64
where
    T: Eq + Hash,
{
    let common_elements = a.intersection(b).count() as f64;
    let total_elements = a.union(b).count() as f64;

    common_elements / total_elements
}
