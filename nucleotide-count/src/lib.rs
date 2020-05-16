use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if let Some(invalid_char) = find_invalid_nucleotide(dna) {
        return Err(invalid_char);
    }
    if is_invalid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let result = dna.chars().fold(0usize, |mut acc, cur| {
        if cur == nucleotide {
            acc += 1
        };
        acc
    });
    Ok(result)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(invalid_char) = find_invalid_nucleotide(dna) {
        return Err(invalid_char);
    }

    let mut nucleotides_count = HashMap::with_capacity(4);
    VALID_NUCLEOTIDES.iter().for_each(|char| {
        nucleotides_count.insert(*char, 0usize);
    });

    dna.chars()
        .fold(&mut nucleotides_count, |nucleotides_count, char| {
            nucleotides_count.entry(char).and_modify(|prev| *prev += 1);
            nucleotides_count
        });

    Ok(nucleotides_count)
}

fn find_invalid_nucleotide(dna: &str) -> Option<char> {
    dna.chars().find(|char| !VALID_NUCLEOTIDES.contains(&char))
}

fn is_invalid_nucleotide(nucleotide: char) -> bool {
    !VALID_NUCLEOTIDES.contains(&nucleotide)
}
