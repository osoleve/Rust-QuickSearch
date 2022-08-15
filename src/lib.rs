pub mod quicksearch;
pub mod string_sim;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::quicksearch::QuickSearch;
    use crate::string_sim::jaro_winkler;
    use crate::util::lines_from_file;
    #[test]
    fn it_works() {
        let names = lines_from_file(r"fake_names.txt");
        let qs = QuickSearch::new(&names);
        let name = "John Q. Peanut";
        if let Some(results) = qs.find(name) {
            assert!(results[0].0.contains("John"));
        } else {
            panic!()
        }
    }
    #[test]
    fn jaro_winkler_works() {
        assert!((jaro_winkler("Trapeze", "Trace") * 1000.0).round() == 853.0)
    }
}
