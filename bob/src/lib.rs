pub fn reply(message: &str) -> &str {
    if is_question(message) {
        if has_letters(message) && no_lowercase_letters(message) {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }
    if is_whitespace(message) {
        return "Fine. Be that way!";
    }
    if has_letters(message) && no_lowercase_letters(message) {
        return "Whoa, chill out!";
    }
    "Whatever."
}

fn is_question(message: &str) -> bool {
    match message.trim().chars().last() {
        Some('?') => true,
        _ => false,
    }
}

fn has_letters(message: &str) -> bool {
    message.chars().any(|char| char.is_alphabetic())
}

fn no_lowercase_letters(message: &str) -> bool {
    message.chars().all(|char| match char {
        'a'..='z' => false,
        _ => true,
    })
}

fn is_whitespace(message: &str) -> bool {
    message.chars().all(|char| char.is_whitespace())
}
