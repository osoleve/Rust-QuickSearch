pub mod quicksearch;
pub mod string_sim;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::quicksearch::QuickSearch;
    use crate::string_sim::{jaro_winkler, ngram_jaccard};
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
            assert!(results[0].0.contains("Jenkins"));
            assert!(results.len() == 1)
        } else {
            panic!()
        }
    }
    #[test]
    fn jaro_winkler_works() {
        assert!((jaro_winkler("Trapeze", "Trace") * 100.0).round() == 85.0)
    }
    #[test]
    fn jaccard_works() {
        let a = HashSet::from([1, 2]);
        let b = HashSet::from([1, 2, 3]);

        assert!((jaccard_similarity(&a, &b) * 100.0).round() == 67.0);
    }
    #[test]
    fn ngram_jaccard_works() {
        let a = "abc";
        let b = "abc";

        assert!(ngram_jaccard(a, b, 1) == 1.0);
        assert!(ngram_jaccard(a, b, 2) == 1.0);
        assert!(ngram_jaccard(a, b, 3) == 1.0);

        let a = "abc";
        let b = "abcabc";
        assert!(ngram_jaccard(a, b, 1) == 1.0);
        assert!((ngram_jaccard(a, b, 2) * 100.0).round() == 67.0);
        assert!((ngram_jaccard(a, b, 3) * 100.0).round() == 33.0);
    }
}
