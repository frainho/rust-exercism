use std::collections::BTreeMap;

pub fn transform(original_scores: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    original_scores
        .iter()
        .flat_map(|(points, chars)| {
            chars
                .iter()
                .map(move |single_char| (single_char.to_ascii_lowercase(), *points))
        })
        .collect()
}
