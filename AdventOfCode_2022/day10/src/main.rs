pub mod part1{
    use std::fs::read_to_string;
    // use itertools::Itertools;

    // struct SignalStrengthIter{
    //     current: i32,
    //     next: i32,
    //     iter_step: i32
    // }
    // impl Iterator for SignalStrengthIter{
    //     type Item = [i32;2];
    //     fn next(&mut self) -> Option<Self::Item>{
    //         self.current = self.next;
    //         self.next += self.iter_step;
    //         Some([self.current,self.next])
    //     }
    // }
    pub fn get_cycle_values(file_name:&str) -> Vec<i32>{
        read_to_string(file_name)
            .expect("Failed to read file")
            .lines()
            .map(|line| {
                if line.starts_with("addx"){
                    let val = line.split(" ").filter(|text| !text.starts_with("addx")).map(|text| text.parse::<i32>().expect("Failed to parse into i32")).sum::<i32>();
                    // println!("addx {val}");
                    val
                }else{
                    // println!("noop");
                    0i32
                }
            }).collect::<Vec<i32>>()
    }
    pub fn main_1(file_name:&str){
		let mut cycle_values = get_cycle_values(file_name);
        let mut register_value = 1;
        let mut sum = 0;
        let mut cycle_count = 0;
        for i in 0..(cycle_values.len()){
            let mut cycle = 2;
            if cycle_values[i] == 0{
                cycle = 1;
            }
            for _ in 0..cycle{
                cycle_count += 1;
                if cycle_count == 20 || (cycle_count - 20) % 40 == 0{
                    sum += register_value*cycle_count;
                    println!("Cycle: {}, Signal Strenght = {}, Total: {}\n",cycle_count,register_value*cycle_count, sum);
                }else{
                    // println!("Cycle: {}, Change: {}, Register: {}", cycle_count, stuff[i], register_value);
                    ();
                }
            }
            register_value += cycle_values[i];

        }
        println!("Part 1: {sum}");

    }
}
mod part2{
    use std::fs::read_to_string;
    use super::part1::get_cycle_values;
    pub fn main_2(file_name:&str){
		let mut cycle_values = get_cycle_values(file_name);
        let mut screen = [false;240];
        let mut register = 1i32;
        let mut cycle_count = 0;
        for i in 0..(cycle_values.len()){
            let mut cycle = 2;
            if cycle_values[i] == 0{
                cycle = 1;
            }
            for _ in 0..cycle{
                // println!("{} | {}",cycle_count%40, register);
                let sprite_pos = [register-1, register, register+1];
                'inner: for pos in sprite_pos.iter() {
                    if &(cycle_count%40) == pos{
                        screen[cycle_count as usize] = true;
                        break 'inner
                    }
                }
                // println!("{}: {}", cycle_count, register);
                cycle_count += 1;
            }
            register += cycle_values[i];
        }
        println!();
        // Print screen.
        for j in 0..240{
            match screen[j]{
                true => print!("#"),
                false => print!(".")
            }
            if (j+1)%40 == 0{
                println!()
            }
        }
        println!("Part2 Answer: {}","ECZUZALR")

    }
}

use part1::main_1;
use part2::main_2;
fn main() {
    // let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
    // main_1(file_name);
    main_2(file_name);
}
