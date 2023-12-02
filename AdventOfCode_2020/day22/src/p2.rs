use std::{fs::read_to_string, collections::VecDeque};
pub fn main_2(file_name:&str)->Option<i32>{
    let mut q1 = VecDeque::new();
    let mut q2 = VecDeque::new();
    let nums = read_to_string(file_name)
        .unwrap()
        .lines()
        .into_iter()
        .filter_map(|s|{
            s.parse::<u32>().ok()
        }).collect::<Vec<u32>>();
    for (idx, num) in nums.iter().enumerate(){
        if idx < nums.len() / 2{
            q1.push_back(*num);
            println!("{num}");
        }else{
            q2.push_back(*num);
            println!("\t{num}");
        }
    }
    while !q1.is_empty() && !q2.is_empty(){
        let c1 = q1.pop_front().unwrap();
        let c2 = q2.pop_front().unwrap();
        if c1 > c2{
            q1.push_back(c1);
            q1.push_back(c2)
        }else{
            q2.push_back(c2);
            q2.push_back(c1)
        }
    }
    let total_score;
    let mut counter = 0;
    if q1.is_empty(){
        total_score = q2.iter().rev().fold(0, |acc,v| {
            counter += 1;
            acc + v*counter
        })
    }else{
        total_score = q1.iter().rev().fold(0, |acc,v| {
            counter += 1;
            acc + v*counter
        })
    }
    Some(total_score.try_into().unwrap())

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
