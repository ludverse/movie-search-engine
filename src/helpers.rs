use regex::Regex;

pub fn format_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .filter(|&c| !(c == '\'' || c == '.'))
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect()
}

pub fn tokenize_term(term: &String) -> Vec<String> {
    term.split_whitespace()
        .map(format_word)
        .collect::<Vec<_>>()
        .join(" ")
        .split(" ")
        .filter(|&word| word.to_string().trim() != "")
        .map(|word| stem(&word.to_string()))
        .collect()
}

pub fn stem(token: &String) -> String {
    let mut de_synonymized = token.to_string();

    let roman_numeral_re = Regex::new(r"^[mdclxvi]+$").unwrap();
    if roman_numeral_re.is_match(de_synonymized.as_str()) {
        let roman_numeral = roman::from(token.to_uppercase().as_str());
        if roman_numeral.is_some() {
            de_synonymized = roman_numeral.unwrap().to_string();
        }
    }

    porter_stemmer::stem(de_synonymized.as_str())
}

