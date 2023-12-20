use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {

    let file = File::open("Q4_input.txt")?;

    let reader = BufReader::new(file);

    let mut total = vec![1; 201]; // fix later

    let length = total.len();

    for (index, line) in reader.lines().enumerate() // go through each of the lines
    {
        let temp = line.unwrap();

        for _copy in 0..total[index] // for each copy
        {
            for count in 0..get_points(&temp) // add 1 to subsequent things
            {
                if (1 + index + count as usize) >= length
                {
                    break;
                }
                total[1 + index + count as usize] += 1;
            }
        }
    }

    let answer:i32 = total.iter().sum();

    println!("answer: {}", answer);

    Ok(())

}

fn get_points(line:&String) -> i32
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

    return counter;
}

