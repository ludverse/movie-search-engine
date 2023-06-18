use porter_stemmer::stem;
use std::fs;
use std::io;
use std::io::Write;
use std::time::Instant;
use roman;

struct SearchResult {
    priority: i32,
    entry_id: String
}

struct Entry {
    title: String,
    year: u64,
    id: String,
}

fn main() {
    let stop_words = fs::read_to_string("src/data/stop_words.json")
        .expect("couldn't read file");
    let stop_words: serde_json::Value =
        serde_json::from_str(&stop_words).expect("couldn't parse JSON data");
    let stop_words = stop_words
        .as_array()
        .unwrap()
        .into_iter()
        .map(|word| String::from(word.as_str().unwrap()))
        .collect::<Vec<String>>();

    loop {
        print!("search: ");
        io::stdout().flush().unwrap();

        let mut search = String::new();
        io::stdin()
            .read_line(&mut search)
            .expect("coudn't read line");
        let mut search = String::from(search.trim());

        let entries = load_entries();

        let start = Instant::now();
        let mut results = get_results(&mut search, &entries, &stop_words);
        let end = Instant::now();
        let elapsed = end - start;

        if results.is_empty() {
            println!("no results found by \"{}\"", search);
        } else {
            println!(
                "searched for \"{}\" in {} seconds",
                search,
                elapsed.as_secs_f64()
            );
            println!("");

            results.sort_by(|a, b| b.priority.cmp(&a.priority));

            for result in results.iter().take(10) {
                let entry = entries
                    .iter()
                    .find(|entry| entry.id == result.entry_id)
                    .unwrap();
                println!("{} {}", entry.year, entry.title);
            }
        }

        println!("");
    }
}

fn format_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .filter(|c| c != &'\'')
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect()
}

fn tokenize_term(term: &String) -> Vec<String> {
    term.split_whitespace()
        .map(format_word)
        .collect::<Vec<_>>()
        .join(" ")
        .split(" ")
        .filter(|&word| String::from(word).trim() != "")
        .map(|word| stem(word))
        .collect()
}

fn get_synonyms(token: &String) -> Vec<String> {
    let mut synonyms: Vec<_> = vec![String::from(token)];

    if let Ok(number) = token.parse::<i32>() {
        synonyms.push(roman::to(number).unwrap().to_lowercase());
    }

    synonyms
}

fn load_entries() -> Vec<Entry> {
    let data = fs::read_to_string("src/data/movies.json").expect("couldn't read file");
    let data: serde_json::Value = serde_json::from_str(&data).expect("couldn't parse JSON data");

    let mut entries: Vec<Entry> = vec![];

    for entry in data.as_array().unwrap().iter() {
        let entry = entry.as_object().unwrap();
        entries.push(Entry {
            title: String::from(entry["title"].as_str().unwrap()),
            year: entry["year"].as_u64().unwrap(),
            id: String::from(entry["title"].as_str().unwrap()),
        });
    }

    entries
}

fn get_results(
    search: &str,
    entries: &Vec<Entry>,
    stop_words: &Vec<String>
) -> Vec<SearchResult> {
    let search_words: Vec<_> = tokenize_term(&String::from(search));

    let mut results: Vec<SearchResult> = Vec::new();

    for entry in entries.iter() {
        let mut entry_priority = 0;
        let mut used_words: Vec<String> = vec![];

        for token in tokenize_term(&entry.title).iter() {
            if used_words.contains(token) { continue };

            let mut token_priority = 0;

            for search_word in search_words.iter() {
                for search_synonym in get_synonyms(&search_word) {
                    if token.contains(&search_synonym) {

                        let mut word_weight = 40;
                        if token == &search_synonym {
                            word_weight = 100;
                        }
                        if stop_words.contains(&search_synonym) {
                            word_weight = 10;
                        }
                        if &search_synonym != search_word {
                            word_weight /= 2;
                        }
                        
                        token_priority += word_weight;
                        used_words.push(String::from(token));
                    }
                }
            }

            entry_priority += token_priority;
        }

        if entry_priority > 0 {
            results.push(SearchResult {
                priority: entry_priority,
                entry_id: entry.id.clone(),
            });
        }
    }

    results
}
