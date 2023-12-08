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
                Self { v: n as u8}
            }
        }
    }
    pub fn value_as_idx(&self) -> usize{
        (self.v-2) as usize
    }
    pub fn value(&self) -> u8{
        if self.v == 11{1}
        else{self.v}
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
        let initial_hand = match (max_element,second_max){
            (5,_)       =>  Hand::FiveKind(cards),
            (4,_)       =>  Hand::FourKind(cards),
            (3,Some(2)) =>  Hand::FullHouse(cards),
            (3,_)       =>  Hand::ThreeKind(cards),
            (2,Some(2)) =>  Hand::TwoPair(cards),
            (2,_)       =>  Hand::OnePair(cards),
            _           =>  Hand::HighCard(cards)
        };
        // Considering the joker makes us claim a hand is a higher rank than it actually is,
        // BUT this devalues the joker to be <2.
        let joker_hand = initial_hand.consider_joker();
        match joker_hand{
            Hand::FiveKind(_) => Hand::FiveKind(cards),
            Hand::FourKind(_) => Hand::FourKind(cards),
            Hand::FullHouse(_) => Hand::FullHouse(cards),
            Hand::ThreeKind(_) => Hand::ThreeKind(cards),
            Hand::TwoPair(_) => Hand::TwoPair(cards),
            Hand::OnePair(_) => Hand::OnePair(cards),
            Hand::HighCard(_) => Hand::HighCard(cards),
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
    fn consider_joker(&self) -> Self{
        let cards = self.get_cards();
        if !cards.contains(&Card{v:11}){
            return self.clone();
        }
        let mut count_array = [0;13];
        for c in cards.iter(){
            count_array[c.value_as_idx()] += 1;
        }
        match self{
            Hand::FiveKind(_) => Hand::new("AAAAA"), // Make them all Ace
            Hand::FourKind(cards) | Hand::FullHouse(cards) => {
                // Find the only non-J-type card and match it.
                // This will allways increase the type value by1.
                let non_j = cards.iter().filter(|c| **c != Card{v:11}).nth(0).unwrap();
                let card_str = (0..5).map(|_| format!("{non_j}")).collect::<String>();
                Hand::new(card_str.as_str())
            },
            Hand::ThreeKind(cards) => {
                // Make it four of a kind by matching the highest single value.
                if count_array[9] == 3{
                    let mut non_js = cards.into_iter().filter(|c| **c != Card{v:11}).map(|c| *c).collect::<Vec<Card>>();
                    non_js.sort();
                    let best_non_js = non_js.last().unwrap();
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{best_non_js}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand
                }else{ // Make it a four-kind by matching the existing triplet
                    let index_of_max = count_array
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.cmp(b))
                        .map(|(index, _)| index)
                        .unwrap();
                    let card_str = &idx_to_str(index_of_max);
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{card_str}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand
                }
            },
            Hand::TwoPair(cards) => {
                let mut count_array = [0;13];
                for c in cards.iter(){
                    count_array[c.value_as_idx()] += 1;
                }
                // Make it a three-kind by finding the highest pair.
                if count_array[9] == 1{
                    let mut non_js = cards.into_iter().filter(|c| **c != Card{v:11}).map(|c| *c).collect::<Vec<Card>>();
                    non_js.sort();
                    let best_non_js = non_js.last().unwrap();
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{best_non_js}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand
                }else{ // Make it a Four of a kind by matching the other pair.
                    let index_of_single = count_array
                        .into_iter()
                        .enumerate()
                        .filter(|(idx,count)| *idx != 9 && *count == 2)
                        .map(|(idx,_)| idx)
                        .nth(0)
                        .unwrap();
                    let card_str = &idx_to_str(index_of_single);
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{card_str}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand

                }
            },
            Hand::OnePair(cards) => {
                let mut count_array = [0;13];
                for c in cards.iter(){
                    count_array[c.value_as_idx()] += 1;
                }
                // Match the highest non-j card
                if count_array[9] == 2{
                    let mut non_js = cards.into_iter().filter(|c| **c != Card{v:11}).map(|c| *c).collect::<Vec<Card>>();
                    non_js.sort();
                    let best_non_js = non_js.last().unwrap();
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{best_non_js}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand
                }else{ // Match the pair.
                    let index_of_pair = count_array
                        .into_iter()
                        .enumerate()
                        .filter(|(_,count)| *count == 2)
                        .map(|(idx,_)| idx)
                        .nth(0)
                        .unwrap();
                    let card_str = &idx_to_str(index_of_pair);
                    let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                    let new_hand_str = old_hand_str.replace("J", format!("{card_str}").as_str());
                    let new_hand = Hand::new(new_hand_str.as_str());
                    new_hand
                }
            },
            Hand::HighCard(cards) => {
                // Match the highest card.
                let mut non_js = cards.into_iter().filter(|c| **c != Card{v:11}).map(|c| *c).collect::<Vec<Card>>();
                non_js.sort();
                let best_non_js = non_js.last().unwrap();
                let old_hand_str = cards.into_iter().map(|c| format!("{c}")).collect::<String>();
                let new_hand_str = old_hand_str.replace("J", format!("{best_non_js}").as_str());
                let new_hand = Hand::new(new_hand_str.as_str());
                new_hand
            },
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
        let hand = Hand::new(card_str); 
        print!("{card_str}");
        (hand, bid_str.parse::<u32>().unwrap())
    }).collect::<Vec<(Hand,u32)>>()
}
fn idx_to_str(idx:usize)->String{
    let s = match idx{
        12 => "A",
        11 => "K",
        10 => "Q",
        9 => "J",
        8 => "T",
        7 => "9",
        6 => "8",
        5 => "7",
        4 => "6",
        3 => "5",
        2 => "4",
        1 => "3",
        _ => "2"
    };
    s.to_string()
}

pub fn main_2(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut data = process_data_string(data_string);
    data = data.into_iter().map(|(h,c)| (h.consider_joker(),c)).collect::<Vec<(Hand,u32)>>();
    
    data.sort();
    let total = data.iter().enumerate().fold(0, |acc,(idx, (_hand,bid))| acc + (idx as u32+1)* *bid);
    Some(total)
}

#[cfg(test)]
    mod tests{
    use super::*;

    fn joker_tests(){ // Don't work anymore, as intent has changed.
        let base = Hand::new("62345").consider_joker();
        let ans = Hand::new("62345");
        assert_eq!(base,ans, "{base}, {ans}");

        let base = Hand::new("TJTJT").consider_joker();
        let ans = Hand::new("TTTTT");
        assert_eq!(base,ans, "{base}, {ans}");
        
        let base = Hand::new("TJTJT").consider_joker();
        let ans = Hand::new("TTTTT");
        assert_eq!(base,ans, "{base}, {ans}");

        let base = Hand::new("JTJTJ").consider_joker();
        let ans = Hand::new("TTTTT");
        assert_eq!(base,ans, "{base}, {ans}");

        let base = Hand::new("JTJTJ").consider_joker();
        let ans = Hand::new("TTTTT");
        assert_eq!(base,ans, "{base}, {ans}");

        let base = Hand::new("8J552").consider_joker();
        let ans = Hand::new("85552");
        assert_eq!(base,ans, "{base}, {ans}");

        let base = Hand::new("JJJJJ").consider_joker();
        let ans = Hand::new("AAAAA");
        assert_eq!(base,ans, "{base}, {ans} <-");

        let base = Hand::new("JJ826").consider_joker();
        let ans = Hand::new("88826");
        assert_eq!(base,ans, "{base}, {ans}");
        
    }

}
