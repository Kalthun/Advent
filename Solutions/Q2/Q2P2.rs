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

fn get_power_set(game:&str) -> i32
{

    let mut temp: Vec<&str> = game.split(' ').collect();

    let _ = temp.remove(0); // get rid of "Game"
    let _ = temp.remove(0); // get rid of "#:"
    let mut prev = temp.remove(0); // set first number

    let mut r_min = 0;
    let mut b_min = 0;
    let mut g_min = 0;

    for w in temp
    {
        let val = prev.parse::<i32>().unwrap();
        match w
        {
            "red"|"red,"|"red;" => if val > r_min { r_min = val }
            "blue"|"blue,"|"blue;" => if val > b_min { b_min = val }
            "green"|"green,"|"green;" => if val > g_min { g_min = val }
            _=> prev = w,
        }
    }

    return r_min * b_min * g_min;
}
