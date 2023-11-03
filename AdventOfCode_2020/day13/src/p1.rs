use core::panic;
use std::{fs::read_to_string, cmp::Ordering};
#[derive(Debug)]
pub enum BusType{
    RealBus(Bus),
    FakeBus
}

pub trait InnerBus{ // Only usable if you know there are only BusType::RealBus in the list.
    fn get_inner(&mut self)-> &mut Bus;
}
impl InnerBus for BusType{
    fn get_inner(&mut self)->&mut Bus {
        match self{
            BusType::RealBus(ref mut bus) => bus,
            BusType::FakeBus => panic!(),
        }
    }
}
impl BusType{
    fn new(s:&str) -> Self{
        match Bus::new(s){
            Some(b) => Self::RealBus(b),
            None => Self::FakeBus,
        }
    }
}
#[derive(Debug)]
pub struct Bus{
    pub bus_id: i32,
    pub current_time: i32,
    next_time: i32,
}
impl Iterator for Bus{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_time = self.next_time;
        self.next_time = self.next_time + self.bus_id;
        Some(self.current_time)
    }
}
impl Bus{
    fn new(s:&str)->Option<Self>{
        match s.parse::<i32>(){
            Ok(v) => Some(Self{ bus_id:v, current_time: 0, next_time: 0+v}),
            Err(_) => None,
        }
    }
}
pub fn process_data_string(data_string:&str)->(i32, Vec<BusType>){
    // The 3rd part is to deal with the empty line at the bottom.
    let split = data_string.split("\r\n").collect::<Vec<&str>>();
    let [start_time_str,bus_route_str] = [split[0],split[1]];

    let start_time = start_time_str.parse::<i32>().expect(format!(">> {start_time_str} <<").as_str());
    let busses = bus_route_str
        .split(",")
        .filter_map(|s|{
            Some(BusType::new(s))
        }).collect::<Vec<BusType>>();
    
    (start_time,busses)
    
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (start_time,mut busses): (i32,Vec<BusType>) = process_data_string(&data_string);

    #[cfg(test)]
    println!("Start time: {start_time}");
    let bus_iterator = busses.iter_mut().filter(|b|{
        match b{
            BusType::RealBus(_b) => true,
            BusType::FakeBus => false,
        }
    });
    for bus in bus_iterator{
        while bus.get_inner().current_time < start_time{
            bus.get_inner().next();
        }
    }
    busses.sort_by(|a,b|{
        match (a,b){
            (BusType::RealBus(x), BusType::RealBus(y)) => x.current_time.cmp(&y.current_time),
            (BusType::RealBus(_), BusType::FakeBus) => std::cmp::Ordering::Less,
            (BusType::FakeBus, _)  => Ordering::Equal,
        }
    });

    let output = (busses[0].get_inner().current_time - start_time) * busses[0].get_inner().bus_id; 
    Some(output)
    

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
