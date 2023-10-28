
mod Part1 {
    use std::fs;
    pub fn char_to_usize(c: char) -> usize{
        let val = c as usize;
        match val{
            97usize..=122usize => {
                //println!("{c}: {}", val-97+1);
                val - 97
            },
            65usize..=96usize => {
                //println!("{c}: {}", val-39+1);
                val - 39
            },
            _ => panic!("Value outside of range! {}: {}",c,val)
        }

    }
    fn process_line(line:&str) -> usize{
        let mut left_char_arr = [0i32;52];
        let mut right_char_arr = [0i32;52];
        let half_line_length = line.len()/2;
        let left = &line[..half_line_length];
        let right = &line[half_line_length..];
        let mut output:Option<usize> = None;
        left.chars()
            .zip(right.chars())
            .for_each(|(left_char, right_char)| {
                let left_char_idx = char_to_usize(left_char);
                let right_char_idx = char_to_usize(right_char);

                if left_char_arr[right_char_idx] >= 1{
                    output = Some(right_char_idx);
                    //println!("Right char {right_char} occurs in both arrays! {right_char_idx}");
                    //println!("L: {:?}\nR: {:?}\n", left_char_arr,right_char_arr);
                    return;
                }else{
                    right_char_arr[right_char_idx] += 1;
                }

                if right_char_arr[left_char_idx] >= 1{
                    output = Some(left_char_idx);
                    //println!("Left char {left_char} occurs in both arrays! {left_char_idx}");
                    //println!("L: {:?}\nR: {:?}\n", left_char_arr,right_char_arr);
                    return;
                }else{
                    left_char_arr[left_char_idx] += 1;
                }
                
        });
        match output{
            None => 0,
            Some(v) => v+1
        }
        
    }
    pub fn get_score_sum(filename:&str, line_processor: fn(line:&str)->usize ) -> Result<usize,std::io::Error>{
        let score_sum =  fs::read_to_string(filename)?
            .lines()
            .map(|line| line_processor(line))
            .sum::<usize>();
        Ok(score_sum)
    }
    pub fn main_1(filename:&str){
        println!("Part1 output: {}",get_score_sum(filename, process_line).expect("Failed to read file!\n"))
    }
}
mod Part2 {
    use std::fs; 
    use crate::Part1::char_to_usize;
    fn process_line(lines:&[&str]) -> usize {
        let mut arrs = [[0i32;52],[0i32;52],[0i32;52]];
        let mut output:Option<usize> = None;
        lines.iter()
            .enumerate()
            .for_each(|(line_idx, line)|{
                line.chars()
                    .for_each(|c|{
                        let c_idx = char_to_usize(c);
                        arrs[line_idx][c_idx] += 1;
                })
        }); 
        for i in 0..52usize{
            if arrs[0][i] >= 1 && arrs[1][i] >= 1 && arrs[2][i] >= 1{
                output = Some(i);
                break;
            }
        }
        match output{
            None => {
                // println!("{:?}",lines[0]);
                // println!("{:?}",lines[1]);
                // println!("{:?}",lines[2]);
                panic!("No same-between-3 char found!")
            },
            Some(v) => {
                // match v{
                //     0..=26 => println!("The shared char is {} ({})", v+1, (v+97) as u8 as char),
                //     _ => println!("The shared char is {} ({})", v+1, (v+39) as u8 as char),
                        
                // }
                v+1
            }
        }
    }

    pub fn get_score_sum(filename:&str, line_processor: fn(line:&[&str])->usize ) -> Result<usize,std::io::Error>{
        let score_sum =  fs::read_to_string(filename)?
            .lines()
            .map(|line| line)
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|lines| line_processor(lines))
            .sum::<usize>();
        Ok(score_sum)
    }

    pub fn main_2(filename:&str){
        println!("Part2 output: {}",get_score_sum(filename, process_line).expect("Failed to read file!\n"))
    }
}

use Part1::main_1;
use Part2::main_2;
fn main() {
    //let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
    main_1(file_name);
    main_2(file_name);
}
