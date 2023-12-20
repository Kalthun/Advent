use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()>
{
    let file = File::open("Q6_input.txt")?;
    let mut reader = BufReader::new(file);

    // get the first line
    let mut time = String::new();
    let _ = reader.read_line(&mut time);
    let mut time_split:Vec<&str> = time.split_whitespace().skip(1).collect();

    let mut dist = String::new();
    let _ = reader.read_line(&mut dist);
    let mut dist_split:Vec<&str> = dist.split_whitespace().skip(1).collect();

    let mut races:Vec<(i32,i32)> = Vec::new();
    for _ in time_split.clone()
    {
        races.push((time_split.remove(0).parse::<i32>().unwrap(), dist_split.remove(0).parse::<i32>().unwrap()));
    }

    let mut answer = 1;
    for race in races
    {
        answer *= compute_valid(&race);
    }

    println!("answer: {}", answer);

    Ok(())
}

fn compute_distance(speed:i32, time:i32) -> i32
{
    return speed * time;
}

fn compute_valid(race:&(i32,i32)) -> i32
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
