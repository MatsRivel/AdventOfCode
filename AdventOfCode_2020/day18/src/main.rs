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

}
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn part1_dummy(){
        let file_names = [r"src\dummy.txt",r"src\dummy1.txt",r"src\dummy2.txt",r"src\dummy3.txt",r"src\dummy4.txt"];
        let solutions = [71,26,437,12240,13632];
        for (file_name,expected_value) in file_names.iter().zip(solutions.iter()){
            let file_name = r"src\dummy4.txt";
            let start = Instant::now();
            let count = main_1(file_name);
            let end = start.elapsed();
            println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
            if let Some(actual_value) = count{
                assert_eq!(actual_value, *expected_value, "Got {actual_value}, expected {expected_value} for {file_name}.\n__________________________");
            }
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
            let expected_value = todo!();
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
    }
	}
	
}
