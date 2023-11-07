mod p1;
mod p2;
use crate::p1::main_1;
use crate::p2::main_2;
use std::time::Instant;
fn main() {

  // Part1 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_1(file_name,2020);
  let end = start.elapsed();
  println!("\nPart 1 Puzzle: {count:?}\nRuntime: {end:?}");
  // Expected ans: 1194

  // Part2 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Puzzle: {count:?}\nRuntime: {end:?}");
  // Expected ans: 48710

}
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn part1_dummy(){
        let file_name = r"src\dummy.txt";
        let start = Instant::now();
        let count = main_1(file_name,2020);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 436;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }

        }

	#[test]
	fn part2_dummy(){
        let file_name = r"src\dummy.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 175594;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	#[test]
	fn part2_dummy3(){
        let file_name = r"src\dummy3.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 2578;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	#[test]
	fn part2_dummy4(){
        let file_name = r"src\dummy4.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 3544142;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	#[test]
	fn part2_dummy5(){
        let file_name = r"src\dummy5.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 261214;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	#[test]
	fn part2_dummy6(){
        let file_name = r"src\dummy8.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 6895259;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
    #[test]
	fn part2_dummy7(){
        let file_name = r"src\dummy7.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 18;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
    #[test]
	fn part2_dummy8(){
        let file_name = r"src\dummy6.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 362;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	
}
