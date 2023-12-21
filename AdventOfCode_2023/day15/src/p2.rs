use std::fs::read_to_string;
use crate::p1::hash_function;
use std::collections::HashMap;
enum Operation{
    Remove,
    Insert(u128)
}
impl Operation{
    fn new(s:&str)->Self{
        if s.contains("-"){
            Self::Remove
        }else{
            let new_s = s.split("=").collect::<Vec<&str>>()[1];
            let number = match new_s.parse::<u128>(){
                Ok(v) => v,
                Err(e) => panic!("{e}\n{s}"),
            };
            Self::Insert(number)
        }
    }
}
pub fn main_2(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let tasks = data_string
        .lines()
        .flat_map(|line| line.split(",") )
        .map(|s| {
            let label = &s[..2];
            let hash = hash_function(&s[..2]) as usize;
            let operation = Operation::new(s);
            (label,hash,operation)
        });//.collect::<Vec<(u128,Operation)>>();

    let mut boxes = vec![Vec::<(&str,u128)>::with_capacity(9);256];
    
    for (label, box_idx, operation) in tasks{
        let current_box = boxes.get_mut(box_idx).expect("We know all the boxes exist");
        match operation{
            Operation::Remove => {
                let mut idx_to_remove = None;
                for (other_idx, (other_label,_)) in current_box.iter().enumerate(){
                    if other_label == &label{
                        idx_to_remove = Some(other_idx);
                        break;
                    }
                }
                if let Some(idx) = idx_to_remove{
                    current_box.remove(idx);
                }
            },
            Operation::Insert(insert_value) => {
                let label_in_current = current_box.iter().fold(false, |acc,(inner_label,_)| acc || inner_label==&label);
                if label_in_current{
                    for (inner_label,inner_value) in current_box.iter_mut(){
                        if inner_label == &label{
                            *inner_value = insert_value;
                            break;
                        }
                    }
                }else{
                    current_box.push( ( label, insert_value ) );
                }

            },
        }
    }
    let mut total = 0;
    for (box_idx, box_content) in boxes.into_iter().enumerate(){
        #[cfg(test)]{
            if !box_content.is_empty(){
                println!("Box {box_idx}: {:?}",box_content);
            }
        }
        for (lense_idx, (_lense_label, focal_length)) in box_content.into_iter().enumerate(){
            let focusing_power = (box_idx+1)*(lense_idx+1)*(focal_length as usize);
            #[cfg(test)]
            println!("\t{} * ({_lense_label}){} * {focal_length} = {focusing_power}",box_idx+1,lense_idx+1);
            total += focusing_power;
        }
    }
    panic!();
    Some(total)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
