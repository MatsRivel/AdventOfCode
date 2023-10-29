mod p1{
    use std::fs::read_to_string;
    use tools;
    pub fn main_1(file_name:&str)->Option<i32>{
      let data_string = read_to_string(file_name).unwrap();
      let mut vec = data_string.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      vec.sort();
      let result = tools::two_members_sum_to_n(&mut vec,2020);
      if let Some([a,b]) = result{
        return Some(a*b);
      }
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
mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i32>{
      let data_string = read_to_string(file_name).unwrap();
      let mut vec = data_string.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      vec.sort();
      let result = tools::three_members_sum_to_n(&mut vec,2020);
      if let Some([a,b,c]) = result{
        return Some(a*b*c);
      }
      None
    }
    }
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}


use p1::main_1;
use p2::main_2;
use std::time::Instant;
fn main() {
  // Part1 Dummy:
    let file_name = r"src\dummy.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = 514579;
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
  };
  // Part1 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_1(file_name);
  let end = start.elapsed();
  println!("\nPart 1 Puzzle: {count:?}\nRuntime: {end:?}");

  // Part2 Dummy:
  let file_name = r"src\dummy.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = 241861950;
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    };

  // Part2 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Puzzle: {count:?}\nRuntime: {end:?}");

}

