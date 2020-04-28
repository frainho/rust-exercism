use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter_map(|a| anagrams(a, word))
        .collect::<HashSet<_>>()
}

fn get_sorted_word(word: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort();
    word_chars
}

fn anagrams<'a>(a: &'a str, b: &str) -> Option<&'a str> {
    let lowercase_a = a.to_lowercase();
    let lowercase_b = b.to_lowercase();
    if lowercase_a == lowercase_b {
        return None;
    }
    if get_sorted_word(&lowercase_b) == get_sorted_word(&lowercase_a) {
        Some(a)
    } else {
        None
    }
}
