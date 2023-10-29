use std::{fs::read_to_string, collections::btree_map::Values};
struct Policy{
    steps_right:usize,
    steps_down:usize,
}
impl Policy{
    fn new(steps_right:usize, steps_down:usize) -> Self{
        Policy { steps_right, steps_down }
    }
}
pub fn main_2(file_name:&str)->Option<i64>{
    let data_string = read_to_string(file_name).unwrap();
    let policies = [
        Policy::new(1,1),
        Policy::new(3,1),
        Policy::new(5,1),
        Policy::new(7,1),
        Policy::new(1,2)
    ];
    let product_count = policies.iter().map(|policy|{
        let tree_counter = data_string.lines().enumerate().skip(1).filter_map(|(outer_idx,line)|{
            // Check vertical idx
            let line_length = line.len();
            let mut tree_found = None;
            for (i,c) in line.char_indices(){
                if i == ((outer_idx/policy.steps_down)*policy.steps_right % line_length) && ((outer_idx) % policy.steps_down == 0){
                    if c == '#'{
                        tree_found = Some(true);
                    }
                }
            }
            tree_found
        }).fold(0i64, |acc, has_tree|{
            if has_tree{
                acc+1i64
            }else{
                acc
            }
        });
        println!("Tree counter: {tree_counter}");
        tree_counter
    }).fold(1, |acc,val|{
        acc*val
    });
    Some(product_count)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
