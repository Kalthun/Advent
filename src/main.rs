use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Point
{
    left:String,
    right:String,
}

fn main() -> std::io::Result<()>
{
    let file = File::open("ex.txt")?;
    let mut reader = BufReader::new(file);


   

    Ok(())
}
