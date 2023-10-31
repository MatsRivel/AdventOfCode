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
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut operation_stack = process_data_string(data_string.as_str());
    let mut idx_seen = HashSet::<usize>::new();
    let mut idx = 0;
    let mut acc = 0;
    let mut function_repaired = false;
    let mut idx_history = Vec::new();
    #[cfg(test)]
    let mut iter_counter = 0;
    loop{
        #[cfg(test)]
        {
            iter_counter += 1;
            if iter_counter >= 100{
                panic!()
            }
        }

        #[cfg(test)]
        let idx_has_been_seen = match idx_seen.get(&idx){
            Some(_) => "y",
            None => "n"
        };
        #[cfg(test)]
        println!("{idx} ({}):\t{:?}  \t>> {acc} <<",idx_has_been_seen,operation_stack[idx]);
        if let Some(_) = idx_seen.get(&idx){
            if !function_repaired{ // We only get to repair one lement
                function_repaired = true;
                // Find the last "jump" and make it a "nope"
                let mut last_idx = idx_history.pop().unwrap();
                loop {
                    if let Operation::Jump(v) = operation_stack[last_idx]{
                        #[cfg(test)]
                        println!("Interviened at idx == {last_idx} !");
                        operation_stack[last_idx] = Operation::Nope(v);
                        break;
                    }else if let Operation::Nope(v) = operation_stack[last_idx]{
                        if v != 0{
                            #[cfg(test)]
                            println!("Interviened at idx =={last_idx} !");
                            operation_stack[last_idx] = Operation::Nope(v);
                            break;
                        }else{
                            last_idx = idx_history.pop().unwrap();
                        }
                    }else {
                        last_idx = idx_history.pop().unwrap();
                    }
                }

            }
        }else{
            idx_seen.insert(idx);
        }
        let operation = operation_stack[idx];
        match operation{
            Operation::Acc(v) => acc+=v,
            Operation::Nope(v) => (),
            Operation::Jump(v) => {
                idx_history.push(idx);
                idx = (idx as i32 + v) as usize; // Not ideal way, but works for this small project. Can deal with overflow if it occurs.
                continue;
            },
            Operation::Terminate => return Some(acc)
        }
        idx_history.push(idx);
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
