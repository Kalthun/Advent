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
    let file = File::open("ex.txt")?;
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
        answer += (count + 1) * hand.bid as usize;
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

    if card_freq.len() == 1 as usize
    {
        return 7; // Five of a kind
    }
    else if (card_freq.len() == 2) && (card_freq.values().any(|&f| f == 4))
    {
        return 6; // Four of a kind
    }
    else if card_freq.len() == 2
    {
        return 5; // Full House
    }
    else if (card_freq.len() == 3) && (card_freq.values().any(|&f| f == 3))
    {
        return 4; // Three of a kind
    }
    else if (card_freq.len() == 3) && (card_freq.values().any(|&f| f == 1))
    {
        return 3; // two pair
    }
    else if card_freq.len() == 4
    {
        return 2;
    }
    else
    {
        return 1; // Ordered High Card
    }
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
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    return *value_table.get(&sym).unwrap();
}
