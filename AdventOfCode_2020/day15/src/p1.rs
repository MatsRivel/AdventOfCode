use std::{fs::read_to_string, collections::HashMap};

struct MemNum{
    previously_spoken: Option<u32>,
    recently_spoken: Option<u32>,
}
impl MemNum{
    fn new(idx:u32)->Self{
        Self { previously_spoken: None, recently_spoken: Some(idx)}
    }
}
pub fn main_1(file_name:&str,target_idx:u32)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    // In this case, data_string is a single line!
    let mut last_spoken_num = 0;
    let mut current_idx = 0;
    let mut numbers = {data_string
        .split(",")
        .map(|s| {
            s.parse::<u32>().unwrap()
        })
        .map(|num| {
            last_spoken_num = num;
            current_idx += 1;
            (num,MemNum::new(current_idx))
        }).collect::<HashMap<u32,MemNum>>()
    };
    #[cfg(none)]
    println!("{current_idx}: {last_spoken_num}");
    while current_idx < target_idx{
        current_idx += 1;
        let current = numbers.get(&last_spoken_num).unwrap();
        if let Some(prev_idx) = current.previously_spoken{
            last_spoken_num = current.recently_spoken.unwrap() - prev_idx;
        }else{
            last_spoken_num = 0;
        }
        match numbers.get_mut(&last_spoken_num){
            Some(seen_memnum) => {
                seen_memnum.previously_spoken = seen_memnum.recently_spoken;
                seen_memnum.recently_spoken = Some(current_idx);
            },
            None => {numbers.insert(last_spoken_num,MemNum::new(current_idx));}
        }
        #[cfg(none)]
        println!("{current_idx}: {last_spoken_num}");
    }
    return Some(last_spoken_num);
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn step_by_step_test(){
        let answers = [0,3,6,0,3,3,1,0,4,0];
        for i in 2..10{
            let x = main_1(r"src\dummy.txt", i+1).unwrap();
            assert_eq!(x,answers[i as usize]);
        }
    }

}
