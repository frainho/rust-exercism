use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let lowercase_word = word.to_lowercase();

    let sorted_word = get_sorted_word(&lowercase_word);

    for possible_anagram_word in possible_anagrams {
        let possible_anagram_word_lowercase = possible_anagram_word.to_lowercase();
        if possible_anagram_word_lowercase == lowercase_word {
            continue;
        }
        let possible_anagram: Vec<char> = get_sorted_word(&possible_anagram_word_lowercase);
        
        if sorted_word == possible_anagram {
            anagrams.insert(*possible_anagram_word);
        }
    }

    anagrams
}

fn get_sorted_word(word: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort();
    word_chars
}