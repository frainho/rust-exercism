use itertools::Itertools;

const CHUNK_SIZE: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter_map(get_char_at_reverse_position)
        .chunks(CHUNK_SIZE)
        .into_iter()
        .map(|chunk| chunk.format(""))
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(get_char_at_reverse_position)
        .collect()
}

fn get_char_at_reverse_position(char: char) -> Option<char> {
    match char {
        '1'..='9' => Some(char),
        'a'..='z' => {
            let reverse_char = b'a' + b'z' - (char as u8);
            Some(reverse_char as char)
        }
        _ => None,
    }
}
