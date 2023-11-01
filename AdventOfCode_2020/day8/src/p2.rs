<<<<<<< HEAD
use core::panic;
use std::{fs::read_to_string, collections::HashSet};
#[derive(PartialEq,Debug,Copy,Clone)]
enum Operation{
    Acc(i32),
    Jump(i32),
    Nope(i32),
    Terminate,
    
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
fn fresh_depth_search(operation_stack: Vec<Operation>)->Option<Vec<Operation>>{
    depth_search(operation_stack, 0, &mut HashSet::<usize>::new(), &mut Vec::<usize>::new(), false)
}
fn depth_search(operation_stack: Vec<Operation>, current_idx: usize,seen_idx: &mut HashSet<usize>, idx_history: &mut Vec<usize>, is_altered:bool) -> Option<Vec<Operation>>{
    match operation_stack[current_idx]{
        Operation::Terminate => return Some(operation_stack),
        _ => ()
    }
    if seen_idx.contains(&current_idx) {
        if is_altered{ // No further alterations possible.
            #[cfg(test)]
            println!("Dead-end at idx: {current_idx} and stack: {operation_stack:?}");
            return None;
        }
        // Keep going backwards in "time" until we find a spot we can attempt to fix things at
        let mut last_idx = current_idx;
        seen_idx.remove(&current_idx);
        loop{
            match operation_stack[last_idx]{
                Operation::Terminate | Operation::Acc(_) | Operation::Nope(0) => {
                    last_idx = idx_history.pop().unwrap();
                    seen_idx.remove(&last_idx);
                    
                },
                Operation::Nope(_) | Operation::Jump(_) => break // Found a possible spot to repair
            }
        }
        let mut new_operation_stack = operation_stack.clone();
        new_operation_stack[last_idx] = match new_operation_stack[last_idx]{
            Operation::Nope(v) => Operation::Jump(v), 
            Operation::Jump(v) => Operation::Nope(v),
            _ => panic!("We've already filtered these out above!")
        };

        #[cfg(test)]
        {
            println!("\nWe've already seen this index ({current_idx}) with this stack.");
            println!("Trying new stack at idx: {last_idx} and stack: {new_operation_stack:?}");

        }
        // Make a new DFS from the current position:
        if let Some(working_stack) = depth_search(new_operation_stack, current_idx, seen_idx, idx_history, true){
            return Some(working_stack);
        }
        else{ // Take another step back using the original stack:
            let mut last_idx = idx_history.pop().unwrap(); 
            seen_idx.remove(&last_idx);
            loop{
                match operation_stack[last_idx]{
                    Operation::Terminate | Operation::Acc(_) | Operation::Nope(0) => last_idx = idx_history.pop().unwrap(),
                    Operation::Nope(_) | Operation::Jump(_) => break // Found a possible spot to repair
                }
            }
            return depth_search(operation_stack, last_idx, seen_idx, idx_history, is_altered)

        }
    }
    // If the current idx has not been seen: Log it
    seen_idx.insert(current_idx);
    idx_history.push(current_idx);
    // Then keep following the path until we either find the "exit" or hit a dead end.
    let next_idx = match operation_stack[current_idx]{
        Operation::Jump(v) => (current_idx as i64 + v as i64) as usize,
        Operation::Nope(_) | Operation::Acc(_) => current_idx+1,
        Operation::Terminate => return Some(operation_stack), // Found the end! Return the current stack for processing.
    };
    #[cfg(test)]
    println!("Continuing with {current_idx} and the same stack: {operation_stack:?}");
    depth_search(operation_stack, next_idx, seen_idx, idx_history, is_altered)


}

pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut operation_stack = process_data_string(data_string.as_str());
    operation_stack.push(Operation::Terminate); // Add a terminator at the end for easy termination in depth-first search.

    #[cfg(test)]
    println!("{operation_stack:?}");
    let operation_stack = fresh_depth_search(operation_stack)
        .expect("Problem is solvable, so we are ok with panicing if we don't find a solution");
    #[cfg(test)]
    println!("{operation_stack:?}");
    // Run through the operations, as we know they have no loop.
    let mut acc = 0;
    let mut idx = 0;
    let mut idx_seen: HashSet<usize> = HashSet::new();
    loop{
        if let Some(_) = idx_seen.get(&idx){
            panic!("Just here in case we hit an infinite loop. The current operation_stack should be loop-free.");
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
            Operation::Terminate => return Some(acc)
        }
        idx += 1;
        continue;
    }
=======
use std::fs::read_to_string;
pub fn main_2(file_name:&str)->Option<i32>{
    None

>>>>>>> f256c707b9daffae1f24c3024df3644f168ef68f
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
