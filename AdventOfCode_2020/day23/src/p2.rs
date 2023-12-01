use std::{fs::read_to_string, collections::VecDeque};
#[derive(Debug,Clone)]
struct BigQ{
    ql: VecDeque<u64>,
    qr: VecDeque<u64>,
    next_l: u64,
    next_r: u64
}
impl BigQ {

}
#[derive(Debug,Clone)]
struct Crab{
    rot: BigQ,
    mem: VecDeque<u64>,

}
impl Crab{
    fn new(s:&str) -> Self{
        let ql = s.chars().filter_map(|c| c.to_digit(10)).map(|n| n as u64).collect::<VecDeque<u64>>();
        let rot = BigQ{ql, qr: VecDeque::<u64>::new(), next_r:10, next_l:1_000_000};
        let mem = VecDeque::<u64>::new();
        Self{rot, mem}
    }
    fn cycle_clockwise(&mut self){
        todo!()
    }
    fn cycle_counter_clockwise(&mut self){
        todo!()
    }
    fn pick_up_3(&mut self){
        todo!()
    }
    fn get_current(&self) -> u64{
        todo!()
    }
    fn insert_mem_at_val(&mut self, target_val: u64){
        todo!()
    }
    fn get_destination(&self)->u64{
        todo!()
    }
    #[cfg(test)]
    fn get_final_order(&self)->u32{
        todo!()
    }
    #[cfg(not(test))]
    fn get_final_order(&mut self)->u32{
        todo!()
    }

}

pub fn main_2(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut crab = Crab::new(&data_string);
    for _i in 1..=1_000_000{
        crab.pick_up_3();
        let desitnation  = crab.get_destination();
        crab.insert_mem_at_val(desitnation);
        crab.cycle_clockwise();

    }
    return Some(crab.get_final_order());

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
