use std::fs::read_to_string;
use crate::p1::{process_data_string,BusType};
//    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997];

pub fn all_divisible(time:i64, reference: &Vec<[i64;2]>)->bool{
    for [modifier,step_size] in reference.iter() {
        if (time) % step_size != 0{
            return false;
        }
    }
    true
}
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

    let step_size = reference.iter().fold(1, |acc,[modifier,step_size]|{
        acc * step_size
    });
    println!("Step_size: {step_size}");
    #[cfg(test)]{
        assert!(step_size <= 1068781, "Step_size is larger than the value we know we're looking for!")
    }
    let mut time = step_size;
    while !all_divisible(time, &reference){
        println!("{time:?}");
        time += step_size;
        #[cfg(test)]{
            if time > 1068781{
                panic!("Surpassed target! >> {time:?} > 1068781 <<");
            }
        }
    }
    Some(time)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn prime_test(){

    }

}
