mod p1{
    use std::fs::read_to_string;
    use tools;
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let n_valid_passwords = data_string.lines().map(|line|{
            let [key,pass]:[&str;2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
            let [bounds,key_str]: [&str;2]= key.split(" ").map(|s| s).collect::<Vec<&str>>().try_into().unwrap();
            let [lower_bound,upper_bound]: [u32;2] = bounds.split("-").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
            let key = key_str.chars().next().unwrap();
            let count = pass.chars().fold(0u32, |acc,c| {
                if c == key{
                    acc+1
                }else{
                    acc
                }
            });
            lower_bound <= count && count <= upper_bound
        }).fold(0, |acc,b|{
            if b{
                acc+1
            }else{
                acc
            }
        });
        Some(n_valid_passwords)
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

    struct PassCase{
        upper_bound:usize,
        lower_bound:usize,
        pass:String,
        key:char,
    }
    impl PassCase{
        fn print_case(&self){
            for (i,c) in self.pass.char_indices(){
                if i == self.lower_bound-1 {
                    if self.get_char_at_idx(i).unwrap() != self.key{
                        print!("!");
                        continue;
                    }
                } else if i == self.upper_bound-1{
                    if self.get_char_at_idx(i).unwrap() == self.key{
                        print!("!");
                        continue;
                    }
                }
                print!("{:1.0?}",i+1);
            }
            println!();
            for (i,c) in self.pass.char_indices(){
                if i == self.lower_bound-1 || i == self.upper_bound-1{
                    print!("{}",self.key);
                }else{  
                    print!(".");
                }
            }
            println!();
            for (i,c) in self.pass.char_indices(){
                print!("{c}");
            }
            print!(" <- {}, [{},{}]",self.is_valid(), self.lower_bound, self.upper_bound);
            println!();
            println!();
        }
        fn new(line:&str) -> Self{
            let [bounds_and_key,pass]:[&str;2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
            let [bounds,key_str]: [&str;2]= bounds_and_key.split(" ").map(|s| s).collect::<Vec<&str>>().try_into().unwrap();
            let [lower_bound,upper_bound]: [usize;2] = bounds.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>().try_into().unwrap();
            let key = key_str.chars().next().unwrap();
            Self { upper_bound, lower_bound, pass: pass.to_string(), key}
        }
        fn is_valid(&self)->bool{
            let left_char = self.get_char_at_idx(self.lower_bound-1);
            let right_char =  self.get_char_at_idx(self.upper_bound-1);
            left_char != right_char && (left_char == Some(self.key) || right_char == Some(self.key))
        }
        fn get_char_at_idx(&self,idx:usize)->Option<char>{
            let mut cs = self.pass.chars();
            let mut c = cs.next();
            let mut i = 0;
            while i < idx{
                c = cs.next();
                i+=1;
            }
            c
        }
    }
    pub fn main_2(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let n_valid_passwords = data_string.lines().map(|line|{
            let passcase = PassCase::new(line);
            passcase.print_case();
            passcase
        }).fold(0, |acc,passcase|{
                if passcase.is_valid(){
                    acc+1
                }else{
                    acc
                }
        });
        Some(n_valid_passwords)
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
  // Part1 Dummy:
    let file_name = r"src\dummy.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = 2;
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
  };
  // Part1 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_1(file_name);
  let end = start.elapsed();
  println!("\nPart 1 Puzzle: {count:?}\nRuntime: {end:?}\n__________________________");

  // Part2 Dummy:
  let file_name = r"src\dummy.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Dummy: {count:?}\nRuntime: {end:?}");
	if file_name == r"src\dummy.txt" && count.is_some(){
		let actual_value = count.unwrap();
		let expected_value = 1;
		assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
    };

  // Part2 Puzzle:
  let file_name= r"src\puzzle.txt";
  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2 Puzzle: {count:?}\nRuntime: {end:?}\n__________________________");
  // 336 is too low!

}