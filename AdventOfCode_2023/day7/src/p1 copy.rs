use std::{fs::read_to_string, collections::HashMap, cmp::Ordering, fmt::Display};

trait MaxOfArr{
    type Item;
    fn max_element(&self) -> Self::Item; 
    fn nth_max_element(&self,n:usize) -> Option<Self::Item>;
}
impl MaxOfArr for Vec<u32>{
    type Item = u32;
    fn max_element(&self)->Self::Item{
        self.iter().fold(0, |acc,val| std::cmp::max(acc,*val))
    }
    fn nth_max_element(&self,n:usize)->Option<Self::Item>{
        if n-1 >= self.len(){
            return None
        } 
        let mut data = self.clone();
        data.sort();
        data.reverse();
        Some(data[n-1])
    } 
}
#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone,Ord,PartialOrd)]
struct NumCard{
    num: u32
}
impl Display for NumCard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.num == 10{
            write!(f,"T")
        }else{
            write!(f,"{}",self.num)
        }
    }
}
impl NumCard{
    fn new(num:u32) -> Option<Self>{
        if 2 <= num && num <= 10{
            Some(Self{num})
        }else{
            None
        }
    }
}
#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone,PartialOrd)]
enum Card{
    A,
    K,
    Q,
    J,
    Num(NumCard),
}
impl Ord for Card{
    fn cmp(&self, other: &Self) -> Ordering {
        match self{
            Card::A => {
                match other{
                    Card::A => Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            Card::K => {
                match other{
                    Card::A => Ordering::Less,
                    Card::K => Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            Card::Q => {
                match other{
                    Card::A | Card::K=> Ordering::Less,
                    Card::Q => Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            Card::J => {
                match other{
                    Card::A | Card::K | Card::Q=> Ordering::Less,
                    Card::J => Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            Card::Num(v) => match other{
                Card::Num(w) => v.cmp(&w),
                _ => Ordering::Less
            },
        }
    }
}
impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Card::A => write!(f,"A"),
            Card::K => write!(f,"K"),
            Card::Q => write!(f,"Q"),
            Card::J => write!(f,"J"),
            Card::Num(v) => write!(f,"{}",v),
        }
    }
}
impl Card{
    fn new(c:char)->Self{
        match c{
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::Num(NumCard { num: 10 }),
            v => Self::Num(NumCard { num: v.to_digit(10).unwrap() })
        }
    }
    fn get_value(&self)->u32{
        match self{
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::Num(v) => v.num,
        }
    }
}
#[derive(PartialEq,Eq,Debug)]
enum Hand{
    FiveKind([Card;5]),
    FourKind([Card;5]),
    FullHouse([Card;5]),
    ThreeKind([Card;5]),
    TwoPair([Card;5]),
    OnePair([Card;5]),
    HighCard([Card;5])
}
impl Display for Hand{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Hand::FiveKind(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Five of a kind: {card_str}")
            },
            Hand::FourKind(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Four of a kind: {card_str}")
            },
            Hand::FullHouse(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Full House: {card_str}")
            },
            Hand::ThreeKind(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Three of a kind: {card_str}")
            },
            Hand::TwoPair(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Two Pair: {card_str}")
            },
            Hand::OnePair(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"One Pair: {card_str}")
            },
            Hand::HighCard(cards) =>{
                let card_str = cards.iter().map(|c| format!("{}",c)).collect::<String>();
                write!(f,"Highest Card: {card_str}")
            },
        }
    }
}
impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type_val() < other.hand_type_val(){
            Some(Ordering::Less)
        }else if self.hand_type_val() > other.hand_type_val() {
            Some(Ordering::Greater)
        }else{
            // Compare the cards in the hand, ordered by how many of their counts.
            for (self_card, other_card) in self.cards_by_influence().iter().zip(other.cards_by_influence().iter()){
                if self_card.get_value() < other_card.get_value(){
                    return Some(Ordering::Less);
                }else if self_card.get_value() > other_card.get_value(){
                    return Some(Ordering::Greater);
                }
            }
            Some(Ordering::Equal)
        }
        
    }
}
impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type_val() < other.hand_type_val(){
            Ordering::Less
        }else if self.hand_type_val() > other.hand_type_val() {
            Ordering::Greater
        }else{
            // Compare the cards in the hand, ordered by how many of their counts.
            for (self_card, other_card) in self.cards_by_influence().iter().zip(other.cards_by_influence().iter()){
                if self_card.get_value() < other_card.get_value(){
                    return Ordering::Less;
                }else if self_card.get_value() > other_card.get_value(){
                    return Ordering::Greater
                }
            }
            Ordering::Equal
        }

    }
}
impl Hand{
    fn new(s:&str)->Option<Self>{
        if s.len() == 0{
            return None;
        }
        let card_map = s.chars().fold(HashMap::<char,u32>::new(), |mut acc, c|{
            if let Some(entry) = acc.get_mut(&c){
                *entry += 1;
            }else{
                acc.insert(c, 1);
            }
            acc
        });
        let mut counts: Vec<u32> = card_map.values().map(|v|*v).collect::<Vec<u32>>();
        counts.sort();
        let max_element = counts.max_element();
        let second_max = counts.nth_max_element(2);
        let mut cards: [Card;5] = s.chars().map(|c| Card::new(c)).collect::<Vec<Card>>().try_into().unwrap();
        cards.sort();
        match (max_element,second_max){
            (5,_)   =>  Some(Hand::FiveKind(cards)),
            (4,_)   =>  Some(Hand::FourKind(cards)),
            (3,Some(2))   =>  Some(Hand::FullHouse(cards)),
            (3,_)   =>  Some(Hand::ThreeKind(cards)),
            (2,Some(2))   =>  Some(Hand::TwoPair(cards)),
            (2,_)   =>  Some(Hand::OnePair(cards)),
            _       =>  Some(Hand::HighCard(cards))
        }
    }
    fn hand_type_val(&self)->u8{
        match self{
            Hand::FiveKind(_)   => 6,
            Hand::FourKind(_)   => 5,
            Hand::FullHouse(_)  => 4,
            Hand::ThreeKind(_)  => 3,
            Hand::TwoPair(_)    => 2,
            Hand::OnePair(_)    => 1,
            Hand::HighCard(_)   => 0,
        }
    }
    fn get_cards(&self) -> [Card;5]{
        match self{
            Hand::FiveKind(cards) => *cards,
            Hand::FourKind(cards) => *cards,
            Hand::FullHouse(cards) => *cards,
            Hand::ThreeKind(cards) => *cards,
            Hand::TwoPair(cards) => *cards,
            Hand::OnePair(cards) => *cards,
            Hand::HighCard(cards) => *cards
        }
    }
    fn cards_by_influence(&self) ->Vec<Card>{
        let mut counter = [u32::MAX;13];
        for card in self.get_cards().iter(){
            counter[(card.get_value()-2) as usize] -= 1;
        }
        let mut all_cards = vec![Card::A, Card::K, Card::Q, Card::J];
        for i in (2..=10).rev(){
            all_cards.push(Card::Num(NumCard { num: i }))
        }
        all_cards.sort();
        let mut card_count = all_cards
            .into_iter()
            .filter(|card| counter[(card.get_value()-2) as usize] != u32::MAX)
            .map(|card| (counter[(card.get_value()-2) as usize], card) )
            .collect::<Vec<(u32,Card)>>();
        card_count.sort();
        for (count,card) in card_count.iter(){
            print!("{card}");
        }
        print!(" - ");
        let output = card_count.into_iter().map(|(_,card)| card).collect::<Vec<Card>>();
        for card in output.iter(){
            print!("{card}");
        }
        println!();
        output
    }
}

fn process_data_string(data_string:String)->Vec<(Hand,u32)>{
    data_string.lines().map(|line| {
        let [card_str, bid_str]: [&str;2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        (Hand::new(card_str).unwrap(), bid_str.parse::<u32>().unwrap())
    }).collect::<Vec<(Hand,u32)>>()
}

pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut data = process_data_string(data_string);
    data.sort();
    let total = data.iter().enumerate().fold(0, |acc,(idx, (_hand,bid))| {
        {
            println!("{_hand}");
        }
        acc + (idx as u32+1)* *bid});
    Some(total)
}

#[cfg(test)]
    mod tests{
    use super::*;
    #[test]
    fn test_max(){
        let a:Vec<u32> = vec![3,1,2];
        assert_eq!(a.nth_max_element(2), Some(2));
        assert_eq!(a.max_element(), 3);
    }
    #[test]
    fn can_make_jjjjj(){
        let new_hand =Hand::new("JJJJJ").unwrap(); 
        assert!(true);
    }
    #[test]
    fn random_comparisons_check(){
        let combinations = vec![
            ("44JJ9","TAK79",Ordering::Greater),
            ("KKJKJ","TAK79",Ordering::Greater),
            ("44JJ9","J6336",Ordering::Greater),
            ("22992","22992",Ordering::Equal),
            ("55KQQ","555AA",Ordering::Less),
            ("JJJJK","TTTTT",Ordering::Less),
            ("6A5Q8","888JJ",Ordering::Less),
            ("66TK9","J7777",Ordering::Less),
        ];
        for (a,b,ans) in combinations.into_iter(){
            assert_eq!(Hand::new(a).unwrap().cmp(&Hand::new(b).unwrap()), ans,"{a} vs {b} <-- {ans:?}");
        }
    }
    #[test]
    fn test_full_house(){
        let hand = Hand::new("AAAKK").unwrap();
        if let Hand::FullHouse(_) = hand{
            assert!(true,"\"Full House\" found! {hand}");
        }else{
            assert!(false, "Should be \"Full House\"! {hand}");
        }
    }
    #[test]
    fn dummy_steps(){
        let file_name = "src/dummy.txt";
        let data_string = read_to_string(file_name).unwrap();
        let mut data = process_data_string(data_string);
        data.sort();
        let expected = vec!["32T3K","KTJJT","KK677","T55J5","QQQJA"];
        for (idx,(hand,_)) in data.iter().enumerate(){
            let new_hand =Hand::new(expected[idx]).unwrap(); 
            assert_eq!(*hand, new_hand, "\n\n idx: {idx} | {hand} vs {new_hand}\n\n")
        }
    }
    #[test]
    fn hand_picked_test(){
        let larger  = Hand::new("AAAAA").unwrap();
        let smaller = Hand::new("23456").unwrap();
        assert!(smaller < larger, "{smaller} !< {larger}" );

        let larger  = Hand::new("JJJJ8").unwrap();
        let ordered_cards = larger.cards_by_influence();
        println!("{larger}");
        for card in ordered_cards{
            print!("{card}");
        }
        println!("___");
        let smaller = Hand::new("Q2222").unwrap();
        assert!(smaller < larger, "{smaller} !< {larger}");
    }
    #[test]
    fn ordering_test_a(){
        let smaller = Hand::new("AQ3A5").unwrap();
        let larger = Hand::new("AQ2A8").unwrap();
        assert_eq!(smaller.cmp(&larger),Ordering::Less,"{smaller} !< {larger}")
    }
    #[test]
    fn ordering_test_b(){
        let mut experiment = vec![Card::new('Q'),Card::new('3'),Card::new('K')];
        experiment.sort();
        let answer = vec![Card::new('K'),Card::new('Q'),Card::new('3')];
        assert_eq!(experiment,answer,)
    }
    #[test]
    fn compare_individual_cards(){
        let mut cards = vec![Card::A, Card::K, Card::Q, Card::J];
        for i in (2..=10).rev(){
            cards.push(Card::Num(NumCard { num: i }))
        }
        for i in 0..cards.len()-1{
            println!("{:?}",cards[i].cmp(&cards[i+1]));
            assert_eq!(cards[i].cmp(&cards[i+1]),Ordering::Greater, "{} > {}", cards[i], cards[i+1]);
        }
        let a = Card::J;
        let b = Card::Num(NumCard { num: 9 });
        assert_eq!(a.cmp(&b),Ordering::Greater);

        let a = Card::J;
        let b = Card::Num(NumCard { num: 10 });
        assert_eq!(a.cmp(&b),Ordering::Greater);

        let a = Card::K;
        let b = Card::Num(NumCard { num: 7 });
        assert_eq!(a.cmp(&b),Ordering::Greater);

        let a = Card::Num(NumCard { num: 7 });
        let b = Card::Num(NumCard { num: 6 });
        assert_eq!(a.cmp(&b),Ordering::Greater);

    }
    #[test]
    fn cmp_functionality_test(){
        assert_eq!(1.cmp(&2), Ordering::Less);
        assert_eq!(2.cmp(&2), Ordering::Equal);
        assert_eq!(3.cmp(&2), Ordering::Greater);
    }
}
