mod p1{
    use std::fs::read_to_string;
    pub fn process_datastring(data_string:&str)-> Vec<[[i32;2];2]>{
        data_string.lines().map(|line| {
            line.split(" -> ").map(|s|{
                let pos: [i32;2] = s.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
                pos
            }).collect::<Vec<[i32;2]>>().try_into().unwrap()
        }).collect::<Vec<[[i32;2];2]>>()
    }
    trait Overlapping{
        fn overlapping(&self,other:&Self)->bool;
    }
    impl Overlapping for [[i32;2];2]{
        fn overlapping(&self,other:&[[i32;2];2]) -> bool{
            // Find shared axis:
            if self[0][0] == other[0][0] && self[1][0] == other[1][0]{ // Exist on same x-idx in pos = [x,y]
                todo!()
            }else if self[0][1] == other[0][1] && self[1][1] == other[1][1]{// Exist on same y-idx in pos = [x,y]
                todo!()
            } 
            else{ // Do not share a row or column.
                false
            }
        }
    }
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let mut vents = process_datastring(&data_string);
        vents.sort_by(|x,y| x[0][0].cmp(&y[0][0]));
        for i in 1..vents.len(){
            
        }
        todo!()
    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}
mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i32>{
        
        todo!();
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
    // let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy_input.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = todo!();
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
    };
	
    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy_input.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = todo!();
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };
}

