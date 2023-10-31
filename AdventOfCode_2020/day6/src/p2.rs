use std::fs::read_to_string;

// Just playing around with test and non-test configs. Not the cleanest way to do something, obviously.
#[cfg(test)]
fn answer_buffer()->Vec<u32>{
    vec![0u32;3]
}
#[cfg(not(test))]
fn answer_buffer()->Vec<u32>{
    vec![0u32;26]
}
pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut group_size = 0;
    let mut group_count = 0;
    let mut answers = answer_buffer();
    for line in data_string.lines(){
        if line == ""{
            group_count += answers.iter().fold(0, |acc,ans| {
                if *ans == group_size { 
                    acc + 1
                 }else { 
                    acc
                 } 
            });
            answers = answer_buffer();
            group_size = 0;
            continue;
        }
        for c in line.chars(){
            let idx = c as usize - 97; // From 'a' to 'z' -> From 0 to 25
            answers[idx] += 1;
        }
        group_size += 1;

    }
    // Add this as the last line is not guaranteed to be empty, so we make sure to add the last group as well.
    group_count += answers.iter().fold(0, |acc,ans| {
        if *ans == group_size { acc + 1 } else { acc } 
    });
    #[cfg(test)]
    println!("{group_count}");

    
    Some(group_count)

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
