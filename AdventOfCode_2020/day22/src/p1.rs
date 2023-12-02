use std::{fs::read_to_string, collections::HashSet};
enum Winner{
    PlayerOne,
    PlayerTwo
}
enum WinnerQueue{
    One(QueueWithState),
    Two(QueueWithState)
}
#[derive(Clone,Debug)]
struct QueueWithState{
    data: Vec<u32>,
}
impl QueueWithState {
    fn new() -> Self{
        Self{data:vec![]}
    }
    fn pop(&mut self)-> u32{
        self.data.remove(0) // Performance is poor, but we have to iterate over everything to find if the state is unique anyways...
    }
    fn push(&mut self,v:u32){
        self.data.push(v) 
    }
    fn is_empty(&self) -> bool{
        self.data.is_empty()
    }
    fn iter(self)->std::vec::IntoIter<u32>{
        self.data.into_iter()
    }
    fn get_state(&self) ->String{
        format!("{:?}",self.data)
    }
}

fn calculate_score(q: QueueWithState)-> u32 {
    let total_score;
    let mut counter = 0;
        total_score = q.iter().rev().fold(0, |acc,v| {
            counter += 1;
            acc + v*counter
        });
    total_score
}
fn prep_game(file_name:&str) -> [QueueWithState;2]{
    let mut q1 = QueueWithState::new();
    let mut q2 = QueueWithState::new();
    let nums = read_to_string(file_name)
        .unwrap()
        .lines()
        .into_iter()
        .filter_map(|s|{
            s.parse::<u32>().ok()
        }).collect::<Vec<u32>>();
    
    for (idx, num) in nums.iter().enumerate(){
        if idx < nums.len() / 2{
            q1.push(*num);
        }else{
            q2.push(*num);
        }
    }
    [q1,q2]
}

fn play_round(q1: &mut QueueWithState, q2: &mut QueueWithState ){
    // Draw Card
    // Determine Winner
    // Push cards to back of winner.
    let c1 = q1.pop();
    let c2 = q2.pop();
    if c1 > c2{
        q1.push(c1);
        q1.push(c2);
    }else{
        q2.push(c2);
        q2.push(c1);
    }
}
fn play_game(q1: QueueWithState, q2: QueueWithState)->WinnerQueue{
    // Start a whole new game.
    // Note: Needs a state-tracker too! This should go into each round, but is refreshed for each game.
    let mut state_tracker:HashSet<String> = HashSet::new();
    state_tracker.insert(q1.get_state());
    state_tracker.insert(q2.get_state());
    todo!()
}
fn initialize_game(q1: QueueWithState, q2: QueueWithState)->u32{
    let winner = play_game(q1, q2);
    let total_score = match winner{
        WinnerQueue::One(q) => calculate_score(q),
        WinnerQueue::Two(q) => calculate_score(q)
    };
     total_score
}
pub fn main_1(file_name:&str)->Option<i32>{
    let [q1, q2] = prep_game(file_name);
    let total_score = initialize_game(q1,q2);
    Some(total_score.try_into().unwrap())

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
