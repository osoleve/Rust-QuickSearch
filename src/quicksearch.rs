use std::collections::HashMap;
use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;
use unidecode::unidecode;

use crate::string_sim::jaro_winkler;

pub struct QuickSearch {
    token_index: HashMap<String, HashSet<String>>,
}

impl QuickSearch {
    fn normalize(s: &str) -> String {
        unidecode(s)
            .replace(&['.', '\''], "")
            .replace('-', " ")
            .to_lowercase()
    }

    fn tokenize(s: &str) -> HashSet<String> {
        s.unicode_words().map(Self::normalize).collect()
    }

    #[must_use]
    pub fn new(names: &Vec<String>) -> QuickSearch {
        let mut token_index = HashMap::<String, HashSet<String>>::new();

        for name in names {
            for token in Self::tokenize(name) {
                token_index
                    .entry(token.to_string())
                    .or_insert_with(HashSet::<String>::new)
                    .insert(name.to_string());
            }
        }

        QuickSearch { token_index }
    }

    pub fn get_token_matches(&self, name: &str) -> Option<HashSet<&String>> {
        let mut matches = HashSet::<&String>::new();
        let mut is_matched = false;
        for token in Self::tokenize(name) {
            if let Some(data) = self.token_index.get(&token) {
                matches.extend(data);
                is_matched |= true;
            }
        }

        if is_matched {
            Some(matches)
        } else {
            None
        }
    }

    fn score(source: &str, target: &str) -> f64 {
        jaro_winkler(source, target)
        //ngram_jaccard(source, target, 3) //trigram jaccard
        //ngram_jaccard(source, target, 2) //bigram jaccard
        //symmetric_damerau_levenshtein_norm(source, target)
    }

    /// # Panics
    ///
    /// Will panic if `Self::score` yields an f64 that
    /// can't be compared against other f64s.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<Vec<(String, f64)>> {
        let mut results = self
            .get_token_matches(name)?
            .iter()
            .map(|s| {
                (
                    (*s).to_string(),
                    Self::score(&Self::normalize(name), &Self::normalize(s)),
                )
            })
            .collect::<Vec<(String, f64)>>();
        results.sort_unstable_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        Some(results)
    }
}
