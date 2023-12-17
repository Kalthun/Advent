use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {


    let mut total: Vec<i32> = Vec::new();

    // Open the File for Reading
    let file = File::open("Q1_input.txt")?;

    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        total.push(get_line_number(line.unwrap()));
    }

    let answer:i32 = total.iter().sum();

    println!("answer is: {}", answer);

    Ok(())

}

fn transform_string(string:&String) -> String
{
    let valid = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut temp = 0;

    for (index, num) in valid.iter().enumerate()
    {
        if string.contains(num)
        {
            temp = index + 1;
            break;
        }
    }

    return temp.to_string();
}

fn get_line_number(line:String) -> i32
{

    let mut temp = String::from("");
    let mut start = String::from(" ");
    let mut end = String::from(" ");

    for c in line.chars()
    {
        if c.is_numeric()
        {
            if start == " "
            {
                start = c.to_string();
            }
            end = c.to_string();
            temp = "".to_string();
        }
        else
        {
            temp = temp + c.to_string().as_str();
            let ret = transform_string(&temp);
            if ret != "0"
            {
                if start == " "
                {
                    start = ret.to_string();
                }
                end = ret.to_string();
                temp = temp.chars().last().unwrap().to_string();
            }
        }
    }

    return 10 * (start.parse::<i32>().unwrap()) + (end.parse::<i32>().unwrap());
}
