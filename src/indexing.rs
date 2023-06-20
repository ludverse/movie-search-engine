use crate::helpers::tokenize_term;
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct IndexEntry {
    pub id: String,
    pub tokens: Vec<String>,
    pub title: String,
    pub year: i32,
    pub extract: Option<String>
}

pub fn create_index(stop_words: &Vec<String>, silent: bool) {
    if !silent {
        println!("indexing entries for you... please hang on tight")
    }

    let entries = fs::read_to_string("src/data/movies.json").expect("failed to read movies.json");
    let entries: serde_json::Value = serde_json::from_str(&entries).expect("failed to parse movies.json data");
    let entries = entries.as_array().expect("failed to parse movies.json data");

    let mut index: Vec<IndexEntry> = vec![];

    for (_i, entry) in entries.iter().enumerate() {
        let entry_title = entry["title"].as_str().unwrap().to_string();

        index.push(IndexEntry {
            id: Uuid::new_v4().to_string(),
            tokens: tokenize_term(&entry_title).iter()
                .filter(|token| !stop_words.contains(token))
                .map(|token| token.to_string())
                .collect::<Vec<_>>(),
            title: entry_title,
            year: entry["year"].as_i64().unwrap() as i32,
            extract: entry.get("extract").and_then(|extract| extract.as_str()).map(|extract| extract.to_string())
        });
    }

    let json_string = serde_json::to_string(&index).expect("failed to serialize index");

    let mut file = File::create("src/data/index.json").expect("failed to create index.json");
    file.write_all(json_string.as_bytes()).expect("failed to write to index.json");

    if !silent {
        println!("index saved to index.json and ready!");
    }
}


pub fn load_index() -> Vec<IndexEntry> {
    let file = fs::read_to_string("src/data/index.json").expect("failed to read index.json");
    let index: serde_json::Value = serde_json::from_str(&file).expect("failed to parse index.json data");
    let index = index.as_array().expect("failed to parse index.json data");

    let mut entries: Vec<IndexEntry> = vec![];

    for entry in index.iter() {
        let entry = entry.as_object().unwrap();
        entries.push(IndexEntry {
            id: entry["id"].as_str().unwrap().to_string(),
            tokens: entry["tokens"].as_array().unwrap().iter().map(|word| word.as_str().unwrap().to_string()).collect::<Vec<String>>(),
            title: entry["title"].as_str().unwrap().to_string(),
            year: entry["year"].as_i64().unwrap() as i32,
            extract: entry.get("extract").and_then(|extract| extract.as_str()).map(|extract| extract.to_string())
        });
    }

    entries
}

