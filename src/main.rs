use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()>
{
    let file = File::open("Q5_input.txt")?;
    let mut reader = BufReader::new(file);

    // get the first line
    let mut time = String::new();
    let _ = reader.read_line(&mut time);

    // get second line
    let mut distance = String::new();
    let _ = reader.read_line(&mut distance);



    Ok(())
}
