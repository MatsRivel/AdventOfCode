mod p1{
  use std::fs::read_to_string;
  pub fn process_data_string(data_string:&str)-> Vec<Vec<i32>>{
    let mut binaries = data_string
      .lines()
      .map(|line|{
        let inner = line.chars().filter_map(|c|{
          match c{
            '0' => Some(0),
            '1' => Some(1),
            _ => None
          }
        }).collect::<Vec<i32>>();
        inner
      }).collect::<Vec<Vec<i32>>>();
      binaries
  }
  pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let data = process_data_string(&data_string);
    let half_size = data.len() / 2;
    let common_len = data[0].len();
    let common_count = data
      .iter()
      .fold(vec![0i32;common_len],|mut acc,inner|{
        acc.iter_mut().zip(inner).for_each(|(a,i)| *a += i);
        acc
      }
    );
    let binary_rep = common_count.iter().map(|val|{
      val / half_size as i32
    });
    let [gamma_rate, epsilon_rate] = binary_rep
      .enumerate()
      .fold([0;2], |[gamma, epsilon],(i,val)|{
        if val == 1{
          [gamma + 2i32.pow((common_len-i-1) as u32), epsilon]
        }else{
          [gamma, epsilon + 2i32.pow((common_len-i-1) as u32)]
        }
      });
    Some(gamma_rate*epsilon_rate)
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
  use crate::p1::process_data_string;
  pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let data_none_rotated = process_data_string(&data_string);
    let mut data = vec![vec![0i32;data_none_rotated.len()];data_none_rotated[0].len()];
    for i in 0..data_none_rotated.len(){
      for j in 0..data_none_rotated[0].len(){
        data[j][i] = data_none_rotated[i][j];
      }
    }
    let half_size = data.len() / 2;
    let common_len = data[0].len();
    let common_count = data
      .iter()
      .fold(vec![0i32;common_len],|mut acc,inner|{
        acc.iter_mut().zip(inner).for_each(|(a,i)| *a += i);
        acc
      }
    );
    let binary_rep = common_count.iter().map(|val|{
      val / half_size as i32
    });
    let [gamma_rate, epsilon_rate] = binary_rep
      .enumerate()
      .fold([0;2], |[gamma, epsilon],(i,val)|{
        if val == 1{
          [gamma + 2i32.pow((common_len-i-1) as u32), epsilon]
        }else{
          [gamma, epsilon + 2i32.pow((common_len-i-1) as u32)]
        }
      });
    Some(gamma_rate*epsilon_rate)
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
  let expected_value = 198;
  assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
  };

  let start = Instant::now();
  let count = main_2(file_name);
  let end = start.elapsed();
  println!("\nPart 2: {count:?}\nRuntime: {end:?}");
if file_name == r"src\dummy_input.txt" && count.is_some(){
  let actual_value = count.unwrap();
  let expected_value = 230;
  assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}");
  };
}

