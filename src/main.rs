use std::fs::File;
use std::io::{BufRead, BufReader};

struct Map
{
    n: i32, // conversion number

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
    let mut seed_input = String::new();
    let _ = reader.read_line(&mut seed_input);

    // convert into Vec of strings
    let mut seed_list: Vec<&str> = seed_input.split_whitespace().collect();
    let _ = seed_list.remove(0);

    // convert into Vec of i64
    let mut seeds: Vec<i64> = Vec::new();
    for _ in seed_list.clone()
    {
        seeds.push(seed_list.remove(0).parse::<i64>().unwrap());
    }

    // store input into a map vector
    let mut maps: Vec<Map> = Vec::new();

    let mut count = 0;

    for line in reader.lines()
    {
        let curr_line = line.unwrap();
        let first = curr_line.clone().chars().next().unwrap_or('-');

        if first.is_numeric() // numbers
        {
            let mut temp:Vec<&str> = curr_line.split_whitespace().collect();
            maps.push(Map{ n:count, d:temp.remove(0).parse::<i64>().unwrap(), s:temp.remove(0).parse::<i64>().unwrap(), r:temp.remove(0).parse::<i64>().unwrap()});
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

    let mut seed_to_soil: Vec<(i64,i64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(i64,i64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(i64,i64)> = Vec::new();
    let mut water_to_light: Vec<(i64,i64)> = Vec::new();
    let mut light_to_temperature: Vec<(i64,i64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(i64,i64)> = Vec::new();
    let mut humidity_to_location: Vec<(i64,i64)> = Vec::new();

    for map in maps
    {
        match map.n
        {
            1 => generate_entries(&map, &mut seed_to_soil),
            2 => generate_entries(&map, &mut soil_to_fertilizer),
            3 => generate_entries(&map, &mut fertilizer_to_water),
            4 => generate_entries(&map, &mut water_to_light),
            5 => generate_entries(&map, &mut light_to_temperature),
            6 => generate_entries(&map, &mut temperature_to_humidity),
            7 => generate_entries(&map, &mut humidity_to_location),
            _ => break, // error
        }
    }

    let mut min:i64 = f32::INFINITY as i64;

    for seed in seeds
    {
        let mut temp = seed;
        temp = in_table(temp, &seed_to_soil);
        temp = in_table(temp, &soil_to_fertilizer);
        temp = in_table(temp, &fertilizer_to_water);
        temp = in_table(temp, &water_to_light);
        temp = in_table(temp, &light_to_temperature);
        temp = in_table(temp, &temperature_to_humidity);
        temp = in_table(temp, &humidity_to_location);

        if temp < min as i64
        {
            min = temp;
        }
    }

    println!("min: {}", min);

    Ok(())
}

fn generate_entries(map:&Map, table:&mut Vec<(i64,i64)>)
{
    for inc in 0..map.r
    {
        table.push((map.s + inc as i64, map.d + inc as i64));
    }
}

fn in_table(num:i64, table:&Vec<(i64,i64)>) -> i64
{
    for entry in table
    {
        if num == entry.0
        {
            return entry.1;
        }
    }
    return num;
}

fn _print_table (table:&Vec<(i64,i64)>)
{
    println!("===\nNEW\n===");
    for entry in table
    {
        println!("({},{})", entry.0, entry.1);
    }
}
