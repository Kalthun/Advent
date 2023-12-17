use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    // Open the File for Reading
    let file = File::open("Q2_input.txt")?;

    let reader = BufReader::new(file);

    let mut answer = 0;

    for game in reader.lines()
    {
        answer += get_power_set(game.unwrap().as_str());
    }

    print!("{}", answer);

    Ok(())

}

