#![allow(clippy::cast_precision_loss)]
pub mod quicksearch;
pub mod string_sim;
pub mod util;

#[cfg(test)]
mod util_tests {
    use crate::util::{jaccard_similarity, lines_from_file};
    use float_cmp::approx_eq;
    use std::collections::HashSet;

    #[test]
    fn lines_from_file_works() {
        let names = lines_from_file(r"fake_names.txt");
        assert!(names.contains(&"John Dean".to_string()));
    }

    #[test]
    fn jaccard_similarity_works() {
        let a = HashSet::from([1, 2]);
        let b = HashSet::from([1, 2, 3]);

        assert!(
            approx_eq!(f64, jaccard_similarity(&a, &b), 0.667, epsilon = 0.001),
            "Algorithm is broken"
        );
        assert!(
            approx_eq!(
                f64,
                jaccard_similarity(&a, &b),
                jaccard_similarity(&b, &a),
                ulps = 5
            ),
            "Not symmetric within 5 units least precision"
        );
    }
}
#[cfg(test)]
mod quicksearch_tests {
    use crate::quicksearch::QuickSearch;
    use pretty_assertions::assert_eq;

    #[test]
    fn token_indexer_works() {
        let names = vec!["Nichole Jenkins".to_string(), "John Smith".to_string()];
        let qs = QuickSearch::new(&names);
        let name = "Nicky Jenkins";
        if let Some(results) = qs.find(name) {
            assert_eq!(results.len(), 1, "Basic indexing failed");
        }
        let name = "John Jenkins";
        if let Some(results) = qs.find(name) {
            assert_eq!(results.len(), 2, "Not all possible values returned");
        }
        let name = "Not Gonna Match";
        assert_eq!(
            qs.find(name),
            None,
            "Found a value where there should be None"
        );
    }

    #[test]
    fn ranking_works() {
        let names = vec!["Nichole Jenkins".to_string(), "J. Smith".to_string()];
        let qs = QuickSearch::new(&names);
        let name = "Nichole Smith";
        if let Some(results) = qs.find(name) {
            assert_eq!(results[0].0, "Nichole Jenkins");
        }
    }
}
#[cfg(test)]
mod string_sim_tests {
    use float_cmp::approx_eq;

    use crate::string_sim::{
        jaro, jaro_winkler, ngram_jaccard, symmetric_damerau_levenshtein_norm,
    };

    #[test]
    fn jaro_works() {
        let a = "Martha";
        let b = "Marhta";
        assert!(approx_eq!(f64, jaro(a, b), 0.944, epsilon = 0.001));
    }
    #[test]
    fn jaro_winkler_works() {
        let a = "Martha";
        let b = "Marhta";
        assert!(approx_eq!(f64, jaro_winkler(a, b), 0.961, epsilon = 0.001));
    }

    #[test]
    fn ngram_jaccard_works() {
        let a = "abc";
        let b = "abcabc";

        assert!(
            approx_eq!(f64, ngram_jaccard(a, b, 1), 1.0, epsilon = 0.001),
            "Width=1 failed"
        );

        assert!(
            approx_eq!(f64, ngram_jaccard(a, b, 2), 0.667, epsilon = 0.001),
            "Width=2 failed"
        );

        assert!(
            approx_eq!(f64, ngram_jaccard(a, b, 3), 0.333, epsilon = 0.001),
            "Width=3 failed"
        );
    }

    #[test]
    fn symmetric_damerau_levenshtein_norm_works() {
        let a = "Coast";
        let b = "Toast";
        assert!(
            approx_eq!(
                f64,
                symmetric_damerau_levenshtein_norm(a, b),
                0.8,
                epsilon = 0.001
            ),
            "Basic scoring failed"
        );
        let a = "Taco";
        let b = "Taco";
        assert!(
            approx_eq!(
                f64,
                symmetric_damerau_levenshtein_norm(a, b),
                1.0,
                epsilon = 0.001
            ),
            "Equality failed"
        );
        let a = "";
        let b = "Taco";
        assert!(
            approx_eq!(
                f64,
                symmetric_damerau_levenshtein_norm(a, b),
                0.0,
                epsilon = 0.001
            ),
            "Empty case failed"
        );
        let a = "Drat";
        let b = "Darth";
        assert!(
            approx_eq!(
                f64,
                symmetric_damerau_levenshtein_norm(a, b),
                0.6,
                epsilon = 0.001
            ),
            "Transposition failed"
        );

        assert!(
            approx_eq!(
                f64,
                symmetric_damerau_levenshtein_norm(a, b),
                symmetric_damerau_levenshtein_norm(b, a),
                ulps = 5
            ),
            "Not symmetric within 5 units least precision"
        );
    }
}
