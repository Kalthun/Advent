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
    let file = File::open("Q8_input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let _ = reader.read_line(&mut buffer);

    let directions:Vec<char> = buffer.chars().filter(|c| *c == 'R' || *c == 'L').collect();

    let _ = reader.read_line(&mut buffer);

    let mut map:HashMap<String,Point> = HashMap::new();

    for line in reader.lines()
    {
        let temp = line.unwrap();
        let mut vals:Vec<&str> = temp.split(|ex| ex == ' ' || ex == '(' || ex == ')' || ex == ',' || ex == '=').filter(|ex| *ex != "").collect();

        map.insert(vals.remove(0).to_string(), Point{left:vals.remove(0).to_string(),right:vals.remove(0).to_string()});
    }

    let mut current_positions:Vec<String> = map.keys().filter(|k| k.ends_with('A')).map(|e| e.to_string()).collect();
    let mut steps = 0;

    while !current_positions.iter().all(|pos| pos.ends_with('Z'))
    {
        let dir = *directions.get(steps % directions.len()).unwrap();
        // println!("{:?} -> dir:{}", current_positions, dir);
        let mut copy:Vec<String> = Vec::new();

        for pos in current_positions
        {
            if dir == 'R'
            {
                copy.push(map.get(&pos).unwrap().right.clone());
            }
            else
            {
                copy.push(map.get(&pos).unwrap().left.clone());
            }
        }
        current_positions = copy;
        steps += 1;
    }

    print!("steps: {}", steps);

    Ok(())
}
