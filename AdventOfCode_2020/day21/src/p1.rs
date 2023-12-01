use std::{fs::read_to_string, collections::{HashMap, HashSet, VecDeque}};
type Allergen = String;
type Food = String;
/*
Food contains allergens.
Each food contains ONE or ZERO allergens.
Each allergen is found in exactly ONE food.
If the allergen is listed, it is definetly in one of the foods.
If an allergen is NOT listed, it MIGHT be in the food.
*/

fn double_vec_to_arr(vec_in:Vec<Vec<String>>) -> [Vec<String>;2]{
    let output: [Vec<String>; 2] = vec_in.try_into().unwrap();
    output
}
fn single_vec_to_arr(mut vec_in:Vec<Vec<String>>) -> [Vec<String>;2]{
    let initial = vec_in.pop().unwrap();
    let output = [initial, vec![]];
    output
}
#[derive(Debug)]
enum AllergenStatus{
    Defined(Food),// Either it definetly has one, or it defietly does not.
    OneOf(Vec<Allergen>), // We know it is an allergen, but not which.
}
impl AllergenStatus{
    fn is_defined(&self)->bool{
        match self{
            AllergenStatus::Defined(_) => true,
            AllergenStatus::OneOf(_) => false,
        }
    }
    fn define_if_possible(&mut self){
        match self{
            AllergenStatus::Defined(_) =>(),
            AllergenStatus::OneOf(list) => {
                if list.len() == 1{
                    let new_food = list[0].clone();
                    *self = AllergenStatus::Defined(new_food);
                }
            },
        }
    } 
    fn get_defined_value(&self) ->Option<Food>{
        match self{
            AllergenStatus::Defined(v) => Some(v.clone()),
            _ => {
                panic!("Shouldn't happen in part1 at least...");
                None
            }
        }
    }
}

fn process_allergen_side(allergen_text:&str) -> Vec<String>{
    let clean_s: Vec<String>;
    if let Some(new_s) = allergen_text.strip_suffix(')'){
        clean_s = new_s.split(", ").filter(|&txt| txt != "").map(|inner_s| inner_s.to_string()).collect::<Vec<String>>();
    }else{
        clean_s = allergen_text.split(", ").filter(|&txt| txt != "").map(|inner_s| inner_s.to_string()).collect::<Vec<String>>();
    }
    clean_s
}
fn process_food_side(food_text:&str) -> Vec<String>{
        food_text.split(" ").filter(|&txt| txt != "").map(|inner_s| inner_s.to_string()).collect::<Vec<String>>()
}
fn process_line(line:&str)->[Vec<String>; 2]{
    let initial_split = line
        .split(" (contains ")
        .map(|s| {
            if s.contains(')'){
                process_allergen_side(s)
            }else{
                process_food_side(s)
            }
        }).collect::<Vec<Vec<String>>>();

        if initial_split.len() == 2{
            double_vec_to_arr(initial_split)
        }else{
            single_vec_to_arr(initial_split)
        }

}
fn get_data(data_string:String)->Vec<[Vec<String>;2]>{
    let output = data_string
        .lines()
        .map(|line| {
            process_line(line)
        }).collect::<Vec<[Vec<String>;2]>>();
    output

}
fn get_content_that_exists_in_all_instances(data: &mut Vec<Vec<String>>) -> Vec<String>{
    if data.is_empty(){
        return vec![];
    }
    let mut valids = data
        .iter()
        .fold(Vec::<String>::new(), |mut acc, val| {
            acc.append(&mut val.clone());
            acc
        });
    valids.sort();
    valids.dedup(); // Remove duplicates from sorted list.
    for food_combination in data.iter(){
        let mut to_be_removed = vec![];
        for (idx, valid) in valids.iter().enumerate(){
            if !food_combination.contains(valid){
                to_be_removed.push(idx);
            }
        }
        to_be_removed.sort();
        for idx in to_be_removed.iter().rev(){
            valids.remove(*idx);
        }
    }
    valids

}

pub fn main_1(file_name:&str)->Option<i32>{
    // Extract data from file:
    let data_string = read_to_string(file_name).unwrap();
    let data = get_data(data_string);
    let mut allergen_data: HashMap<Allergen,Vec<Vec<Food>>> = HashMap::new();
    // Go through the data to store it in a more conventient way.
    for [foods,allergens] in data.iter() {
        for allergen in allergens.iter(){
            if allergen_data.contains_key(allergen){
                allergen_data.get_mut(allergen).unwrap().push(foods.clone());
            }else{
                allergen_data.insert(allergen.clone(), vec![foods.clone()]);
            }
        }
    }
    
    // Clean up the data. Once an allergen is defined, the food it corresponds to MUST be in the list.
    // So we remove all foods that do NOT occur in every instance of known allergens.
    let cleaned_allergen_data: HashMap<Allergen, Vec<Food>> = allergen_data
        .iter_mut()
        .map(|(key,val)|{
            let valid_foods = get_content_that_exists_in_all_instances(val);
            (key.clone(), valid_foods)
        }).collect::<HashMap<Allergen,Vec<Food>>>();
    

    let mut remove_from_consideration: Vec<Food> = Vec::new();
    let mut allergens: HashMap<Allergen, AllergenStatus> = cleaned_allergen_data
        .iter()
        .map(|(key,val)|{
            if val.len() == 1{
                remove_from_consideration.push(val[0].clone());
                (key.clone(), AllergenStatus::Defined(val[0].clone()))
            }else{
                (key.clone(), AllergenStatus::OneOf(val.clone()))
            }
        }).collect::<HashMap<Allergen, AllergenStatus>>();

    
    while remove_from_consideration.len() != 0{
        let mut new_consideration: Vec<Food> = vec![];
        let mut allergen_to_check = VecDeque::<Allergen>::new();

        for (allergen, status) in allergens.iter_mut(){
            // Only care about non-defined allergens:
            if let AllergenStatus::OneOf(list) = status{
                let mut to_remove = VecDeque::<usize>::new();
                // If any food in the list has been defined, remove it from this allergens pool of options.
                for (idx,food) in list.iter().enumerate(){
                    if remove_from_consideration.contains(food){
                        to_remove.push_front(idx); // Pushihng higher indices to front so we remove the higher first. This prevents interference with lower indices.
                    }
                }
                // Remove it here
                for idx in to_remove.into_iter(){
                    list.remove(idx);
                }
                // To satisfy the borrowchecker, we cant mutate while iterating over what we want to mutate.
                // Therefor we save what we want to mutate, then mutate it later.
                if list.len() == 1{
                    allergen_to_check.push_back(allergen.clone());
                }
            }
        }
        // If a food is now defined, mark it as such.
        // Any previously not defined foods will now go through the same loop.
        for allergen in allergen_to_check.into_iter(){
            allergens.get_mut(&allergen).unwrap().define_if_possible();
            match allergens.get_mut(&allergen).unwrap(){
                AllergenStatus::Defined(food) => new_consideration.push(food.clone()),
                AllergenStatus::OneOf(_) => (),
            }
        }
        remove_from_consideration = new_consideration;
    }
    println!("{allergens:#?}");
    // Now everything should be defined (NOTE: Edge-case where two allergens have two foods they can both be. Not considered here)
    let known_allergens = allergens.values().into_iter().filter_map(|v| v.get_defined_value() ).collect::<Vec<Food>>();
    let clean_food_counter = data
        .iter()
        .map(|[foods,_]|{
            foods
        }).fold(0, |acc,foods|{
            println!{"{foods:?}"};
            let clean_count = foods
                .iter()
                .fold(0, |counter,value| {
                    if known_allergens.contains(value){
                        counter
                    }else{
                        counter+1
                    }
                });
            acc + clean_count
        });
    
    Some(clean_food_counter)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
