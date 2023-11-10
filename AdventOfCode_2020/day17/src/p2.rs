use std::fs::read_to_string;
use crate::p1::ndim_main;
pub fn main_2(file_name:&str,n_cycles:usize)->Option<i32>{
    ndim_main::<4usize>(file_name,n_cycles)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
