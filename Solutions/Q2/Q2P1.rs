use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    // Open the File for Reading
    let file = File::open("Q2_input.txt")?;

    let reader = BufReader::new(file);

    let mut answer = 0;

    for (index, game) in reader.lines().enumerate()
    {
        if is_valid(game.unwrap().as_str())
        {
            answer += index + 1;
        }
    }

    print!("{}", answer);

    Ok(())

}

fn is_valid(game:&str) -> bool
{

    let mut temp: Vec<&str> = game.split(' ').collect();

    temp.remove(0); // get rid of "Game"
    temp.remove(0); // get rid of "#:"
    let mut prev = temp.remove(0); // set first number

    for w in temp
    {
        match w
        {
            "red"|"red,"|"red;" => if prev.parse::<i32>().unwrap() > 12 { return false }
            "blue"|"blue,"|"blue;" => if prev.parse::<i32>().unwrap() > 14 { return false }
            "green"|"green,"|"green;" => if prev.parse::<i32>().unwrap() > 13 { return false }
            _=> prev = w,
        }
    }

    return true;
}
