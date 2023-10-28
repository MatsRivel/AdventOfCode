mod p1{
    use std::fs::read_to_string;

    pub enum Direction{
        Horizontal(i32),
        Vertical(i32),
    }
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        // Goes throug the lines, and makes an enum based on the direction.
        // Merges the values for the horizontal and vertical direction into a [i32;2] array,
        // Then takes the product of that array.
        let product = data_string
            .lines()
            .filter_map(|line|{
                if let Some(new_line) = line.strip_prefix("forward "){
                    Some(Direction::Horizontal(new_line.parse::<i32>().unwrap()))
                }else if let Some(new_line) = line.strip_prefix("down "){
                    Some(Direction::Vertical(new_line.parse::<i32>().unwrap()))
                }else if let Some(new_line) = line.strip_prefix("up "){
                    Some(Direction::Vertical(-1*new_line.parse::<i32>().unwrap()))
                }else{
                    None
                }
            }).fold([0,0], |acc, act|{
                match act{
                    Direction::Horizontal(v) => [acc[0]+v,acc[1]],
                    Direction::Vertical(v) => [acc[0],acc[1]+v]
                }
            }).iter()
            .fold(1,|acc,v| {acc*v});
        Some(product)
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
    use crate::p1::Direction;
    pub fn main_2(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        // Goes throug the lines, and makes an enum based on the direction.
        // Merges the values for the horizontal and vertical direction into a [i32;2] array,
        // Then takes the product of that array.
        let [forward_pos, depth,_aim]:[i32;3] = data_string
            .lines()
            .filter_map(|line|{
                if let Some(new_line) = line.strip_prefix("forward "){
                    Some(Direction::Horizontal(new_line.parse::<i32>().unwrap()))
                }else if let Some(new_line) = line.strip_prefix("down "){
                    Some(Direction::Vertical(new_line.parse::<i32>().unwrap()))
                }else if let Some(new_line) = line.strip_prefix("up "){
                    Some(Direction::Vertical(-1*new_line.parse::<i32>().unwrap()))
                }else{
                    None // Just in case there is an empty line at the very end :)
                }
            }).fold([0i32;3], |acc, act|{
                let [forward_pos, depth, aim] = acc;
                match act{
                    Direction::Horizontal(v)    => [forward_pos+v, depth + (aim * v), aim],
                    Direction::Vertical(v)      => [forward_pos, depth, aim + v]
                }
            });
        let product = forward_pos * depth;
        Some(product)
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
    println!("\nPart 1: {count:?}\nRuntime: {end:?}\n__________________________");
    if file_name == r"src\dummy_input.txt" && count.is_some(){
        let actual_value = count.unwrap();
        let expected_value = 150;
        assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };
    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt" && count.is_some(){
        let actual_value = count.unwrap();
        let expected_value = 900;
        assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };
}

