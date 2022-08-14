use std::collections::HashMap;
use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;
use unidecode::unidecode;

use crate::string_sim::*;

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
                let set = token_index
                    .entry(token.to_string())
                    .or_insert_with(HashSet::<String>::new);
                set.insert(name.to_string());
            }
        }

        QuickSearch { token_index }
    }

    #[must_use]
    pub fn get_token_matches(&self, name: &str) -> Option<HashSet<&String>> {
        let mut matches = HashSet::<&String>::new();
        for token in Self::tokenize(name) {
            if let Some(data) = self.token_index.get(&token) {
                matches.extend(data);
            }
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    fn scorer(source: &str, target: &str) -> f64 {
        jaro_winkler(source, target)
        //ngram_jaccard(source, target, 3) //trigram jaccard
        //ngram_jaccard(source, target, 2) //bigram jaccard
    }

    /// # Panics
    ///
    /// Will panic if `scorer` yields an f64 that
    /// can't be compared against other f64s.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<Vec<(String, f64)>> {
        if let Some(names) = self.get_token_matches(name) {
            let mut results = names
                .iter()
                .map(|s| {
                    (
                        (*s).to_string(),
                        Self::scorer(&Self::normalize(name), &Self::normalize(s)),
                    )
                })
                .collect::<Vec<(String, f64)>>();
            results.sort_unstable_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
            Some(results)
        } else {
            None
        }
    }
}
