use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use crate::util::jaccard_similarity;

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
            k += 1
        }
        if source_chars[i] != target_chars[k] {
            transpositions += 1;
        }
        k += 1
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
pub fn ngram_jaccard(source: &str, target: &str, ngram_width: usize) -> f64 {
    let source_chars: Vec<&str> = source.graphemes(true).collect();
    let target_chars: Vec<&str> = target.graphemes(true).collect();

    let source_bigrams: HashSet<&[&str]> = source_chars.windows(ngram_width).collect();
    let target_bigrams: HashSet<&[&str]> = target_chars.windows(ngram_width).collect();

    jaccard_similarity(&source_bigrams, &target_bigrams)
}

fn damerau_levenshtein(source_chars: &[&str], target_chars: &[&str]) -> usize {
    let source_len = source_chars.len() + 1;
    let target_len = target_chars.len() + 1;

    if min(source_len, target_len) == 0 {
        return max(source_len, target_len);
    }

    let mut dl_matrix = vec![vec![0; target_len]; source_len];

    for i in 1..source_len {
        dl_matrix[i][0] = i
    }
    for j in 1..target_len {
        dl_matrix[0][j] = j
    }

    for j in 1..target_len {
        for i in 1..source_len {
            dl_matrix[i][j] = if source_chars[i - 1] == target_chars[j - 1] {
                dl_matrix[i - 1][j - 1]
            } else {
                let delete = dl_matrix[i - 1][j];
                let insert = dl_matrix[i][j - 1];
                let substitute = dl_matrix[i - 1][j - 1];

                1 + min(delete, min(insert, substitute))
            };
            if i > 1 && j > 1 {
                if source_chars[i - 1] == target_chars[j - 2]
                    && source_chars[i - 2] == target_chars[j - 1]
                {
                    let temp = dl_matrix[i][j];
                    if temp > dl_matrix[i - 2][j - 2] + 1 {
                        dl_matrix[i][j] = dl_matrix[i - 2][j - 2] + 1
                    }
                }
            }
        }
    }

    dl_matrix[source_len - 1][target_len - 1]
}

pub fn symmetric_damerau_levenshtein_norm(source: &str, target: &str) -> f64 {
    let source_chars: Vec<&str> = source.graphemes(true).collect();
    let target_chars: Vec<&str> = target.graphemes(true).collect();

    let longer_len = max(source_chars.len(), target_chars.len());

    1.0 - damerau_levenshtein(&source_chars, &target_chars) as f64 / longer_len as f64
}
