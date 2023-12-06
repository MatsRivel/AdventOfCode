use std::fs::read_to_string;

use crate::p1::{Race,dist,ways_to_win_race};

fn get_race(data_string:String)->Race{
    let [time_str, distance_str]: [&str;2] = data_string.lines().collect::<Vec<&str>>().try_into().unwrap();
    let time = time_str.split_whitespace().skip(1).collect::<String>().parse::<u128>().unwrap();
    let dist = distance_str.split_whitespace().skip(1).collect::<String>().parse::<u128>().unwrap();
    Race { time, dist }
}
pub fn main_2(file_name:&str)->Option<u128>{
    let data_string = read_to_string(file_name).unwrap();
    let race = get_race(data_string);
    let ways_to_win = ways_to_win_race(&race);
    Some(ways_to_win)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
