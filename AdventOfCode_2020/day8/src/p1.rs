use std::{fs::read_to_string, collections::HashSet, thread::current};
#[derive(PartialEq,Debug,Copy,Clone)]
enum Operation{
    Acc(i32),
    Jump(i32),
    Nope(i32),
    
}
impl Operation{
    fn new(s:&str)->Self{
        let pair:[&str;2] = s.split(" ").collect::<Vec<&str>>().try_into().expect("We don't mind if this crashes.");
        let val = {
            if let Some(other_s_val) = pair[1].strip_prefix("+"){
                let val = other_s_val.parse::<i32>().expect("Infallable");
                val
            } else{
                let other_s_val = pair[1].strip_prefix("-").expect("Infallable");
                let val = other_s_val.parse::<i32>().expect("Infallable");
                -1*val
            }
        };
        match pair[0]{
            "nop" => Operation::Nope(val),
            "jmp" => Operation::Jump(val),
            "acc" => Operation::Acc(val),
            _ => panic!("Invalid pair: {pair:?}")
        }
    }
}
fn process_data_string(data_string:&str)->Vec<Operation>{
    data_string.lines().map(|line|{
        Operation::new(line)
    }).collect::<Vec<Operation>>()
}


fn depth_search(operation_stack: &Vec<Operation>, current_idx: usize, idx_history:&mut Vec<usize>, seen_idx: HashSet<usize>)->bool{
    if seen_idx.contains(&current_idx){
        return true;
    }
    let current_operation = &operation_stack[current_idx];


    todo!()
}
fn leads_to_seen_node(operation_stack: &Vec<Operation>, current_idx: usize, seen_idx: HashSet<usize>){

}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut operation_stack = process_data_string(data_string.as_str());
    let mut idx_seen = HashSet::<usize>::new();
    let mut idx = 0;
    let mut acc = 0;
    #[cfg(test)]
    let mut iter_counter = 0;
    loop{
        #[cfg(test)]
        let idx_has_been_seen = match idx_seen.get(&idx){
            Some(_) => "y",
            None => "n"
        };
        #[cfg(test)]
        println!("{idx} ({}):\t{:?}  \t>> {acc} <<",idx_has_been_seen,operation_stack[idx]);
        if let Some(_) = idx_seen.get(&idx){
            return Some(acc);
        }
        idx_seen.insert(idx);
        let operation = operation_stack[idx];
        match operation{
            Operation::Acc(v) => acc+=v,
            Operation::Nope(_) => (),
            Operation::Jump(v) => {
                idx = (idx as i32 + v) as usize; // Not ideal way, but works for this small project. Can deal with overflow if it occurs.
                continue;
            },
        }
        idx += 1;
        continue;
    }
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
