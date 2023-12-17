use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point
{
    r_index: i32,
    c_index: i32,
}

fn main() -> std::io::Result<()> {

    // Open the File for Reading
    let file = File::open("Q2_input.txt")?;

    let reader = BufReader::new(file);

    // need to track prev and next line

    // need to have a checker that when you hit a symbol (!char.is_numeric and != '.') then you check around it

    // create a Vector to store all numbers found and what indexes they are? (as tuples)

    // we can remove them if their position is too close

    // maybe create a Point class for fun


    Ok(())

}

