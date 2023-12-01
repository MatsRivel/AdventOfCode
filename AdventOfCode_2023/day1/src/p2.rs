use std::fs::read_to_string;

fn str_starts_with_num(s:&str)->Option<u32>{
    if let Ok(v) = s[0..1].parse::<u32>(){
        return Some(v);
    }else if s.starts_with("one"){
        return Some(1);
    }else if s.starts_with("two"){
        return Some(2);
    }else if s.starts_with("three") {
        return Some(3);
    }else if s.starts_with("four") {
        return Some(4);
    }else if s.starts_with("five") {
        return Some(5);
    }else if s.starts_with("six") {
        return Some(6);
    }else if s.starts_with("seven") {
        return Some(7);
    }else if s.starts_with("eight") {
        return Some(8);
    }else if s.starts_with("nine") {
        return Some(9);
    }else if s.starts_with("zero") {
        return Some(0);
    }
    None
}
fn str_ends_with_num(s:&str) -> Option<u32> {
    if let Ok(v) = s[s.len()-1..s.len()].parse::<u32>(){
        return Some(v);
    }else if s.ends_with("one"){
        return Some(1);
    }else if s.ends_with("two"){
        return Some(2);
    }else if s.ends_with("three") {
        return Some(3);
    }else if s.ends_with("four") {
        return Some(4);
    }else if s.ends_with("five") {
        return Some(5);
    }else if s.ends_with("six") {
        return Some(6);
    }else if s.ends_with("seven") {
        return Some(7);
    }else if s.ends_with("eight") {
        return Some(8);
    }else if s.ends_with("nine") {
        return Some(9);
    }else if s.ends_with("zero") {
        return Some(0);
    }
    None
}

fn first_and_last_num_of_line(line:&str)-> [u32;2]{
    let mut left_val = None;
    let mut right_val = None;
    for left in 0..(line.len()){
        let section = &line[left..];
        if let Some(v)  = str_starts_with_num(section){
            left_val = Some(v);
            break
        }
    }
    for right in (0..(line.len())).rev(){
        let section = &line[..right+1];
        if let Some(v)  = str_ends_with_num(section){
            right_val = Some(v);
            break
        }
    };

    [left_val.expect("left is not none"),right_val.expect("right is not none")]
}

pub fn main_2(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let output = data_string
        .lines()
        .map(|line|{
            let nums = first_and_last_num_of_line(line);
            nums[0]*10 + nums[1]
        })
        .fold(0, |acc,v| acc+v);
    Some(output)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn num_test(){
        let text = ["zero","one","two","three","four","five","six","seven","eight","nine"];
        let nums = [0u32,1,2,3,4,5,6,7,8,9];
        for (text,num) in text.iter().zip(nums){
            assert_eq!(first_and_last_num_of_line(format!("{text}{num}").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaa{text}aaa{num}aaaa").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("{num}{text}").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaa{num}aaa{text}aaaa").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaaaaa{text}").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaaaaa{text}aaaaaa").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("{text}aaaaaa").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaaaaa{num}").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("aaaaaa{num}aaaaaa").as_str()), [num,num]);
            assert_eq!(first_and_last_num_of_line(format!("{num}aaaaaa").as_str()), [num,num]);

        }
    }

}
