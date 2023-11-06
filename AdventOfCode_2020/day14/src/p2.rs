use std::{fs::read_to_string, collections::HashMap};
use crate::p1::{Bits, process_mem_string, process_mask_string};
impl Bits{
    fn depth_first_mask_application(bits: &Bits,mask:&Bits, idx: usize)->Vec<Bits>{
        if idx == 36{
            return vec![*bits];
        }

        let elements = match mask.bits[idx]{
            Some(false) => Bits::depth_first_mask_application(bits, mask, idx+1),
            Some(true) => {
                if let Some(true) = bits.bits[idx]{
                    Bits::depth_first_mask_application(bits, mask, idx+1)
                }else{
                    let mut new_bits = bits.clone();
                    new_bits.bits[idx] = Some(true);
                    Bits::depth_first_mask_application(&new_bits, mask, idx+1)
                }
            },
            None => {
                let mut vec_a = Bits::depth_first_mask_application(bits, mask, idx+1);
                let mut new_bits = bits.clone();
                new_bits.bits[idx] = Some(true);
                let mut vec_b = Bits::depth_first_mask_application(&new_bits, mask, idx+1);
                vec_a.append(&mut vec_b);
                vec_a
            }
        };
        return elements;

    }
    fn apply_floating_mask(&self, mask:&Bits)->Vec<Bits>{
        Bits::depth_first_mask_application(&self, mask, 0)
    }
}
pub fn main_2(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let mut memory = HashMap::<u64,Vec<Bits>>::new();
    let mut mask = Bits::new_blank_mask();
    for line in data_string.lines(){
        if line.starts_with("mask"){
            mask = process_mask_string(line);
            continue;
        }
        let (mem,mut bits) = process_mem_string(line);
        let all_bits = bits.apply_floating_mask(&mask);
        memory.insert(mem,all_bits);
    }
    let total = memory.values().fold(0u64, |acc,possibilities|{
        acc + possibilities.iter().fold(0u64, |acc,v|{
            acc + v.to_int()
        })
        
    });
    Some(total)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
