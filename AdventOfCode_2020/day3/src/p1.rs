use std::fs::read_to_string;
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let tree_counter = data_string.lines().enumerate().skip(1).map(|(outer_idx,line)|{
        let line_length = line.len();
        line.char_indices().fold(false,|acc,(inner_idx,c)| {
            if inner_idx == (outer_idx*3 % line_length) && c=='#'{
                true
            }else{
                acc
            }
        })
    }).fold(0,|acc,inner| if inner{acc+1}else{acc});
    Some(tree_counter as i32)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
