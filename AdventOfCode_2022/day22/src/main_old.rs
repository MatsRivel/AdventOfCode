
mod p1{
    use core::panic;
    use std::{fs::read_to_string, collections::HashMap};

    // Note: MapCycle cant just keep track of its initial idx, but also needs to adjust for 
    // Large amounrs of empty space before/after that might be occupied by others rows, but not this one.
    struct MapCycle{
        void_adjustment:i64, // So that we can consider space not occupied in our row, but in others it might be.
        height: i64,
        current_idx:i64,
        max_idx:i64, // Both allow rollover for going through the vec, but also to adjust for void spaces to the right of our existing ones.
        cycle: Vec<Tile>,

    }

    impl MapCycle {
        fn new(line:& str, height:i64) -> Self{
            let mut void_count = 0;
            let cycle = line
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
            let max_idx = (cycle.len()-1) as i64;
            Self{ void_adjustment: void_count, height, current_idx: 0, max_idx, cycle}
        }
        fn prev(&mut self) -> Option<Tile>{
            self.current_idx -= 1;
            if self.current_idx < 0{
                self.current_idx += self.max_idx;
            }
            return Some(self.cycle[self.current_idx as usize]);

        }
        fn get_global_idx(&self) -> i64{
            self.void_adjustment + self.current_idx
        }
        fn get_value_by_global_idx(&self, global_idx: i64) -> Tile{
            let mut idx = global_idx - self.void_adjustment;
            if idx > self.max_idx{
                idx -= self.max_idx;
            }else if idx < 0{
                idx += self.max_idx;
            }
            self.cycle[idx as usize]
        }
        fn get_value_by_local_idx(&self, local_idx: i64) -> Tile{
            let mut idx =local_idx;
            if idx > self.max_idx{
                idx -= self.max_idx;
            }else if idx < 0{
                idx += self.max_idx;
            }
            self.cycle[idx as usize]
        }
    }

    impl Iterator for MapCycle{
        type Item=Tile;
        fn next(&mut self) -> Option<Self::Item>{
            self.current_idx += 1;
            if self.current_idx > self.max_idx{
                self.current_idx -= self.max_idx;
            }
            return Some(self.cycle[self.current_idx as usize]);
        }
    }

    #[derive(Clone,Copy,PartialEq,Debug)]
    enum Tile{
        Wall,
        Floor,
    }
    #[derive(Debug,Clone)]
    enum Rot{
        Left,
        Right
    }
    #[derive(Debug,Clone)]
    enum Orders{
        Rotate(Rot),
        Straight(i64)
    }
    #[derive(Debug,Clone)]
    enum Facing{
        North,
        South,
        East,
        West
    }
    fn rotate(dir:Facing, order:&Rot) -> Facing{
        match order{
            Rot::Left=> match dir{
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North
            }
            Rot::Right => match dir{
                Facing::North => Facing::West,
                Facing::East => Facing::North,
                Facing::South => Facing::East,
                Facing::West => Facing::South
            }
        }
    }
    fn move_horizontal(pos:[i64;2], dist:i64, dir:&Facing, map:&mut Vec<MapCycle>) -> [i64;2]{
        let mut new_pos = pos.clone();
        let step_direction = match dist{
            0.. => 1,
            _ => -1
        };
        for _ in 0..(dist.abs()){
            if map[new_pos[0] as usize].get_value_by_local_idx(new_pos[1]+step_direction) == Tile::Wall{
                return new_pos;
            }
            let local_map = &map[new_pos[0] as usize];
            let mut idx =new_pos[1]+step_direction;
            if idx > local_map.max_idx{
                idx -= local_map.max_idx;
            }else if idx < 0{
                idx += local_map.max_idx;
            }
            new_pos[1] = idx;
        }
        return new_pos;
    }
    fn vertical_rollover(global_pos_in:[i64;2], step_direction:i64, map:&mut Vec<MapCycle>)->[i64;2]{
        let global_horizontal = global_pos_in[1] + map[global_pos_in[0] as usize].void_adjustment;
        let mut safe_global_pos = [global_pos_in[0], global_horizontal];
        loop{
            let new_pos = [safe_global_pos[0] - step_direction, global_horizontal];
            let max_idx = map[new_pos[0] as usize].max_idx;
            let void_adjustment = map[new_pos[0] as usize].void_adjustment;
            let new_local_horizontal = global_horizontal - void_adjustment;
            if new_local_horizontal < 0 || new_local_horizontal > max_idx{
                break;
            }
            safe_global_pos = new_pos;
        }
        return safe_global_pos;

    }
    fn move_vertical(pos:[i64;2], dist:i64, dir:&Facing, map:&mut Vec<MapCycle>) -> [i64;2]{
        let mut new_pos = pos.clone();
        let step_direction = match dist{
            0.. => 1,
            _ => -1
        };
        for _ in 0..(dist.abs()){
            let mut height = new_pos[0] + step_direction;
            if height < 0{
                height += map.len() as i64 -1;
            }

            let global_idx = new_pos[1] + map[new_pos[0] as usize].void_adjustment;
            let mut new_global_pos = [height, global_idx]; 
            let new_max_idx = map[height as usize].max_idx;
            let local_idx = global_idx - map[height as usize].void_adjustment;
            if local_idx < 0 || local_idx > new_max_idx{
                new_global_pos = vertical_rollover(new_global_pos, step_direction, map);
            };
            let new_local_pos = [new_global_pos[0], new_global_pos[1] - map[new_global_pos[0] as usize].void_adjustment];
            if map[new_global_pos[0] as usize].get_value_by_local_idx(new_local_pos[1]) == Tile::Wall {
                break;
            }
            new_pos = new_local_pos;
        }
        return new_pos;
    }
    fn advance(pos:[i64;2], dist:i64, dir:&Facing, map:&mut Vec<MapCycle>) -> [i64;2]{
        let new_pos = match dir{
            Facing::North => move_vertical(pos, -dist, dir, map),
            Facing::South => move_vertical(pos, dist, dir, map),
            Facing::East => move_horizontal(pos, dist, dir, map),
            Facing::West => move_horizontal(pos, -dist, dir, map),
        };
        new_pos
    }
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
    fn process_data_string(s:&str) -> (Vec<MapCycle>, Vec<Orders>){
        let mut rules_string="";
        let mut height = -1;
        let map_rows = s.lines()
            .map(|line| {
                match line.contains('.') || line.contains('#'){
                    true => {
                        height += 1;
                        Some(MapCycle::new(line, height))
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
            .collect::<Vec<MapCycle>>();
        let rules = process_rules(rules_string);
        return (map_rows, rules);
    }
    fn get_total_score(pos:[i64;2], dir:Facing) -> i64{
        let mut score = 0;
        score += 1000*pos[0];
        score += 4*pos[1];
        score += match dir{
            Facing::North => 3,
            Facing::South => 1,
            Facing::East => 0,
            Facing::West => 2,
        };
        return score;
    }
    fn print_map(pos_history:&HashMap<[i64;2],Facing>, map:&Vec<MapCycle>){
        // let pos_vec = pos_history.iter().map(|(pos,_)| pos).collect::<Vec<&[i64;2]>>();
        for (y,line) in map.iter().enumerate() {
            for _ in 0..line.void_adjustment{
                print!(" ");
            }
            for (x,element) in line.cycle.iter().enumerate(){
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
    pub fn main_1(file_name:&str)->Option<i64>{
        let data_string = read_to_string(file_name).expect("We know file exists");
        let (mut map,instructions) = process_data_string(&data_string);
        // let mut pos_history = Vec::<([i64;2],Facing)>::new();
        let mut pos_history = HashMap::<[i64;2],Facing>::new();
        let (mut pos, mut direction) = ([0,0], Facing::East);
        println!("{instructions:?}");
        for instruction in instructions.iter() {
            match instruction{
                Orders::Rotate(rot) => {
                    direction = rotate(direction, rot);
                    println!("{pos:?}, {direction:?}");
                    pos_history.insert([pos[0],pos[1]],direction.clone());
                },
                Orders::Straight(n) => {
                    for _ in 0..*n{
                        pos = advance(pos, 1,  &direction, &mut map);
                        println!("{pos:?}, {direction:?}");
                        pos_history.insert([pos[0],pos[1]],direction.clone());
                    }
                }
            }
            
        }
        let global_pos = [pos[0], pos[1] + map[pos[0] as usize].void_adjustment];
        let score = get_total_score(global_pos, direction);
        print_map(&pos_history, &map);
        Some(score)
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
    pub fn main_2(file_name:&str)->Option<i64>{
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
    if file_name == r"src\dummy_input.txt"{
        let expected_value = 6032i64;
        assert!(count.unwrap() == expected_value, "\n- - - Wrong Dummy Answer! Got {}, should be {} - - -\n",count.unwrap(),expected_value)
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}