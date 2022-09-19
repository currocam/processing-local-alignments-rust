use processing_local_alignments;
use std::io;
use std::io::prelude::*;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    mode: String,
    // #[clap(parse(from_os_str))]
    // path: std::path::PathBuf,
}
fn main() ->io::Result<()>{
    let args = Cli::parse();
    let stdin = io::stdin();
    match args.mode.as_str(){
        "to_cig" => {
            let mut lines = stdin.lock().lines();
            while let Some(Ok(seq1)) = lines.next(){
                let unaligned1 = &seq1.chars().filter(|x| *x != '-').collect::<String>();
                print!("{}\t", unaligned1);
                let seq2 = &lines.next().unwrap().unwrap();
                let unaligned2 = &seq2.chars().filter(|x| *x != '-').collect::<String>();
                print!("{}\t", unaligned2);
                println!("{}", processing_local_alignments::edits_to_cigar(
                    &processing_local_alignments::edits(&seq1, &seq2)));
                lines.next();
            }              
                io::stdout().flush().unwrap();
        },
        "from_cig" => {
            for line in stdin.lock().lines() {
                let line = line.unwrap();
                let mut splits = line.split("\t");
                let (seq1, seq2) = processing_local_alignments::align(
                     splits.next().unwrap(),
                     splits.next().unwrap(),
                     &processing_local_alignments::cigar_to_edits(splits.next().unwrap())
                );
                println!("{}\n{}", seq1, seq2);
                io::stdout().flush().unwrap();

            }
        },
        _ => ()
    }
    Ok(())
}
