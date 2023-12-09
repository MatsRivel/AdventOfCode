use std::{fs::read_to_string, fmt::Display, cmp::Ordering, collections::HashMap};

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


#[derive(Debug,PartialEq,Eq,Hash,Copy,Clone,PartialOrd,Ord)]
pub struct Card{
    pub v: u8
}
impl Card{
    pub fn new(c:char)->Self{
        match c{
            'A' => Self{v:14},
            'K' => Self{v:13},
            'Q' => Self{v:12},
            'J' => Self{v:11},
            'T' => Self{v:10},
            v => {
                let n = match v.to_digit(10) {
                    Some(n) => n,
                    None => panic!("char == {c}"),
                };
                if n < 2{
                    panic!("value can not be lower than 2! {c}");
                }
                Self { v: n as u8}
            }
        }
    }
    pub fn value(&self) -> u8{
        self.v
    }
}
impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value(){
            14 => write!(f,"A"),
            13 => write!(f,"K"),
            12 => write!(f,"Q"),
            11 => write!(f,"J"),
            10 => write!(f,"T"),
            v => write!(f,"{v}"),
        }
    }
}
#[derive(PartialEq, Eq,Debug, Clone)]
pub enum Hand{
    FiveKind([Card;5]),
    FourKind([Card;5]),
    FullHouse([Card;5]),
    ThreeKind([Card;5]),
    TwoPair([Card;5]),
    OnePair([Card;5]),
    HighCard([Card;5])
}
impl Hand{
    pub fn new(s:&str)->Self{
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
        let cards: [Card;5] = s.chars().map(|c| Card::new(c)).collect::<Vec<Card>>().try_into().unwrap();
        match (max_element,second_max){
            (5,_)       =>  Hand::FiveKind(cards),
            (4,_)       =>  Hand::FourKind(cards),
            (3,Some(2)) =>  Hand::FullHouse(cards),
            (3,_)       =>  Hand::ThreeKind(cards),
            (2,Some(2)) =>  Hand::TwoPair(cards),
            (2,_)       =>  Hand::OnePair(cards),
            _           =>  Hand::HighCard(cards)
        }
    }
    pub fn type_value(&self) -> u8{
        match self{
            Hand::FiveKind(_) => 6,
            Hand::FourKind(_) => 5,
            Hand::FullHouse(_) => 4,
            Hand::ThreeKind(_) => 3,
            Hand::TwoPair(_) => 2,
            Hand::OnePair(_) => 1,
            Hand::HighCard(_) => 0,
        }
    }
    pub fn get_cards(&self) -> [Card;5]{
        match self{
            Hand::FiveKind(card) => *card,
            Hand::FourKind(card) => *card,
            Hand::FullHouse(card) => *card,
            Hand::ThreeKind(card) => *card,
            Hand::TwoPair(card) => *card,
            Hand::OnePair(card) => *card,
            Hand::HighCard(card) => *card,
        }
    }
}
impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.type_value() != other.type_value(){
            return self.type_value().cmp(&other.type_value());
        }
        let a = self.get_cards();
        let b = other.get_cards();
        for (ca,cb) in a.iter().zip(b.iter()){
            if ca.cmp(&cb) == Ordering::Equal{
                continue;
            }
            return ca.cmp(&cb);
        }
        Ordering::Equal
    }
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

pub fn process_data_string(data_string:String)->Vec<(Hand,u32)>{
    data_string.lines().map(|line| {
        let [card_str, bid_str]: [&str;2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        (Hand::new(card_str), bid_str.parse::<u32>().unwrap())
    }).collect::<Vec<(Hand,u32)>>()
}

pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut data = process_data_string(data_string);
    data.sort();
    let total = data.iter().enumerate().fold(0, |acc,(idx, (_hand,bid))| acc + (idx as u32+1)* *bid);
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;
    #[test]
    pub fn ordering_test_b(){
        let mut experiment = vec![Card::new('Q'),Card::new('3'),Card::new('K')];
        experiment.sort();
        let answer = vec![Card::new('3'),Card::new('Q'),Card::new('K')];
        assert_eq!(experiment,answer,)
    }
    #[test]
    pub fn ordering_test_c(){
        let smaller = Hand::new("555TJ");
        let larger = Hand::new("JQQQA");
        assert_eq!(smaller.cmp(&larger),Ordering::Less,"{smaller} !< {larger}");
        
        let smaller = Hand::new("677KK");
        let larger = Hand::new("JQQQA");
        assert_eq!(smaller.cmp(&larger),Ordering::Less,"{smaller} !< {larger}");

        let smaller = Hand::new("677KK");
        let larger = Hand::new("555TJ");
        assert_eq!(smaller.cmp(&larger),Ordering::Less,"{smaller} !< {larger}");

    }
    #[test]
    pub fn test_max(){
        let a:Vec<u32> = vec![3,1,2];
        assert_eq!(a.nth_max_element(2), Some(2));
        assert_eq!(a.max_element(), 3);
    }
    #[test]
    pub fn can_make_jjjjj(){
        let new_hand =Hand::new("JJJJJ"); 
        assert!(true);
    }

    #[test]
    pub fn test_full_house(){
        let hand = Hand::new("AAAKK");
        if let Hand::FullHouse(_) = hand{
            assert!(true,"\"Full House\" found! {hand}");
        }else{
            assert!(false, "Should be \"Full House\"! {hand}");
        }
    }
    #[test]
    pub fn dummy_steps(){
        let file_name = "src/dummy.txt";
        let data_string = read_to_string(file_name).unwrap();
        let mut data = process_data_string(data_string);
        println!("Sorting starting:");
        data.sort();
        println!("Sorting finished.");
        let expected = vec!["32T3K","KTJJT","KK677","T55J5","QQQJA"];
        for (idx,(hand,_)) in data.iter().enumerate(){
            let new_hand =Hand::new(expected[idx]); 
            assert_eq!(*hand, new_hand, "\n\n idx: {idx} | {hand} vs {new_hand}\n\n");
        }
    }
    #[test]
    pub fn compare_individual_cards(){
        let mut cards = vec![];
        for i in (2..=14).rev(){
            cards.push(Card{v:i})
        }
        for i in 0..cards.len()-1{
            println!("{:?}",cards[i].cmp(&cards[i+1]));
            assert_eq!(cards[i].cmp(&cards[i+1]),Ordering::Greater, "{} > {}", cards[i], cards[i+1]);
        }

    }
    #[test]
    pub fn cmp_functionality_test(){
        assert_eq!(1.cmp(&2), Ordering::Less);
        assert_eq!(2.cmp(&2), Ordering::Equal);
        assert_eq!(3.cmp(&2), Ordering::Greater);
    }

}
