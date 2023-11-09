use std::{fs::read_to_string, collections::HashMap, ops::RangeInclusive, str::Lines};
pub struct Template{
    pub template: HashMap<String,[RangeInclusive<u32>;2]>,
}
impl Template{
    pub fn new(template_info:Vec<&str>) -> Self{
        let keys_and_vals = template_info.iter()
            .map(|&s|{
                let [key,v_str]:[&str;2] = s.split(": ").collect::<Vec<&str>>().try_into().unwrap();
                let [left_range, right_range]:[&str;2] = v_str.split(" or ").collect::<Vec<&str>>().try_into().unwrap();
                let [lower_left, upper_left]:[&str;2] = left_range.split("-").collect::<Vec<&str>>().try_into().unwrap();
                let [lower_right, upper_right]:[&str;2] = right_range.split("-").collect::<Vec<&str>>().try_into().unwrap();
                (key.to_string(),
                 [lower_left.parse::<u32>().unwrap(),
                  upper_left.parse::<u32>().unwrap(),
                  lower_right.parse::<u32>().unwrap(),
                  upper_right.parse::<u32>().unwrap()]
                )
            });//.collect::<Vec<(String,[u32;4])>>();
        let mut template = HashMap::<String,[RangeInclusive<u32>;2]>::new();
        for (key,[lower_left,upper_left,lower_right,upper_right]) in keys_and_vals{
            template.insert(key, [lower_left..=upper_left, lower_right..=upper_right]);
        }
        Self{template}
    }
    pub fn in_range(&self, field:&String, num:&u32)->bool{
        let ranges = match self.template.get(field){
            Some(v) => v,
            None => panic!("Field does not exist!")
        };
        ranges[0].contains(num) || ranges[1].contains(num)
    }
}
// Plan:
// Select one ticket T.
// Go through each value V.
// See which fields V fit into.
// 
// Go through each field and the values in them.
// If any field has just one value (Or one value exists in only one field!) that field/value combinations is defined.
// Remove that value from other fields (Or field from the field-pool!).
//
// This filtering applies cross-tickets. So once a field has decided on an idx it is done.
pub fn get_new_template(lines_iter:&mut Lines<'_>) -> Template{
    let mut template_lines = vec![];
    while let Some(line) = lines_iter.next() {
        if line == ""{
            break; // Found the split between tempalte and tickets.
        }
        template_lines.push(line);
    }
    // Make the template:
    Template::new(template_lines)
}
pub fn invalid_ticket_indices(tickets:&Vec<Vec<u32>>, memory: &mut HashMap<String,Vec<usize>>, template:&Template)->Vec<[usize;2]>{
    let mut invalids = vec![];
    for (ticket_idx, ticket) in tickets.iter().enumerate(){
        for (target_idx,target_number) in ticket.iter().enumerate(){
            let mut invalid_number = true;
            for (memory_key, valid_indices) in memory.iter_mut(){
                if template.in_range(memory_key, target_number){ // TODO: This clone is probably not good. Should refactor.
                    invalid_number = false;
                }
            }
            if invalid_number{
                invalids.push([ticket_idx,target_idx]);
            }
        }
    }
    invalids
}
pub fn make_tickets(lines_iter:&mut Lines<'_>)->Vec<Vec<u32>>{
    let mut tickets = Vec::<Vec<u32>>::new();
    while let Some(line) = lines_iter.next(){
        // Skip the lines that don't hold information:
        if line.starts_with("your ticket:") || line.starts_with("nearby") || line ==""{
            continue;
        }
        let nums = line.split(",")
            .map(|s| {s.parse::<u32>().unwrap()})
            .collect::<Vec<u32>>();
        tickets.push(nums);
    }
    tickets
}
pub fn initialize_memory(tickets:&Vec<Vec<u32>>, template: &Template)->HashMap<String,Vec<usize>>{
    let mut memory = HashMap::<String,Vec<usize>>::new();
    let mem_vec = (0..tickets[0].len()).collect::<Vec<usize>>();
    for key in template.template.keys(){
        memory.insert(key.clone(), mem_vec.clone()); // Cloning is fine, as we're not doing this very many times. < 20 for puzzle input.
    }
    memory
}
pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut lines_iter = data_string.lines().into_iter();
    let template = get_new_template(&mut lines_iter);
    // Add all tickets. The first one is ours.
    let tickets = make_tickets(&mut lines_iter);

    let mut memory = initialize_memory(&tickets, &template);
    // Now we have:
    // - A template which we can use to check if any given number fits in a field.
    // - A memory to remember which indices might belong to what field.
    // - A tickets-vec that lets us check in with what numbers each ticket has (and in what order).

    // Now, we go through and make all idx-field-combinations valid:

    // Then, go through and fiter out invalid indices.
    let invalids = invalid_ticket_indices(&tickets,  &mut memory, &template);
    let sum_invalids = invalids.iter().fold(0,|acc,v| acc+ tickets[v[0]][v[1]]);
    Some(sum_invalids)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
