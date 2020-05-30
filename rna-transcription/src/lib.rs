#[derive(Debug, PartialEq)]
pub struct DNA(String);
const DNA_NUCLEOTIDES: &str = "ACGT";
#[derive(Debug, PartialEq)]
pub struct RNA(String);
const RNA_NUCLEOTIDES: &str = "ACGU";

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (idx, letter) in dna.chars().enumerate() {
            if !DNA_NUCLEOTIDES.contains(letter) {
                return Err(idx);
            } else {
                continue;
            }
        }
        Ok(DNA(dna.to_owned()))
    }

    pub fn into_rna(self) -> RNA {
        let converted_dna = self
            .0
            .chars()
            .map(|dna_letter| match dna_letter {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>();
        RNA::new(&converted_dna).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (idx, letter) in rna.chars().enumerate() {
            if !RNA_NUCLEOTIDES.contains(letter) {
                return Err(idx);
            } else {
                continue;
            }
        }
        Ok(RNA(rna.to_owned()))
    }
}
