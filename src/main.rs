use quicksearch::quicksearch::QuickSearch;

use quicksearch::util::lines_from_file;

fn main() {
    let names = lines_from_file(r"..\fake_names.txt");

    println!("Building token index...");
    let qs = QuickSearch::new(&names);

    let name = "John Q. Peanut";
    println!("Searching for {name} in {} names", names.len());

    if let Some(results) = qs.find(name) {
        println!("{:#?}", &results[0..5]);
    } else {
        println!("No results for {}", name);
    }
}
