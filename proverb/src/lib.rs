pub fn build_proverb(list: &[&str]) -> String {
    let mut words = list.iter().peekable();
    let mut proverb = String::new();

    while let Some(word) = words.next() {
        let next_word = match words.peek() {
            Some(next_word) => next_word,
            None => {
                proverb.push_str(&format!(
                    "And all for the want of a {}.",
                    list.get(0).unwrap()
                ));
                break;
            }
        };

        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            word, next_word
        ));
    }

    proverb
}
