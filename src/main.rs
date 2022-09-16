fn main() {
    println!("Hello, world!");
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
    return (seq1.into_iter().collect(), seq2.into_iter().collect());
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
}