use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let file = File::open("Q4_input.txt")?;

    let reader = BufReader::new(file);

    let mut sum:Vec<i32> = Vec::new();

    for line in reader.lines()
    {
        sum.push(get_points(line.unwrap()));
    }

    let answer:i32 = sum.iter().sum();

    println!("answer: {}", answer);

    Ok(())

}

fn get_points(line:String) -> i32
{
    let mut temp:Vec<&str> = line.split_whitespace().collect();

    let _ = temp.remove(0); // get rid of "Card"
    let _ = temp.remove(0); // get rid of "#:"

    let mut winning_numbers:Vec<i32> = Vec::new();
    let mut numbers:Vec<i32> = Vec::new();
    let mut divider_reached = false;

    for sub in temp
    {
        if sub.eq("|")
        {
            divider_reached = true;
        }
        else
        {
            if divider_reached == false
            {
                winning_numbers.push(sub.parse::<i32>().unwrap());
            }
            else
            {
                numbers.push(sub.parse::<i32>().unwrap());
            }
        }
    }

    let mut counter = 0;

    for number in numbers
    {
        if winning_numbers.contains(&number)
        {
            counter += 1;
        }
    }

    if counter == 0
    {
        return 0;
    }
    else
    {
        let base:i32 = 2;
        return base.pow(counter - 1);
    }
}

