use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let file = File::open("Q3_input.txt")?;

    let reader = BufReader::new(file);

    let mut sum:Vec<i32> = Vec::new();

    for line in reader.lines()
    {
        sum.push(get_points(line.unwrap()));
    }

    let answer:i32 = sum.iter().sum();

    Ok(())

}

fn get_points(line:String) -> i32
{
    todo!()
}

