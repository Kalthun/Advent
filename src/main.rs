use std::fs::File;
use std::io::{BufRead, BufReader};

struct Map
{
    d: i64, // destination
    s: i64, // source
    r: i64, // source
}

fn main() -> std::io::Result<()>
{
    let file = File::open("Q5_input.txt")?;
    let mut reader = BufReader::new(file);

    // get the first line
    let mut seed_input = String::new();
    let _ = reader.read_line(&mut seed_input);

    // convert into Vec of strings
    let mut seed_list: Vec<&str> = seed_input.split_whitespace().collect();
    let _ = seed_list.remove(0);

    // convert into Vec of i64
    let mut seeds: Vec<i64> = Vec::new();
    for _ in seed_list.clone() {
        seeds.push(seed_list.remove(0).parse::<i64>().unwrap());
    }

    Ok(())
}
