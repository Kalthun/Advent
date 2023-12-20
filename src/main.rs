use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let file = File::open("Q4_input.txt")?;

    let reader = BufReader::new(file);

    

    Ok(())

}
