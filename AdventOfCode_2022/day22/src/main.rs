mod map_components{
    #[derive(Clone,Copy,PartialEq,Debug)]
    pub enum Tile{
        Wall,
        Floor,
    }
    #[derive(Debug,Clone)]
    pub enum Rot{
        Left,
        Right
    }
    #[derive(Debug,Clone)]
    pub enum Orders{
        Rotate(Rot),
        Straight(i64)
    }
    #[derive(Debug,Clone, PartialEq)]
    pub enum Facing{
        North,
        South,
        East,
        West
    }
    pub struct MapRow{
        pub void_adjustment:i64,
        pub height:i64,
        pub max_idx:i64,
        pub row:Vec<Tile>
        
    }
    impl MapRow {
        pub fn new(line:& str, height:i64) -> Self{
            let mut void_count = 0;
            let row = line
                .chars()
                .map(|c| {
                    match c{
                        '#' => Some(Tile::Wall),
                        '.' => Some(Tile::Floor),
                        _ => {
                            void_count += 1;
                            None
                        }
                    }
                }).filter_map(|c| c)
                .collect::<Vec<Tile>>();
            let max_idx = (row.len()-1) as i64;
            Self{ void_adjustment: void_count, height, max_idx, row}
        }
    }
    
}

mod p1{
    use std::collections::btree_set::Difference;
    use std::fs::{read_to_string, DirEntry};
    use std::collections::HashMap;
    use crate::map_components::{Tile,Rot,Orders,Facing,MapRow};
    fn process_rules(rules:&str) -> Vec<Orders>{
        let mut output = Vec::<Orders>::new();
        let mut num = 0i64;
        rules.chars().for_each(|c|{
            if c.is_numeric(){  
                num*=10;
                num += c.to_digit(10).expect("Pre-checked that it is a number :)") as i64;
            }else{
                output.push(Orders::Straight(num));
                num = 0;
                match c{
                    'R' => output.push(Orders::Rotate(Rot::Right)),
                    'L' => output.push(Orders::Rotate(Rot::Left)),
                    _ => panic!("Unexpected direction: {c}")

                }
            }
        });
        output.push(Orders::Straight(num)); // We dont mind an extra "0" in the orders :)
        output

    }
    fn process_data_string(s:&str) -> (Vec<MapRow>, Vec<Orders>){
        let mut rules_string="";
        let mut height = -1;
        let map_rows = s.lines()
            .map(|line| {
                match line.contains('.') || line.contains('#'){
                    true => {
                        height += 1;
                        Some(MapRow::new(line, height))
                    },
                    false => {
                        match line.contains('R') || line.contains('L'){
                            true => {rules_string = line;},
                            false => ()
                        }
                    None
                    }
                }
            }).filter_map(|s| s) // Filters out the "None"s'
            .collect::<Vec<MapRow>>();
        let rules = process_rules(rules_string);
        return (map_rows, rules);
    }
    fn get_total_score(pos:[i64;2], dir:Facing) -> i64{
        let mut score = 0;
        score += 1000*(pos[0]+1);
        score += 4*(pos[1]+1);
        score += match dir{
            Facing::North => 3,
            Facing::South => 1,
            Facing::East => 0,
            Facing::West => 2,
        };
        return score;
    }
    fn print_map(pos_history:&HashMap<[i64;2],Facing>, map:&Vec<MapRow>){
        // let pos_vec = pos_history.iter().map(|(pos,_)| pos).collect::<Vec<&[i64;2]>>();
        for (y,line) in map.iter().enumerate() {
            for _ in 0..line.void_adjustment{
                print!(" ");
            }
            for (x,element) in line.row.iter().enumerate(){
                if let Some(dir) = pos_history.get(&[y as i64,x as i64]){
                    match dir{
                        Facing::North => print!("^"),
                        Facing::South => print!("v"),
                        Facing::East => print!(">"),
                        Facing::West => print!("<"),
                    }
                    continue;
                }
                match element{
                    Tile::Wall => print!("#"),
                    Tile::Floor => print!("."),
                }
            }
            println!();
        }
    }
    fn rotate(dir:&Facing, order:&Rot) -> Facing{
        match order{
            Rot::Right=> match dir{
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North
            }
            Rot::Left => match dir{
                Facing::North => Facing::West,
                Facing::East => Facing::North,
                Facing::South => Facing::East,
                Facing::West => Facing::South
            }
        }
    }
    fn step_horizontal(pos:[i64;2],dir:&Facing, data:&Vec<MapRow>) -> Option<[i64;2]>{
        let step = match dir{
            Facing::East => 1i64,
            Facing::West => -1i64,
            _ => panic!("--- Tried to move verticaly, but function requires horizontal movement! ---")            
        };
        let row = &data[pos[0] as usize];
        let new_pos: [i64;2] = {
            let mut new_x = pos[1]+step;
            while new_x < 0{
                new_x += row.max_idx+1;
            }
            while new_x > row.max_idx{
                new_x -= row.max_idx+1;
            }
            [pos[0],new_x]
        };
        if row.row[new_pos[1] as usize] == Tile::Wall{
            return None;
        }
        return Some(new_pos);
    }
    fn step_vertical(pos:[i64;2],dir:&Facing, data:&Vec<MapRow>) -> Option<[i64;2]>{
        let step = match dir{
            Facing::South => 1i64,
            Facing::North => -1i64,
            _ => panic!("--- Tried to move horizontally, but function requires vertical movement! ---")            
        };
        let global_pos = [pos[0], pos[1] + data[pos[0] as usize].void_adjustment];
        let new_global_pos: [i64; 2] = {
            let [mut new_x, new_y] = [global_pos[0] +step, global_pos[1]];
            if new_x >= data.len() as i64{
                new_x -= data.len() as i64 ;
            }
            if new_x < 0i64{
                new_x += data.len() as i64 ;
            }
            let row = &data[new_x as usize];
            while new_y < data[new_x as usize].void_adjustment || new_y >  row.max_idx + row.void_adjustment{
                new_x -= step;
                if new_x < 0i64{
                    new_x += data.len() as i64 -1;
                }
                if new_x >= data.len() as i64{
                    new_x -= data.len() as i64 -1;
                }
            }
            [new_x,new_y]
        };
        let row = &data[new_global_pos[0] as usize];
        let new_pos = [new_global_pos[0], new_global_pos[1] - row.void_adjustment ];
        if row.row[new_pos[1] as usize] == Tile::Wall{
            return None;
        }
        return Some(new_pos);
    }
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let (data, instructions) = process_data_string(&data_string);
        let mut pos_hist = HashMap::<[i64;2],Facing>::new();
        let (mut pos, mut dir) = ([0i64,0i64],Facing::East);
        pos_hist.insert(pos.clone(), dir.clone());
        println!("{instructions:?}\n");
        for instruction in instructions.iter(){
            match instruction{
                Orders::Rotate(rot) => {
                    dir = rotate(&dir,rot);
                    pos_hist.insert(pos.clone(), dir.clone());
                },
                Orders::Straight(dist) => {
                    match dir{
                        Facing::North | Facing::South => {
                            for _ in 0..*dist{
                                if let Some(new_pos) = step_vertical(pos, &dir, &data){
                                    pos = new_pos;
                                    pos_hist.insert(pos.clone(), dir.clone());
                                }else{
                                    break;
                                }
                            }
                        },
                        Facing::East  | Facing::West  => {
                            for _ in 0..*dist{
                                if let Some(new_pos) = step_horizontal(pos, &dir, &data){
                                    pos = new_pos;
                                    pos_hist.insert(pos.clone(), dir.clone());
                                }else{
                                    break;
                                }
                            }
                        },
                    }
                },
            }
            print_map(&pos_hist, &data);
            println!("\n");
        }
        print_map(&pos_hist, &data);
        println!("{instructions:?}\n");
        if file_name == r"src\dummy_input.txt"{
            assert_eq!(pos[0],6i64);
            assert_eq!(pos[1],8i64);
            assert_eq!(dir,Facing::East);
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

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}