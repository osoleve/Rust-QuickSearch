pub mod quicksearch;
pub mod string_sim;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::quicksearch::QuickSearch;
    use crate::string_sim::jaro_winkler;
    use crate::util::lines_from_file;
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
        assert!((jaro_winkler("Trapeze", "Trace") * 1000.0).round() == 853.0)
    }
}
