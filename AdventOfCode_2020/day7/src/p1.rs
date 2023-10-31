use std::{fs::read_to_string, collections::HashMap};

pub struct Bag{
    pub name: String,
    pub contents: Vec<(usize,String)>,
}

pub fn process_line<'a>(line:&'a str)->Bag{
    let elements = line.split(" ")
        .enumerate()
        .filter_map(|(idx,s)|{
            // Filters the empty bags out of the pattern.
            let empty_bag = match (idx,s){
                    (4,"no") => true,
                    (5, "other") => true,
                    (6, "bags.") => true,
                    _ => false
            };
            if empty_bag{
                None
            }else if idx < 2 {// Always keep the first two elements, as they represent the colour of the bag.
                Some(s)
            }else if idx > 3 && (idx%4 == 0 || idx%4 == 1 || idx%4 == 2 ){ // Any "added" bags:
                Some(s)
            }else{
                None
            }
        })
        .collect::<Vec<&str>>();
    let name = format!("{} {}", elements[0], elements[1]);
    let mut contents = Vec::new();
    #[cfg(test)]
    println!("{elements:?}");
    for idx in (2..elements.len()).step_by(3){
        let count = elements[idx].parse::<usize>().expect(format!("Should be digit: elements[{}] =  {}", idx, elements[idx]).as_str());
        let target = format!("{} {}", elements[idx+1], elements[idx+2]);
        contents.push((count,target));
    }
    Bag { name, contents }
    
}

fn depth_first_search(target_bag_name: &String, current_bag_name: &String, bags_with_shiny: &mut HashMap<String,bool>, bags: & HashMap<String,Bag>)->bool{
    if current_bag_name == target_bag_name{
        return true;
    }
    #[cfg(test)]
    println!("Current: {current_bag_name}");
    let current_bag = bags.get(current_bag_name).expect("We know that this bag exists");
    for (_count, bag_name) in &current_bag.contents{
        if Some(&true) == bags_with_shiny.get(bag_name){ // If we've already verified this bag has a golden bag in it:
            bags_with_shiny.insert(current_bag_name.clone(), true);
            return true;
        }
        // If the current bag does not have any shiny evidende, check its inner content too:
        let shiny_evidence = depth_first_search(target_bag_name, &bag_name, bags_with_shiny, bags);
        if shiny_evidence{
            bags_with_shiny.insert(current_bag_name.clone(), true);
        }
    }
    *bags_with_shiny.get(current_bag_name).expect("We know the bag exists")
}
fn search_for_bag(target_bag: &String, bags_with_shiny: &mut HashMap<String,bool>, bags: & HashMap<String,Bag>){
    for bag in bags.keys(){
        _ = depth_first_search(target_bag, bag, bags_with_shiny, bags)
    }
}

pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let bags = data_string
        .lines()
        .map(|line|{
            let bag = process_line(line);
            (bag.name.clone(), bag)
        }).collect::<HashMap<String,Bag>>();
    
    let mut count_bags = bags.keys().map(|bag_name| (bag_name.clone(),false)).collect::<HashMap<String,bool>>();
    let target_bag = "shiny gold".to_string();
    search_for_bag(&target_bag, &mut count_bags, &bags);
    #[cfg(test)]
    println!();
    let bags_with_gold = count_bags.iter().fold(0, |acc,(name,has_gold)|{
        #[cfg(test)]
        println!("{name}: {has_gold}");
        if *has_gold{
            acc+1
        }else{
            acc
        }
    });
    Some(bags_with_gold)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
