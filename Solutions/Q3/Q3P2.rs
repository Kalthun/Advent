use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let file = File::open("Q3_input.txt")?;

    let reader = BufReader::new(file);

    let mut gear_positions:Vec<(i32,i32)> = Vec::new();

    let mut num_position_pair:Vec<(i32, Vec<(i32,i32)>)> = Vec::new(); // the number and all position that make it valid

    let mut answer = 0;

    let mut num = String::new();
    let mut pos: Vec<(i32,i32)> = Vec::new();
    let mut end_of_num;

    for (row, line) in reader.lines().enumerate()
    {
        for (col, c) in line.unwrap().chars().into_iter().enumerate()
        {
            if c.is_numeric()
            {
                end_of_num = false;
                num += c.to_string().as_str();
                generate_connected(row as i32, col as i32, &mut pos);
            }
            else if c == '*'
            {
                end_of_num = true;
                gear_positions.push((row as i32, col as i32));
            }
            else
            {
                end_of_num = true;
            }

            if end_of_num && !num.is_empty()
            {
                num_position_pair.push((num.parse::<i32>().unwrap(), pos));
                num = String::new();
                pos = Vec::new();
            }
        }

        if !num.is_empty()
        {
            num_position_pair.push((num.parse::<i32>().unwrap(), pos));
            num = String::new();
            pos = Vec::new();
        }
    }

    for gear in gear_positions
    {
        let mut adjacent = 0;
        let mut ratio = 1;

        for pair in &num_position_pair
        {
            if pair.1.contains(&gear)
            {
                adjacent += 1;
                ratio *= pair.0;
            }
        }

        if adjacent == 2
        {
            answer += ratio;
        }
    }

    println!("{}", answer);

    Ok(())

}

fn generate_connected(row:i32, col:i32, vec:&mut Vec<(i32,i32)>)
{
    let mut temp:Vec<(i32,i32)> = Vec::new();

    temp.push((row - 1, col - 1));
    temp.push((row - 1, col));
    temp.push((row - 1, col + 1));
    temp.push((row, col - 1));
    temp.push((row, col + 1));
    temp.push((row + 1, col - 1));
    temp.push((row + 1, col));
    temp.push((row + 1, col + 1));

    for pos in temp
    {
        if !vec.contains(&pos)
        {
            vec.push(pos);
        }
    }
}

