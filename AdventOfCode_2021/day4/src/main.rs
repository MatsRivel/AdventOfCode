mod p1{
    use std::fs::read_to_string;
    const BOARD_SIDES:usize=5;
    const BOARD_SIZE:usize = BOARD_SIDES*BOARD_SIDES;
    struct Board{
        numbers: [Option<u32>;BOARD_SIZE],
    }
    impl Board{
        pub fn new(s:&[&str]) -> Self{
            let numbers = s.iter().map(|line|{
                line.split(' ').map(|num| num.parse::<u32>().unwrap())
            }).enumerate()
            .fold([None;BOARD_SIZE], |mut acc, (outer_idx, itr)| {
                itr.enumerate().for_each(|(inner_idx, n)| acc[inner_idx+BOARD_SIDES*outer_idx] = Some(n));
                acc
            });
            Self{numbers}
        }
        fn get_idx_from_2d(&self,x:usize,y:usize) -> usize{
            x*BOARD_SIDES+y
        }
        pub fn get_from_2d(&self,x:usize,y:usize) -> Option<u32>{
            let idx = self.get_idx_from_2d(x,y);
            self.numbers[idx]
        }
        pub fn get_from_idx(&self,idx:usize) -> Option<u32>{
            self.numbers[idx]
        }
        pub fn set_from_2d(&mut self,x:usize,y:usize, val:Option<u32>){
            let idx = self.get_idx_from_2d(x,y);
            self.numbers[idx] = val;
        }
        pub fn set_from_idx(&mut self,idx:usize, val:Option<u32>){
            self.numbers[idx] = val;
        }
        pub fn has_num(&self,val:u32) -> Option<usize>{
            if let Some(pos) = self.numbers.iter().position(|&x| x==Some(val)){
                Some(pos)
            }else{
                None
            }
        }
    }

    fn process_numbers(line:&str) -> Vec<u32>{
        line.split(",").map(|s| {
            s.parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }
    pub fn process_boards(lines:&str) {
        let all_lines = lines.lines().filter_map(|line|{
            if line.starts_with("\n"){
                None
            }else{
                Some(line)
            }
        }).collect::<Vec<&str>>();
        let my_boards = Vec::<Board>::with_capacity(all_lines.len()/5);
        for i in 0..(all_lines.len()/5){
            let b = Board::new(&all_lines[i..(i+5)]);
            all_lines.puhs()
        }

    }
    pub fn main_1(file_name:&str)->Option<i32>{
        let complete_data_string = read_to_string(file_name).unwrap();
        let (numbers_string, data_string):(&str, &str) =complete_data_string.split_once("\n").unwrap();
        let numbers: Vec<u32> = process_numbers(&numbers_string);
        let boards = process_boards(data_string);
        for n in numbers.iter(){
            for b in boad
        }
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

