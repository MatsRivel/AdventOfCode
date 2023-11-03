use std::fs::read_to_string;
use crate::p1::{process_data_string,BusType};

pub fn main_2(file_name:&str)->Option<i64>{
    // Note to self:
    // A better solution might be to do prime factorialization. Ignore any components shared by all.
    // Use that as the step-size.
    let data_string = read_to_string(file_name).unwrap();
    // Start_time is no longer relevant, so we ignore it.
    let (_,mut busses): (i32,Vec<BusType>) = process_data_string(&data_string);
    let reference = busses.iter().enumerate().filter_map(|(i, b)|{
        match b{
            BusType::RealBus(rb) => Some([i as i64, rb.bus_id as i64]),
            BusType::FakeBus => None,
        }
    }).collect::<Vec<[i64;2]>>();
    let mut current_time = 0; 
    'outer: loop{
        current_time += reference[0][1];
        #[cfg(test)]{
            if current_time > 1_068_781{
                panic!("Too high! {current_time}> 1068781!");
            }
        }
        for (i,[skew, time_step_size]) in reference.iter().enumerate(){
            if (current_time + skew) % time_step_size != 0{
                println!("{i}: {current_time}+{skew} % {time_step_size} == {}", (current_time + skew) % time_step_size);
                continue 'outer;
            }
        }
        break 'outer; // Found a match!
    }
    Some(current_time)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
