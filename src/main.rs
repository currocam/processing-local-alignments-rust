use regex::Regex;
use std;

fn main() {
    split_pairs("Hello, world!");
}

fn align(x:&str, y:&str, edits:&str) -> (String, String) {
    let (mut x, mut y) = (x.chars(), y.chars());
    let mut seq1: Vec<char> = Vec::new();
    let mut seq2: Vec<char> = Vec::new();

    for edit in edits.chars(){
        match edit {
            'M' => {
                seq1.push(x.next().unwrap());
                seq2.push(y.next().unwrap());
            }   
            'D' =>{
                seq1.push(x.next().unwrap());
                seq2.push('-');
            }
            'I' =>{
                seq1.push('-');
                seq2.push(y.next().unwrap());
            }
            _ => panic!("Unexpected edit char was found")
        }
    }
    return (seq1.into_iter().collect(), seq2.into_iter().collect())
}

fn split_pairs(cigar: &str) -> Vec<(u64, char)>{
    let mut pairs = Vec::new();
    for cap in  Regex::new(r"(\d+)([^\d]+)").unwrap().captures_iter(cigar){
        pairs.push(
            (cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
             (cap.get(2).unwrap().as_str().chars().nth(0).unwrap()))
        )
    }
    return pairs
}

fn edits(x: &str, y:&str) -> String{
    let mut edits: Vec<char> = Vec::new();
    for position in x.chars().zip(y.chars()) {
        let (pos1, pos2) = position;
        if pos1 == '-' {
            edits.push('I');
        } else if pos2 == '-' {
        edits.push('D');
        } else {
         edits.push('M');
    }
}
    return edits.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_align() {
        assert_eq!(
            align("ACCACAGTCATA", "ACAGAGTACAAA", "MDMMMMMMIMMMM"),
            ("ACCACAGT-CATA".to_owned(), "A-CAGAGTACAAA".to_owned()));
    }    
    #[test]
    fn test_align_in_lowercase() {
        assert_eq!(
            align("accaaagta", "acaaatgtcca", "MDMMIMMMMIIM"),
            ("acca-aagt--a".to_owned(), "a-caaatgtcca".to_owned()));
    }
    #[test]
    fn test_align_with_empty_sequence() {
        assert_eq!(
            align("a", "", "D"),
            ("a".to_owned(), "-".to_owned()));
    }
    #[test]
    fn test_edits() {
        assert_eq!(
            edits("ACCACAGT-CATA", "A-CAGAGTACAAA"), "MDMMMMMMIMMMM".to_owned());
    }    
    #[test]
    fn test_edits_in_lowercase() {
        assert_eq!(
            edits("acca-aagt--a", "a-caaatgtcca"), "MDMMIMMMMIIM".to_owned());
    }
    #[test]
    fn test_edits_with_contiguos_gaps() {
        assert_eq!(
            edits("acgttcga", "aaa---aa"), "MMMDDDMM".to_owned());
    }
    #[test]
    fn test_split_pairs() {
        assert_eq!(
            split_pairs("1M1D6M1I4M"),
            vec![(1 as u64, 'M'), (1 as u64, 'D'), (6 as u64, 'M'), (1 as u64, 'I'), (4 as u64, 'M')]
        );
    }
}