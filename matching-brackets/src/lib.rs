pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for bracket in string.chars() {
        match bracket {
            '{' | '[' | '(' => stack.push(bracket),
            '}' | ']' | ')' => match stack.last() {
                None => stack.push(bracket),
                Some(last_element) => {
                    if bracket == get_oposite_bracket(*last_element) {
                        stack.pop();
                    }
                }
            },
            _ => continue,
        }
    }
    stack.is_empty()
}

fn get_oposite_bracket(bracket: char) -> char {
    match bracket {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => bracket,
    }
}
