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
  let lowest_known = 6663;
  assert!(count.unwrap() > lowest_known,"Answer must be > {lowest_known}!");

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
	fn part1_dummy_large_loop(){
        let file_name = r"src\large_dummy.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 198;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }
    }
	#[test]
    #[should_panic]
	fn part1_dummy_large_loop2(){
        let file_name = r"src\large_dummy2.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 198;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }
    }
	#[test]
	fn part1_dummy_false_loop(){
        let file_name = r"src\dummy_false_loop.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 4;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }
    }
    #[test]
    #[should_panic]
	fn part1_dummy_false_loop_impossible(){
        let file_name = r"src\dummy_false_loop_impossible.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");

    }
	#[test]
	fn part1_dummy1(){
        let file_name = r"src\dummy1.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 4;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }

    }
	#[test]
	fn part1_dummy2(){
        let file_name = r"src\dummy2.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 8;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }

    }
	#[test]
	fn part1_dummy3(){
        let file_name = r"src\dummy3.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 8;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
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
            let expected_value = 4;
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
            let expected_value = 4;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
    #[test]
	fn part2_dummy6(){
        let file_name = r"src\dummy6.txt";
        let start = Instant::now();
        let count = main_2(file_name);
        let end = start.elapsed();
        println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 8;
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
            let expected_value = 10;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
        }
	}
	
}
