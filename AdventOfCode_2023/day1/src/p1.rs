use std::fs::read_to_string;
pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let output = data_string
        .lines()
        .map(|line| {
            let nums = line.chars().filter_map(|c| {
                c.to_digit(10)
            })
            .collect::<Vec<u32>>();
            nums[0]*10 + nums.last().unwrap()
        })
        .fold(0, |acc,v| acc+v);
    Some(output)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
