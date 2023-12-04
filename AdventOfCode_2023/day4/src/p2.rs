use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

use crate::p1::{process_line,read_lines};

struct Card{
    winner_nums: Vec<u32>,
    actual_nums: Vec<u32>
}
impl Card{
    fn new(s:String)->Self{
        let (winner_nums, actual_nums) = process_line(s.as_str());
        Self { winner_nums, actual_nums }
    }
    fn get_number_of_winners(&self)->u32{
        self.actual_nums.iter().filter(|n| self.winner_nums.contains(*n)).count() as u32
    }
}

fn solve_cards(cards: &HashMap<u32,Card>, mut memory: HashMap<u32,u32>, current: u32)->(u32,HashMap<u32,u32>){
    let mut counter = 0;
    let mut mem = memory;
    if let Some(known_solution) = mem.get(&current){
        return (*known_solution, mem);
    }
    if let Some(wcard) = cards.get(&current){
        let n_winners = wcard.get_number_of_winners();
        for winner in current+1..current+1+n_winners{
            let (count, new_mem) = solve_cards(cards, mem, winner);
            mem = new_mem;
            counter += count;
        }
    }
    mem.insert(current, counter+1);
    (counter+1, mem)
}

pub fn main_2(file_name:&str)->Option<u32>{
    let lines = read_lines(file_name).expect("We don't mind crashing this early.");
    let cards = lines
        .filter_map(|line| line.ok() )
        .enumerate()
        .map(|(idx, line)| (idx as u32 +1, Card::new(line)))
        .collect::<HashMap<u32,Card>>();
    // For each card, follow it until we have no winners.
    let total = cards.keys().fold(0,|acc,card_num| acc+ solve_cards(&cards, HashMap::new(),*card_num).0 );
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
