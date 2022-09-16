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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_align() {
        assert_eq!(
            align("ACCACAGTCATA", "ACAGAGTACAAA", "MDMMMMMMIMMMM"),
            ("ACCACAGT-CATA".to_owned(), "A-CAGAGTACAAA".to_owned()));
    }
}