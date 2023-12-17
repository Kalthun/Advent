use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    // Open the File for Reading
    let file = File::open("Q2_input.txt")?;

    let reader = BufReader::new(file);




    Ok(())

}

