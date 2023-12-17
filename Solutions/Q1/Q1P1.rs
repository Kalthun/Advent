use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let mut total: Vec<u32> = Vec::new();

    // Open the File for Reading
    let file = File::open("Q1_input.txt")?;

    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        total.push(get_line_number(line.unwrap()));
    }

    let answer:u32 = total.iter().sum();

    println!("answer is: {}", answer);

    Ok(())

}

fn get_line_number(line:String) -> u32
{

    let mut start = ' ';
    let mut end = ' ';

    for c in line.chars()
    {
        if c.is_numeric()
        {
            if start == ' '
            {
                start = c;
                end = c;
            }
            else
            {
                end = c;
            }
        }
    }

    return 10 * (start.to_digit(10).unwrap()) + (end.to_digit(10).unwrap());
}
