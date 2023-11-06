use std::{fs::read_to_string, collections::HashMap};
#[derive(Clone,Copy,Debug)]
pub struct Bits{
    pub bits: [Option<bool>;36]
}
impl Bits{
    pub fn new_from_string(s:&str)->Self{
        if s.len() != 36{
            panic!("String must be 36 characters! >>{s}<< ({})",s.len());
        }
        let bits = s.chars()
            .rev()
            .map(|c|{
                match c{
                    'X' => None,
                    '1' => Some(true),
                    '0' => Some(false),
                    _ => panic!("Invalid char! >>{c}<<")
                }
            }).collect::<Vec<Option<bool>>>()
            .try_into()
            .expect("Failed to convert Vec of size x into arr of size 36!");
        Self{bits}
    }
    pub fn new_from_num(num:u64) -> Self{
        let bits = (0..36)
            .map(|n| Some(((num >> n) & 1) != 0))
            .collect::<Vec<Option<bool>>>()
            .try_into()
            .expect("Failed to convert Vec of size x into arr of size 36!");
        Self{bits}
    }
    pub fn new_blank_mask()->Self{
        Self { bits: [Some(false);36] }
    }
    fn apply_mask(&mut self, mask:&Bits){
        self.bits.iter_mut().zip(mask.bits.iter()).for_each(|(a,b)|{
            if let Some(a_bit) = a{
                if let Some(b_bit) = b{
                    *a_bit = *b_bit;
                }
            }
        })
    }
    pub fn to_int(&self)->u64{
        self.bits
            .iter()
            .enumerate()
            .fold(0u64,|acc, (idx, bit)| {
                if let Some(true) = bit{ 
                    acc + 2u64.pow(idx as u32) 
                }else{ 
                    acc
                }
            })
    }
}

pub fn process_mem_string(s:&str)->(u64,Bits){
    let elements = s.split(" = ").collect::<Vec<&str>>();
    let target_int = elements[1].parse::<u64>().unwrap();
    let mem_id = elements[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap().parse::<u64>().unwrap();
    let bits = Bits::new_from_num(target_int);
    (mem_id,bits)
}
pub fn process_mask_string(s:&str)->Bits{
    let elements = s.split(" = ").collect::<Vec<&str>>();
    let target_str = elements[1];
    Bits::new_from_string(target_str)
}
pub fn main_1(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let mut memory = HashMap::<u64,Bits>::new();
    let mut mask = Bits::new_blank_mask();
    for line in data_string.lines(){
        if line.starts_with("mask"){
            mask = process_mask_string(line);
            continue;
        }
        let (mem,mut bits) = process_mem_string(line);
        bits.apply_mask(&mask);
        memory.insert(mem,bits);
    }
    let total = memory.values().fold(0u64, |acc,v|{
        acc + v.to_int()
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
