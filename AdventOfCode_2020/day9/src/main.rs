mod p1;
mod p2;
use crate::p1::main_1;
use crate::p2::main_2;
use std::time::Instant;
fn main() {

  // Part1 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let preamble = 25;
  let count = main_1(file_name, preamble);
  let end = start.elapsed();
  println!("\nPart 1 Puzzle: {count:?}\nRuntime: {end:?}");

  // Part2 Puzzle:
  let file_name= r"src\puzzle.txt";
  let preamble = 25;
  let start = Instant::now();
  let count = main_2(file_name,preamble);
  let end = start.elapsed();
  println!("\nPart 2 Puzzle: {count:?}\nRuntime: {end:?}");

}
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn part1_dummy(){
        let file_name = r"src\dummy.txt";
        let preamble = 5;
        let start = Instant::now();
        let count = main_1(file_name, preamble);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 127;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }

        }

	#[test]
	fn part2_dummy(){
        let file_name = r"src\dummy.txt";
        let start = Instant::now();
        let preamble = 5;
        let count = main_2(file_name, preamble);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 62;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    }
	}
	
}
