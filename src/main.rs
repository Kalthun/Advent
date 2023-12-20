use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

fn main() -> std::io::Result<()>
{
    let file = File::open("Q7_input.txt")?;
    let reader = BufReader::new(file);



    Ok(())
}
