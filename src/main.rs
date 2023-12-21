use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

fn main() -> std::io::Result<()>
{
    let file = File::open("Q8_input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer);

    let directions:Vec<char> = buffer.chars().filter(|c| *c == 'R' || *c == 'L').collect();

    let _ = reader.read_line(&mut buffer);

    println!("directions:{:?}", directions);

    for line in reader.lines()
    {
    }

    Ok(())
}
