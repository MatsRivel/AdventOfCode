
use crate::p1::get_invalid_number;
use std::fs::read_to_string;
fn window_search(numbers_indices:&Vec<(i64,usize)>,target:i64,window_size:usize)->Option<[usize;2]>{
    // Try a smaller window size:
    if window_size == 0{
        return None;
    }
    let mut left = 0;
    let mut right = window_size-1;
    let mut sum = numbers_indices
        .iter()
        .filter_map(|(v,i)|{
            if *i < right{
                Some(v)
            }else{
                None
            }
        }).fold(0, |acc,v| acc+v);
        
    while right < numbers_indices.len(){
        sum += numbers_indices[right].0;
        #[cfg(test)]{
            numbers_indices[left..right].iter().for_each(|(_,i)|print!("{i} + "));
            println!("{} ",numbers_indices[right].1);
            numbers_indices[left..right].iter().for_each(|(v,_)|print!("{v} + "));
            print!("{} ",numbers_indices[right].0);
            println!("== {sum} | Target: {target}");
        }
        if sum ==target{
            return Some([left,right]);
        }else{
            sum -= numbers_indices[left].0;
            left += 1;
            right +=1;
            
        }
    }
    #[cfg(test)]
    { // We know that the test window size is correct, so no point in iterating further if we know its allready failed...
        return None;
    }
    window_search(numbers_indices, target, window_size-1)
}
pub fn main_2(file_name:&str, preamble:usize)->Option<i64>{
    let data_string = read_to_string(file_name).unwrap();
    let numbers = data_string.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let target_number = get_invalid_number(&numbers, preamble).expect("We know part1 works, or we wouldnt have gotten to part2...");
    #[cfg(test)]
    println!("________________________________________________________________");
    let numbers_indices = numbers.iter().enumerate().map(|(i,v)| (*v, i)).collect::<Vec<(i64,usize)>>();
    let mut window_size = numbers_indices.len();
    #[cfg(test)]
    {
        window_size = 4;
    }
    let indices = window_search(&numbers_indices, target_number, window_size).expect("We are ok with panic if this fails.");
    let contigous_slice = &numbers_indices[indices[0]..=indices[1]];
    let c_nums = contigous_slice.iter().map(|(v,i)| *v).collect::<Vec<i64>>();
    let max = c_nums.iter().fold(i64::MIN, |acc,&v|{
        if v > acc{
            v
        }else{
            acc
        }
    });
    let min = c_nums.iter().fold(i64::MAX, |acc,&v|{
        if v < acc{
            v
        }else{
            acc
        }
    });
    Some(max+min)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
