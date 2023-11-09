use std::{fs::read_to_string, collections::HashMap};
use crate::p1::{Template,get_new_template, invalid_ticket_indices, make_tickets, initialize_memory};

pub fn main_2(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let mut lines_iter = data_string.lines().into_iter();
    let template = get_new_template(&mut lines_iter);
    // Add all tickets. The first one is ours.
    let mut tickets = make_tickets(&mut lines_iter);

    let mut memory = initialize_memory(&tickets, &template);
    
    // Some tickets are invalid: Remove them.
    let mut invalid_tickets = invalid_ticket_indices(&tickets, &mut memory, &template).iter().map(|v| v[0]).collect::<Vec<usize>>();
    invalid_tickets.sort();
    // Remove them in reverse order to not have to consider idx changes
    for &remove_idx in invalid_tickets.iter().rev(){
        tickets.remove(remove_idx);
    }
    // Then, go through and fiter out invalid indices.
    for ticket in tickets.iter(){
        for (target_idx,target_number) in ticket.iter().enumerate(){
            for (memory_key, valid_indices) in memory.iter_mut(){
                if !template.in_range(memory_key, target_number){ // TODO: This clone is probably not good. Should refactor.
                    if let Some(remove_idx) = valid_indices.iter().position(|pos| *pos == target_idx){
                        valid_indices.remove(remove_idx);
                    }
                }
            }

        }
    }
    { 
        for (key,val) in memory.iter(){
            println!("{key}: {:?}",val);
        }
        println!();
    }

    // Then we go through and look for lonely indices:
    let mut locked_indices: Vec<usize> = Vec::new();
    let mut change_found = true;
    let mut backup_run = true; // In case ordering causes a false positive, an additional run will always fix this.
    while change_found || backup_run{
        if !change_found{
            backup_run = false;
        }
        change_found = false;
        for (_, valid_indices) in memory.iter_mut(){
            if valid_indices.len() != 1{
                for i in locked_indices.iter(){
                    if let Some(remove_idx) = valid_indices.iter().position(|pos| pos == i){
                        valid_indices.remove(remove_idx);
                        change_found = true;
                    }
                }
            }
            if valid_indices.len() == 1{ // Single idx. Add it, then move on.
                if !locked_indices.contains(&valid_indices[0]){ // We might iterate multiple times, so we shouldnt keep pusing if not needed.
                    locked_indices.push(valid_indices[0]);
                }
                continue;
            } 
        }
    }

    {   
        for (key,val) in memory.iter(){
            println!("{key}: {:?}",val);
        }
    }
    let total_product = memory.iter().fold(1u64, |acc,(name,v)| {
        #[cfg(test)]{
            tickets[0][v[0]] as u64 * acc
        }
        #[cfg(not(test))]{
            print!("\t{name}: {v:?}");
            if name.starts_with("departure"){
                println!("<---");
                tickets[0][v[0]] as u64 * acc
            }else{
                println!();
                acc
            }
        }
    });
    Some(total_product)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
