use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Hand<>
{
    strength:i8,
    cards:String,
    bid:i32,
}

fn main() -> std::io::Result<()>
{
    let file = File::open("Q7_input.txt")?;
    let reader = BufReader::new(file);

    let mut hands: Vec<Hand> = Vec::new();

    for line in reader.lines()
    {

        let curr_line = line.unwrap();
        let mut split:Vec<&str> = curr_line.split_whitespace().collect();
        let temp = split.remove(0).to_string();
        hands.push(Hand
            {
                strength:_compute_strength(&temp),
                cards:temp,
                bid:split.remove(0).parse::<i32>().unwrap(),
            });
    }

    hands.sort_by(|a, b| compare_hands(a,b));

    for hand in &hands {
        println!("strength: {}, cards: {:?}, bid: {}", hand.strength, hand.cards, hand.bid);
    }

    let mut answer = 0;

    for (count, hand) in hands.iter().enumerate()
    {
        answer += (count + 1) * (hand.bid as usize);
        println!("count:{}, a:{}", count + 1, answer);
    }

    println!("answer: {}", answer);

    Ok(())
}

fn _compute_strength(cards:&String) -> i8
{

    let mut card_freq = HashMap::new();

    let split:Vec<char> = cards.chars().collect();

    split.iter().for_each(|c|
    {
        card_freq
        .entry(*c)
        .and_modify(|f| *f += 1)
        .or_insert(1);
    });

    println!("{:?}", card_freq);

    if card_freq.contains_key(&'J')
    {
        if card_freq.len() == 1 as usize
        {
            return 7;
        }

        let mut max = 0;
        let mut max_key = ' ';

        for entry in &card_freq
        {
            if *entry.0 != 'J' && *entry.1 > max
            {
                max = *entry.1;
                max_key = *entry.0;
            }
        }

        card_freq.insert(max_key, max + card_freq.get(&'J').unwrap());
        card_freq.remove(&'J');
        println!("max:{}", max);
        println!("max_key:{}", max_key);
        println!("{:?}", card_freq);
    }

    let mut strength = 1;

    if card_freq.len() == 1 as usize
    {
        strength = 7; // Five of a kind
    }
    else if (card_freq.len() == 2) && (card_freq.values().any(|&f| f == 4))
    {
        strength = 6; // Four of a kind
    }
    else if card_freq.len() == 2
    {
        strength = 5; // Full House
    }
    else if (card_freq.len() == 3) && (card_freq.values().any(|&f| f == 3))
    {
        strength = 4; // Three of a kind
    }
    else if (card_freq.len() == 3) && (card_freq.values().any(|&f| f == 1))
    {
        strength = 3; // two pair
    }
    else if card_freq.len() == 4
    {
        strength = 2;
    }

    println!("strength:{}", strength);
    println!("");
    return strength;

}

fn compare_hands(a:&Hand, b:&Hand) -> Ordering
{
    if a.strength > b.strength
    {
        return Ordering::Greater;
    }

    if b.strength > a.strength
    {
        return Ordering::Less;
    }

    let mut a_cards:Vec<char> = a.cards.chars().collect();
    let mut b_cards:Vec<char> = b.cards.chars().collect();

    for _ in 0..5
    {
        let temp_a = get_value(a_cards.remove(0));
        let temp_b = get_value(b_cards.remove(0));
        if temp_a > temp_b
        {
            return Ordering::Greater;
        }
        else if temp_b > temp_a
        {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn get_value(sym:char) -> i8
{
    let value_table = HashMap::from
    ([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    return *value_table.get(&sym).unwrap();
}
