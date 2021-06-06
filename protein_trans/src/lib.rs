#![allow(warnings)]
use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static! {
    static ref CODONS_PROTEIN: HashMap<&'static str, &'static str> = [("AUG", "Methionine"),
    ("UUU", "Phenylalanine"),
    ("UUC", "Phenylalanine"),
    ("UUA", "Leucine"),
    ("UUG", "Leucine"),
    ("UCU", "Serine"),
    ("UCC", "Serine"),
    ("UCA", "Serine"),
    ("UCG", "Serine"),
    ("UAU", "Tyrosine"),
    ("UAC", "Tyrosine"),
    ("UGU", "Cysteine"),
    ("UGC", "Cysteine"),
    ("UGG", "Tryptophan"),
    ("UAA", "STOP"),
    ("UAG", "STOP"),
    ("UGA", "STOP"),
    ].iter().cloned().collect();

}

pub fn decode (rna: &str) -> Result<Vec<&str>, &str>
{
    if (rna.len() % 3 != 0 || rna.len() == 0){
        Err("wrong RNA length")
    }
    else{
            let mut index =0;
            let mut res_vector: Vec<&str> = Vec::new();
            while (index < rna.len()){
                let mut codon = &rna[index..index+3];
                index = index+3;
                let result = compare(&CODONS_PROTEIN, codon);
                match result{
                    Ok("STOP") => return Ok(res_vector),
                    Err(e) => return Err(e),
                    Ok(v) => res_vector.push(v),
                };

            }
        Ok(res_vector)
    }
}

pub fn compare<'a>(codons_protein: &HashMap<&'static str, &'static str>, codon: &'a str) -> Result<&'a str,&'a str>
{
    match codons_protein.get(&codon){
        Some(v) => Ok( v),
        None => Err("There is no such protein"),
    }

}
