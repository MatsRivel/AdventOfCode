use core::panic;
use std::fs::read_to_string;
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut adapters = data_string.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    adapters.sort();
    let mut one_differences = 0;
    let mut three_differences = 1; // The last 1 is 3+ our highest adapter.
    if adapters[0] == 1{
        one_differences += 1;
    }else if adapters[0] == 3{
        three_differences += 1;
    }else if adapters[0] == 0 || adapters[0] == 2{
        ()// These are ok, but not counted.
    }else{
        panic!("Socket i 0, so anythign > 3 causes an error.")
    }
    for i in 1..adapters.len() {
        if adapters[i] - adapters[i-1] == 3{
            three_differences += 1;
        }else if adapters[i] - adapters[i-1] == 1{
            one_differences += 1;
        }
    }
    Some(one_differences*three_differences)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
