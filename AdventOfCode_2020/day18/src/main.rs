mod p1;
mod p2;
use crate::p1::main_1;
use crate::p2::main_2;
use std::time::Instant;
fn main() {

  // Part1 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_1(file_name);
  let end = start.elapsed();
  println!("\nPart 1 Puzzle: {count:?}\nRuntime: {end:?}");

  // Part2 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Puzzle: {count:?}\nRuntime: {end:?}");
  assert!(count.unwrap() < 62582114816962,">>> Too high! <<<")

}
#[cfg(test)]
mod tests{
    use super::*;

	#[test]
	fn part1_dummy1(){
        let file_name = r"src\dummy1.txt";
        let expected_value = 26;
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy1: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }
	#[test]
	fn part1_dummy(){
        let file_name = r"src\dummy.txt";
        let expected_value = 71;
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part1_dummy2(){
        let file_name = r"src\dummy2.txt";
        let expected_value = 437;
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy2: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part1_dummy3(){
        let file_name = r"src\dummy3.txt";
        let expected_value = 12240;
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy3: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part1_dummy4(){
        let file_name = r"src\dummy4.txt";
        let expected_value = 13632;
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy4: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }
    
	#[test]
	fn part2_dummy1(){
        let file_name = r"src\dummy1.txt";
        let expected_value = 46;
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy1: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }
	#[test]
	fn part2_dummy(){
        let file_name = r"src\dummy.txt";
        let expected_value = 231;
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part2_dummy2(){
        let file_name = r"src\dummy2.txt";
        let expected_value = 1445;
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy2: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part2_dummy3(){
        let file_name = r"src\dummy3.txt";
        let expected_value = 669060;
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy3: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }

	#[test]
	fn part2_dummy4(){
        let file_name = r"src\dummy4.txt";
        let expected_value = 23340;
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy4: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
        }
        
    }
	
}
