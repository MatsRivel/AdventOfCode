mod p1{
    use std::fs::read_to_string;
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let data = &data_string.lines().map(|line|{
            line.trim().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>();
        let mut counter = 0;
        for i in 1..data.len(){
            if data[i] > data[i-1]{
                counter+=1
            }
        }
        Some(counter)
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
        let data_string = read_to_string(file_name).unwrap();
        let data = &data_string.lines().map(|line|{
            line.trim().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>();
        let mut counter = 0;
        for i in 3..data.len(){
            if data[i-3] < data[i]{
                counter+=1
            }
        }
        Some(counter)
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
    let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt" && count.is_some(){
        let actual_value = count.unwrap();
        let expected_value = 7;
        assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };
    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt" && count.is_some(){
        let actual_value = count.unwrap();
        let expected_value = 5;
        assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };
}

