use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()>
{
    let file = File::open("Q6_input.txt")?;
    let mut reader = BufReader::new(file);

    // get the first line
    let mut time = String::new();
    let _ = reader.read_line(&mut time);
    let time_split:Vec<&str> = time.split_whitespace().skip(1).collect();
    let temp1:i64 = time_split.join("").parse::<i64>().unwrap();

    let mut dist = String::new();
    let _ = reader.read_line(&mut dist);
    let dist_split:Vec<&str> = dist.split_whitespace().skip(1).collect();
    let temp2:i64 = dist_split.join("").parse::<i64>().unwrap();

    let answer:i64 = compute_valid((temp1, temp2));

    println!("answer: {}", answer);

    Ok(())
}

fn compute_distance(speed:i64, time:i64) -> i64
{
    return speed * time;
}

fn compute_valid(race:(i64,i64)) -> i64
{
    let mut counter = 0;
    for hold_time in 1..race.0
    {
        if compute_distance(hold_time, race.0 - hold_time) > race.1
        {
            counter += 1;
        }
    }
    return counter;
}
