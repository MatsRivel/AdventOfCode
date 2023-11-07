use std::fs::read_to_string;
use crate::p1;
pub fn main_2(file_name:&str)->Option<u32>{
    p1::main_1(file_name, 30000000)

}

#[cfg(test)]
    mod tests{
    use super::*;

    // #[test]
    fn my_test(){

    }

}
