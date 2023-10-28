mod p1{
    use std::fs::read_to_string;
    fn parse_data(data_string:&str) -> Vec<i32>{
        data_string.lines().map(|line| {
            let num = line.trim().parse::<i32>().expect("We know the input; Should be safe.");
            num
        }).collect::<Vec<i32>>()
    }
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let mut dyn_data = parse_data(&data_string).iter().enumerate().map(|(idx,val)| (idx,*val)).collect::<Vec<(usize,i32)>>();
        for act_idx in 0..dyn_data.len(){
            // Find where the value we're looking for is:
            let idx = dyn_data.iter().position(|(i,_)| i == &act_idx).expect("Infallable");
            let val = dyn_data[idx].1;

            // Find where we're putting the value
            let target_idx = {
                let mut ti =  val + idx as i32 ;
                if val <0{ // Adjust for direction of insertion if going backwards.
                    ti -= 1;
                }
                while ti < 0{ // Make sure it is not too small
                    ti += dyn_data.len() as i32;
                }
                while ti >= dyn_data.len() as i32{ // Make sure it is not too big
                    ti -= dyn_data.len() as i32 -1;
                    
                }
                // We now know the index is withing the correct range:
                ti as usize
            };

            // Extract the value we're moving:
            let old_value = dyn_data.remove(idx);
            
            // Insert it to where its suppsoed to go:
            dyn_data.insert(target_idx, old_value);
            
            
            if file_name == r"src\dummy_input.txt"{ // Validation that I haven't fucked anything up yet!
                let adj_data = dyn_data.iter().map(|(_,v)| v.clone()).collect::<Vec<i32>>();
                let adj_idx = act_idx +1;
                match adj_idx{
                    1 => assert_eq!(&adj_data, &vec![ 2, 1,-3, 3,-2, 0, 4],"steps:{adj_idx}, val: {val}"),
                    2 => assert_eq!(&adj_data, &vec![ 1,-3, 2, 3,-2, 0, 4],"steps:{adj_idx}, val: {val}"),
                    3 => assert_eq!(&adj_data, &vec![ 1, 2, 3,-2,-3, 0, 4],"steps:{adj_idx}, val: {val}"),
                    4 => assert_eq!(&adj_data, &vec![ 1, 2,-2,-3, 0, 3, 4],"steps:{adj_idx}, val: {val}"),
                    5 => assert_eq!(&adj_data, &vec![ 1, 2,-3, 0, 3, 4,-2],"steps:{adj_idx}, val: {val}"),
                    6 => assert_eq!(&adj_data, &vec![ 1, 2,-3, 0, 3, 4,-2],"steps:{adj_idx}, val: {val}"),
                    7 => assert_eq!(&adj_data, &vec![ 1, 2,-3, 4, 0, 3,-2],"steps:{adj_idx}, val: {val}"),
                    _ => ()
                }
            }
        }
        // Find the zero:
        let zero_idx = dyn_data.iter().position(|(i,v)| v==&0).expect("We know 0 is there") as i32;
        
        // We want the 1k, 2k, and 3k index after the first zero.
        // Adjust it so that it cycles back around, and will fit in the data vec.
        let mut indices = [1000i32,2000i32,3000i32];
        indices.iter_mut().for_each(|val|{
            *val += zero_idx;
            while *val >= dyn_data.len() as i32{
                *val -= dyn_data.len() as i32;
            }
        });

        let output: Vec<i32> = indices.iter().map(|i| dyn_data[*i as usize].1).collect::<Vec<i32>>();
        println!("\nOutput: {output:?}");           // Just looking :)
        let output_sum = output.iter().fold(0, |acc,v| acc+v);
        if file_name == r"src\dummy_input.txt"{     // Validation that I haven't fucked anything up yet!
            assert_eq!(output,vec![4, -3, 2], "||--- Dummy output is wrong! ---||");
            assert_eq!(output_sum,3, "||--- Dummy output is wrong! ---||");
        }
        Some(output_sum)
    }

#[cfg(test)]
mod tests{
    use std::collections::HashSet;

    use super::*;
	
	#[test]
	fn check_uniqueness(){
	}
}

}
mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i32>{
      None
    }
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}

use p1::main_1;
use p2::main_2;
use std::time::Instant;
fn main() {
    let file_name = r"src\dummy_input.txt";
    // let file_name= r"src\JC_input.txt";
    // let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
    if file_name ==r"src\puzzle_input.txt"{
        let ans = count.unwrap();
        println!("{} > {} = {}", ans,16533, ans > 16533);
        assert!((ans < 16533), " || --- Too Low! --- || {ans} < {:?}",16533i32);
        assert!((ans > 16533), " || --- Too High! --- || {ans} > {:?}",16533i32);
    }


    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}