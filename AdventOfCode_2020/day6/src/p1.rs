use std::{fs::read_to_string, io::BufRead};
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();

    let mut group_count = 0;
    let mut answers = [false;26];
    for line in data_string.lines(){
        if line == ""{
            group_count += answers.iter().fold(0, |acc,ans| if *ans { acc + 1 } else { acc } );
            answers = [false;26];
            continue;
        }
        for c in line.chars(){
            let idx = c as usize - 97; // From 'a' to 'z' -> From 0 to 25
            answers[idx] = true;
        }
    }
    // Add this as the last line is not guaranteed to be empty, so we make sure to add the last group as well.
    group_count += answers.iter().fold(0, |acc,ans| if *ans { acc + 1 } else { acc } );
    
    Some(group_count)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
