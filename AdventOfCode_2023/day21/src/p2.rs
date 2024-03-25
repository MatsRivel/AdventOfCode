use std::fs::read_to_string;
pub fn main_2(file_name:&str,n_steps:i32)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
