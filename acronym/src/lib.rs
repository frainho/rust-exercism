pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .flat_map(|word| {
            println!("word {}", word);
            if word.len() <= 4 {
                return word
                    .chars()
                    .filter(|c| !c.is_ascii_punctuation())
                    .take(1)
                    .collect::<Vec<char>>();
            }
            let mut chars = Vec::new();
            for (idx, char) in word.chars().enumerate() {
                if idx == 0usize || char.is_uppercase() {
                    chars.push(char)
                }
            }

            chars
        })
        .collect::<String>()
        .to_uppercase()
}
