use std::{fs::read_to_string, collections::HashMap, fmt::Display};
use crate::p1::{Bits, process_mem_string, process_mask_string};
impl Display for Bits{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"'{}'",self.to_int())
    }
}
impl Bits{
    fn len(&self)->usize{
        self.bits.len()
    }
    fn depth_first_mask_application(bits: &Bits,mask:&Bits, idx: usize)->Vec<u64>{
        if idx == mask.len(){
            #[cfg(none)]{
                println!("{:?}",bits.to_int());
            }
            return vec![bits.to_int()];
        }
        let current_sum = match mask.bits[idx]{
            Some(false) => {
                let v = Bits::depth_first_mask_application(bits, mask, idx+1);

                v
            },
            Some(true) => {
                let v;
                if let Some(true) = bits.bits[idx]{
                    v = Bits::depth_first_mask_application(bits, mask, idx+1)
                }else{
                    let mut new_bits = bits.clone();
                    new_bits.bits[idx] = Some(true);
                    v = Bits::depth_first_mask_application(&new_bits, mask, idx+1)
                }
                v
            },
            None => {
                // Run with the original bit:
                let mut v_a = Bits::depth_first_mask_application(bits, mask, idx+1);
                // Make a clone, flip a bit, and run with that version too:
                let mut new_bits = bits.clone();
                new_bits.bits[idx] = Some(!new_bits.bits[idx].unwrap());
                let mut v_b = Bits::depth_first_mask_application(&new_bits, mask, idx+1);
                v_a.append(&mut v_b);
                v_a
            }
        };
        return current_sum;

    }
    fn apply_floating_mask(&self, mask:&Bits)->Vec<u64>{
        Bits::depth_first_mask_application(&self, mask, 0)
    }
}

pub fn main_2(file_name:&str) -> Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    // Memory uses a memory idx as a key, and then stores the mask to use and the value to apply the mask to.
    let mut memory = HashMap::<u64,u64>::new();
    let mut mask = Bits::new_blank_mask(); // This is safe as there is always a mask at the first line :)
    
    for line in data_string.lines(){
        if line.starts_with("mask"){
            mask = process_mask_string(line);
            continue;
        }
        let (mem,bits) = process_mem_string(line);
        let mem_bits = Bits::new_from_num(mem);
        let all_mems = mem_bits.apply_floating_mask(&mask);
        for m in all_mems.iter() {
            let val = bits.to_int();
            memory.insert(*m,val);
        }
    }
    // Then we actually process the changes:
    let mut sum = 0;
    for num in memory.values(){
        sum += num;
    }
    Some(sum)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn mem_mask_test1(){
        let mut mask = Bits::new_from_string("000000000000000000000000000000X1001X");
        let mut addr = Bits::new_from_num(42);
        let all_mems = addr.apply_floating_mask(&mask);
        for known_val in [26,27,58,59].iter(){
            assert!(all_mems.contains(known_val),"\n>> Value {known_val} should have occured as a permutation of mem={addr}, but it did not occur!\n{all_mems:?} <<\n")
        }
    }
    #[test]
    fn mem_mask_test2(){
        let mut mask = Bits::new_from_string("00000000000000000000000000000000X0XX");
        let mut addr = Bits::new_from_num(26);
        let all_mems = addr.apply_floating_mask(&mask);
        for known_val in [16,17,18,19,24,25,26,27].iter(){
            assert!(all_mems.contains(known_val),"\n>> Value {known_val} should have occured as a permutation of mem={addr}, but it did not occur!\n{all_mems:?} <<\n")
        }
    }

}
