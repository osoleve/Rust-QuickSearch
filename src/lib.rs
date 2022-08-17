#![allow(clippy::cast_precision_loss)]
pub mod quicksearch;
pub mod string_sim;
pub mod util;

#[cfg(test)]
mod tests {
    const ERROR_MARGIN: f64 = 0.001;

    use crate::quicksearch::QuickSearch;
    use crate::string_sim::{
        jaro, jaro_winkler, ngram_jaccard, symmetric_damerau_levenshtein_norm,
    };
    use crate::util::{jaccard_similarity, lines_from_file};
    use std::collections::HashSet;
    #[test]
    fn lines_from_file_works() {
        let names = lines_from_file(r"fake_names.txt");
        assert!(names.contains(&"John Dean".to_string()));
    }
    #[test]
    fn token_indexer_works() {
        let names = vec!["Nichole Jenkins".to_string(), "John Smith".to_string()];
        let qs = QuickSearch::new(&names);
        let name = "Nicky Jenkins";
        if let Some(results) = qs.find(name) {
            assert!(results.len() == 1);
        }
        let name = "John Jenkins";
        if let Some(results) = qs.find(name) {
            assert!(results.len() == 2);
        }
        let name = "Not Gonna Match";
        assert!(qs.find(name) == None);
    }
    #[test]
    fn jaro_works() {
        assert!((jaro("MARTHA", "MARHTA") - 0.944).abs() < ERROR_MARGIN);
    }
    #[test]
    fn jaro_winkler_works() {
        assert!((jaro_winkler("Crave", "Crate") - 0.907).abs() < ERROR_MARGIN);
    }
    #[test]
    fn jaccard_works() {
        let a = HashSet::from([1, 2]);
        let b = HashSet::from([1, 2, 3]);

        assert!((jaccard_similarity(&a, &b) - 0.667).abs() < ERROR_MARGIN);
    }
    #[test]
    fn ngram_jaccard_works() {
        let a = "abc";
        let b = "abc";

        assert!(ngram_jaccard(a, b, 1).abs() - 1.0 < ERROR_MARGIN);
        assert!(ngram_jaccard(a, b, 2).abs() - 1.0 < ERROR_MARGIN);
        assert!(ngram_jaccard(a, b, 3).abs() - 1.0 < ERROR_MARGIN);

        let a = "abc";
        let b = "abcabc";
        assert!(ngram_jaccard(a, b, 1).abs() - 1.000 < ERROR_MARGIN);
        assert!((ngram_jaccard(a, b, 2) - 0.667).abs() < ERROR_MARGIN);
        assert!((ngram_jaccard(a, b, 3) - 0.333).abs() < ERROR_MARGIN);
    }
    #[test]
    fn ranking_works() {
        let names = vec!["Nichole Jenkins".to_string(), "J. Smith".to_string()];
        let qs = QuickSearch::new(&names);
        let name = "Nichole Smith";
        if let Some(results) = qs.find(name) {
            assert!(results[0].0.contains("Jenkins"));
        }
    }
    #[test]
    fn symmetric_damerau_levenshtein_norm_works() {
        let a = "Coast";
        let b = "Toast";
        assert!(symmetric_damerau_levenshtein_norm(a, b).abs() - 0.800 < ERROR_MARGIN);
        let a = "Taco";
        let b = "Taco";
        assert!(symmetric_damerau_levenshtein_norm(a, b).abs() - 1.000 < ERROR_MARGIN);
        let a = "";
        let b = "Taco";
        assert!(symmetric_damerau_levenshtein_norm(a, b).abs() - 0.000 < ERROR_MARGIN);
        let a = "Drat";
        let b = "Darth";
        assert!(symmetric_damerau_levenshtein_norm(a, b).abs() - 0.600 < ERROR_MARGIN);
    }
}
