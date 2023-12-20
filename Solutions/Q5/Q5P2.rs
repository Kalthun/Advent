use std::fs::File;
use std::io::{BufRead, BufReader};

struct Entry
{
    // could make an enum

    // 1: seed  -> soil
    // 2: soil  -> fert
    // 3: fert  -> water
    // 4: water -> light
    // 5: light -> temp
    // 6: temp  -> hum
    // 7: hum   -> loc

    d: i64, // destination
    s: i64, // source
    r: i64, // range
}

fn main() -> std::io::Result<()>
{
    let file = File::open("Q5_input.txt")?;
    let mut reader = BufReader::new(file);

    // get the first line
    let mut seed_info = String::new();
    let _ = reader.read_line(&mut seed_info);

    let mut s_to_s: Vec<Entry> = Vec::new();
    let mut s_to_f: Vec<Entry> = Vec::new();
    let mut f_to_w: Vec<Entry> = Vec::new();
    let mut w_to_l: Vec<Entry> = Vec::new();
    let mut l_to_t: Vec<Entry> = Vec::new();
    let mut t_to_h: Vec<Entry> = Vec::new();
    let mut h_to_l: Vec<Entry> = Vec::new();

    let mut count = 0;
    for line in reader.lines()
    {
        let curr_line = line.unwrap();
        let first = curr_line.clone().chars().next().unwrap_or('-');

        if first.is_numeric() // numbers
        {
            let mut temp:Vec<&str> = curr_line.split_whitespace().collect();

            if temp.get(0) > temp.get(1)
            {
                continue;
            }

            match count
            {
                1 => s_to_s.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                2 => s_to_f.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                3 => f_to_w.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                4 => w_to_l.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                5 => l_to_t.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                6 => t_to_h.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                7 => h_to_l.push(Entry{d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()}),
                _ => println!("ERROR"),
            }
        }
        else if first != '-' // new map type
        {
            count += 1;
        }
        else // blankspace
        {
            continue;
        }
    }

    let mut seed_pair: Vec<&str> = seed_info.split_whitespace().collect();
    let _ = seed_pair.remove(0);

    let mut min:i64 = f32::INFINITY as i64;
    let mut start = 0;

    for (index,_) in seed_pair.clone().iter().enumerate()
    {
        if index % 2 == 0
        {
            start = seed_pair.remove(0).parse::<i64>().unwrap();
        }
        else
        {
            for count in 0..seed_pair.remove(0).parse::<i64>().unwrap()
            {
                let mut temp = start + count;

                temp = in_range(temp, &s_to_s);
                temp = in_range(temp, &s_to_f);
                temp = in_range(temp, &f_to_w);
                temp = in_range(temp, &w_to_l);
                temp = in_range(temp, &l_to_t);
                temp = in_range(temp, &t_to_h);
                temp = in_range(temp, &h_to_l);

                if temp < min as i64
                {
                    min = temp;
                    println!("min:{}", min);
                }
            }
        }
    }

    println!("min: {}", min);

    Ok(())
}

fn in_range(num:i64, map:&Vec<Entry>) -> i64
{
    for entry in map
    {
        if num >= entry.s && num < (entry.s + entry.r)
        {
            return entry.d + num - entry.s;
        }
    }
    return num;
}
