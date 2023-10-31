use std::{fs::read_to_string, collections::HashMap};

use crate::p1::{process_line, Bag};



fn depth_first_count(current_bag_name: &String, bags_with_shiny: &mut HashMap<String,i32>, bags: & HashMap<String,Bag>)->i32{
    let current_bag = bags.get(current_bag_name).expect("We know that this bag exists");
    if current_bag.contents.len() == 0{
        #[cfg(test)]
        println!("{current_bag_name}: 0");
        return 0;
    }
    let mut current_bag_count = *bags_with_shiny.get(current_bag_name).expect("We know this bag exists");
    if current_bag_count != 0{
        return current_bag_count;
    }
    for (count, child_bag_name) in &current_bag.contents{
        let child_bag_contains = depth_first_count(child_bag_name, bags_with_shiny, bags);
        current_bag_count += (child_bag_contains+1) * (*count as i32);
    }

    bags_with_shiny.insert(current_bag_name.clone(),current_bag_count);
    #[cfg(test)]
    println!("{current_bag_name}: {current_bag_count}");

    current_bag_count
}


pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let bags = data_string
        .lines()
        .map(|line|{
            let bag = process_line(line);
            (bag.name.clone(), bag)
        }).collect::<HashMap<String,Bag>>();
    
    let mut count_bags = bags.keys().map(|bag_name| (bag_name.clone(),0)).collect::<HashMap<String,i32>>();
    let target_bag = "shiny gold".to_string();
    let bags_in_shiny_bag = depth_first_count(&target_bag, &mut count_bags, &bags);
    Some(bags_in_shiny_bag)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
