mod indexing;
mod helpers;

use indexing::{IndexEntry, create_index, load_index};
use helpers::tokenize_term;
use std::fs;
use std::io;
use std::io::Write;
use std::time::Instant;
use std::path::Path;

struct SearchResult {
    priority: i32,
    year: i32,
    entry_id: String
}

fn main() {
    let stop_words = fs::read_to_string("src/data/stop_words.json").expect("failed to read stop_words.json");
    let stop_words: serde_json::Value = serde_json::from_str(&stop_words).expect("failed to parse stop_words.json data");
    let stop_words = stop_words.as_array()
        .unwrap()
        .into_iter()
        .map(|word| String::from(word.as_str().unwrap()))
        .collect::<Vec<String>>();

    if !Path::new("src/data/index.json").exists() { create_index(&stop_words, false) }
    let index = load_index();

    loop {
        print!("search: ");
        io::stdout().flush().unwrap();

        let mut search = String::new();
        io::stdin()
            .read_line(&mut search)
            .expect("coudn't read line");
        let mut search = search.trim().to_string();

        let start = Instant::now();

        let mut results = get_results(&mut search, &index);

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

            results.sort_by(|a, b| {
                let order = b.priority.cmp(&a.priority);
                if order.is_eq() {
                    b.year.cmp(&a.year)
                } else {
                    order
                }
            });

            for result in results.iter().take(10) {
                let entry = index
                    .iter()
                    .find(|entry| entry.id == result.entry_id)
                    .unwrap();
                println!("{} {}", entry.year, entry.title);
            }
        }

        println!("");
    }
}

fn get_results(
    search: &str,
    index: &Vec<IndexEntry>
) -> Vec<SearchResult> {
    let search_words: Vec<_> = tokenize_term(&String::from(search));

    let mut results: Vec<SearchResult> = Vec::new();

    for entry in index.iter() {
        let mut entry_priority = 0;
        let mut used_words: Vec<String> = vec![];

        for token in entry.tokens.iter() {
            if used_words.contains(&token) { continue };

            let mut token_priority = 0;

            for search_word in search_words.iter() {
                if token == search_word {

                    let word_weight = 100; // it got empty here :(

                    token_priority += word_weight;
                    used_words.push(String::from(token));
                }
            }

            entry_priority += token_priority;
        }

        if entry_priority > 0 {
            results.push(SearchResult {
                priority: entry_priority,
                year: entry.year,
                entry_id: entry.id.clone(),
            });
        }
    }

    results
}

