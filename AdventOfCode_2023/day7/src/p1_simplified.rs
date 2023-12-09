use std::{fs::read_to_string, fmt::Display, cmp::Ordering, collections::HashMap};

fn char_to_card(c:char)->u8{
    match c{
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        v => v.to_digit(10).unwrap() as u8
    }
}
fn process_data_string(data_string:String)->Vec<([u8;5],u32)>{
    data_string.lines().map(|line| {
        let [card_str, bid_str]: [&str;2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        let bid = bid_str.parse::<u32>().unwrap();
        let cards: [u8;5] = card_str.chars().map(|c| char_to_card(c) ).collect::<Vec<u8>>().try_into().unwrap();
        (cards, bid)
    }).collect::<Vec<([u8;5],u32)>>()
}
fn get_card_type(card:&[u8;5])->CardType{
    let mut counts = [0;13];
    card.iter().for_each(|c| counts[*c as usize -2] += 1);
    if counts.contains(&5){
        CardType::FiveOfAKind
    }else if counts.contains(&4){
        CardType::FourOfAKind
    }else if counts.contains(&3) && counts.contains(&2){
        CardType::FullHouse
    }else if counts.contains(&3){
        CardType::ThreeOfAKind
    }else if counts.iter().filter(|v| **v==2).count() == 2{
        CardType::TwoPair
    }else if counts.contains(&2){
        CardType::OnePair
    }else{
        CardType::HighCard
    }
}
enum CardType{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn card_by_count(card:&[u8;5])->Vec<[u8;2]>{
    card.iter()
        .fold(HashMap::<u8,u8>::new(), |mut acc,v|{
            if let Some(entry) = acc.get_mut(v){
                *entry += 1;
            }else{
                acc.insert(*v, 1);
            }
            acc
        })
        .into_iter()
        .map(|(card,count)| [count,card])
        .collect::<Vec<[u8;2]>>()
}
fn print_card(&card:&[u8;5]){
    for c in card.iter(){
        match c{
            14 => print!("A"),
            13 => print!("K"),
            12 => print!("Q"),
            11 => print!("J"),
            10 => print!("T"),
            v => print!("{v}")
        }
    }
    println!();
}
pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut data = process_data_string(data_string);
    let mut bins: Vec<Vec<([u8;5],u32)>> = vec![vec![];7];
    for (card,bid) in data.into_iter(){
        match get_card_type(&card){
            CardType::FiveOfAKind   => bins[6].push((card,bid)),
            CardType::FourOfAKind   => bins[5].push((card,bid)),
            CardType::FullHouse     => bins[4].push((card,bid)),
            CardType::ThreeOfAKind  => bins[3].push((card,bid)),
            CardType::TwoPair       => bins[2].push((card,bid)),
            CardType::OnePair       => bins[1].push((card,bid)),
            CardType::HighCard      => bins[0].push((card,bid)),
        }
    }
    let mut counter = 1;
    let mut total = 0;
    for mut bin in bins.into_iter(){
        bin.sort();
        for (_, bid) in bin.into_iter(){
            total += bid * counter;
            counter +=1;
        }
    }
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;


}
