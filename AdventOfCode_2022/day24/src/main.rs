
mod BlizzardPack{
    use std::cmp::{min, max};

    #[derive(PartialEq,Debug)]
    pub enum Axis{
        N,
        S,
        W,
        E,
    }
    #[derive(Debug)]
    pub struct Blizzard{
        pub initial_pos:[u32;2], // To identify this blizzard.
        pub dir: Axis, // Is it traveling horizontally or vertically?
        pub min_pos: [u32;2], // Lowest position it can reach. (To account for "exits")
        pub max_pos: [u32;2], // Highest position it can reach (To account for "exits")
    }
    impl Blizzard{
        pub fn pos_at_step_n(&self, n:&u32)->[u32;2]{
            // println!("Dir: {:?}, Max_pos: {:?}", self.dir, self.max_pos);
            let new_pos = match &self.dir{
                Axis::N => {
                    let [mut vert,horiz] = self.initial_pos;
                    let [max_vert, max_horiz] = self.max_pos;
                    let [min_vert, min_horiz] = self.min_pos;
                    // println!("Dir: {:?}, {vert:?}, {horiz:?}, n: {n}", self.dir);
                    vert += n;
                    while vert> max_vert{
                        vert -= max_vert;
                    }
                    [vert, horiz]
                },
                Axis::S => {
                    let [mut vert,horiz] = self.initial_pos;
                    let [max_vert, max_horiz] = self.max_pos;
                    let [min_vert, min_horiz] = self.min_pos;
                    // println!("Dir: {:?}, {vert:?}, {horiz:?}, n: {n}", self.dir);
                    while vert < min_vert+n{
                        vert += max_vert;
                    }
                    vert -= n;
                    [vert, horiz]
                },
                Axis::W => {
                    let [vert,mut horiz] = self.initial_pos;
                    let [max_vert, max_horiz] = self.max_pos;
                    let [min_vert, min_horiz] = self.min_pos;
                    // println!("Dir: {:?}, {vert:?}, {horiz:?}, n: {n}", self.dir);
                    while horiz < min_horiz+n{
                        horiz += max_horiz;
                    }
                    horiz -= n;
                    [vert, horiz]
                },
                Axis::E => {
                    let [vert,mut horiz] = self.initial_pos;
                    let [max_vert, max_horiz] = self.max_pos;
                    let [min_vert, min_horiz] = self.min_pos;
                    // println!("Dir: {:?}, {vert:?}, {horiz:?}, n: {n}", self.dir);
                    horiz += n;
                    while horiz > max_horiz{
                        horiz -= max_horiz;
                    }
                    [vert, horiz]
                }
            };

            // println!(">>{new_pos:?}");
            new_pos
        }
    }
    pub trait BlizzardState
    where
        for<'a> &'a Self: IntoIterator<Item = &'a Blizzard>,
    {
        fn get_current_state(&self, time: u32) -> Vec<[u32; 2]> {
            self.into_iter().map(|b| b.pos_at_step_n(&time)).collect::<Vec<[u32; 2]>>()
        }
        fn get_min_max(&self) -> [[u32;2];2]{
            // Very much a wasted resource here. // TODO: Just take in the true min/max. This is done only to account for the two edge-cases..
            let min = self.into_iter().fold([u32::MAX;2], |acc,v|{
                [min(acc[0],v.min_pos[0]), min(acc[1], v.min_pos[1])]
            });
            let max = self.into_iter().fold([u32::MIN;2], |acc,v|{
                [max(acc[0],v.max_pos[0]), max(acc[1], v.max_pos[1])]
            });
            
            [min,max]
        }
        fn pos_is_in_valid_area(&self, pos:&[u32;2], min:&[u32;2], max:&[u32;2] )->bool{
            pos[0] >= min[0] && pos[1] >= min[1] && pos[0] <= max[0] && pos[1] <= max[1]
        }
        fn get_current_adjasent_state(&self, time: u32, pos:&[u32;2], min:&[u32;2], max:&[u32;2]) -> Vec<[u32; 2]> {
            self.into_iter().filter_map(|b| {
                let new = b.pos_at_step_n(&time);
                let in_valid_area = self.pos_is_in_valid_area(pos, min, max);
                println!("new overlaps with pos: {new:?} | {pos:?} | {} | In valid area? {in_valid_area}", new[0]==pos[0] || new[1]==pos[1]);
                if (new[0] != pos[0] || new[1] != pos[1]) && in_valid_area{
                    Some(new)
                }else{
                    None
                }               
            }).collect::<Vec<[u32; 2]>>()
        }
        fn get_available_space(&self, time: u32, pos:&[u32;2], min:&[u32;2], max:&[u32;2]) -> Vec<[u32; 2]> {
            let mut surrounding: Vec<[u32;2]> = Vec::new();
            for i in -1..=1i32{
                for j in -1..=1i32{
                    let new_raw = [pos[0] as i32+i, pos[1] as i32+j];
                    // println!("new_raw: {new_raw:?}, {min:?}, {max:?}");
                    if i > 0 && j > 0 && i < 5 && j < 5{
                        let new = [new_raw[0] as u32, new_raw[1] as u32];
                        surrounding.push(new);
                    }
                }
            }
            let occupied = self.into_iter().map(|b|{
                b.pos_at_step_n(&time)
            }).collect::<Vec<[u32;2]>>();

            surrounding.iter().filter_map(|pos|{
                if occupied.contains(pos){
                    None
                }else{
                    Some(*pos)
                }
            }).collect::<Vec<[u32;2]>>()
        }
    }
}
mod ActorPack{
    pub enum Move{
        N,
        S,
        L,
        R,
        O
    }
}
mod p1{
    use core::time;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use crate::BlizzardPack::{Axis, Blizzard, BlizzardState};
    use crate::ActorPack::Move;
    use priority_queue::DoublePriorityQueue;
    
    impl BlizzardState for Vec<Blizzard>{} // Default implementation :)
    pub fn process_string(data_string:&str)-> (Vec<Blizzard>, [u32;2]){
        let mut bottom_right_wall = [0u32;2]; // Will be altered for each step of the generation below.

        let mut blizzard_map = data_string.lines()
            .enumerate()
            .fold(Vec::<Blizzard>::new(), |mut acc, (i,line)|{
                bottom_right_wall[0] = i as u32; // temp solution; Has unnessecry rewrites.
                let row = line.char_indices()
                    .filter_map(|(j,c)|{
                        bottom_right_wall[1] = j as u32;// temp solution; Has unnessecry rewrites. 
                        let [x,y] = [i as u32, j as u32];
                        match c{
                            '^' => Some(Blizzard{
                                initial_pos:[x,y],
                                dir:Axis::N,
                                min_pos:[0,0],
                                max_pos:[0,0],
                            }),
                            'v' => Some(Blizzard{
                                initial_pos:[x,y],
                                dir:Axis::S,
                                min_pos:[0,0],
                                max_pos:[0,0],
                            }),
                            '<' => Some(Blizzard{
                                initial_pos:[x,y],
                                dir:Axis::W,
                                min_pos:[0,0],
                                max_pos:[0,0],
                            }),
                            '>' => Some(Blizzard{
                                initial_pos:[x,y],
                                dir:Axis::E,
                                min_pos:[0,0],
                                max_pos:[0,0],
                            }),
                            _ => None,
                        }
                    }
                );
                for b in row{
                    acc.push(b);
                }
                acc
            }
        );

        // Adjust for edge case of blizard reaching entrance/exit, which gives them 1 more space to move to.
        blizzard_map.iter_mut()
            .for_each(|val|{
                if val.initial_pos[1] == bottom_right_wall[1]-1 && (val.dir == Axis::S || val.dir == Axis::N){
                    val.max_pos = [bottom_right_wall[0], bottom_right_wall[1]-1] // Might reach exit.
                }else{
                    val.max_pos = [bottom_right_wall[0]-1, bottom_right_wall[1]-1]
                }

                if val.initial_pos[1] == 1 && (val.dir == Axis::S || val.dir == Axis::N){
                    val.min_pos = [0u32,1u32]; // Might reach entrance
                } // Initial pos is already set to [1,1] as default.s
         }
        );
        (blizzard_map, [bottom_right_wall[0]-1, bottom_right_wall[1]-1])
    }
    fn distance_to_end(from:&[u32;2], to: &[u32;2])->u32{
        let x_dist = {
            if from[0] > to[0]{
                from[0]-to[0]
            }else{
                to[0] - from[0]
            }
        };
        let y_dist = {
            if from[1] > to[1]{
                from[1]-to[1]
            }else{
                to[1] - from[1]
            }
        };
        println!("Distance from {from:?} to {to:?} = {}",x_dist+y_dist);
        x_dist+y_dist
    }
    fn distance_to_neighbour(from:&[u32;2], to: &[u32;2])->u32{
        distance_to_end(from,to)
    }
    pub fn print_map(b_map:&Vec<Blizzard>, current_pos:&[u32;2], target:&[u32;2], time:u32, min:&[u32;2], max:&[u32;2]){
        for _ in (min[1])..=(max[1]+2){
            print!("# ");
        }
        println!();
        for i in (min[0])..=(max[0]){
            print!("# ");
            for j in (min[1])..=(max[1]){
                let pos = [i,j];
                if pos == *target{
                    print!("O");
                }else if pos == *current_pos{
                    print!("X");
                }else if b_map.get_current_state(time).contains(&pos){
                    print!("H");
                }else{
                    print!(".");
                }
                print!(" ");
            }
            println!("#");
        }
        for _ in (min[1])..=(max[1]+2){
            print!("# ");
        }
        println!();
    }
    pub fn a_star_search(b_map:&Vec<Blizzard>, current_pos:&[u32;2], target:&[u32;2])-> Option<u32>{
        let default_gscore = 500;
        let mut open_set: DoublePriorityQueue<[u32;2], u32> = DoublePriorityQueue::new();
        // let mut came_from: HashMap<&[u32;2],[u32;2]> = HashMap::new();
        let mut gscore: HashMap<[u32;2],u32> = HashMap::new();
        let mut fscore: HashMap<[u32;2],u32> = HashMap::new(); // fscore[n] == gscore[n] + h(n)
        open_set.push(*current_pos, default_gscore as u32);
        gscore.insert(*current_pos, 0u32);
        fscore.insert(*current_pos, distance_to_end(current_pos, target));
        let [min,max] = b_map.get_min_max();
        while !open_set.is_empty(){
            let (current, current_value) = open_set.pop_min().expect("Can not fail, as empty set breaks the loop first.");
            print_map(b_map, &current, target, current_value, &min, &max);
            println!("Current: {current:?}, Target: {target:?}, == {}", current==*target);
            if current == *target{
                return Some(current_value);
            }else if current[0] > 5 || current[1] > 5{
                continue;
            }
            let neighbours = b_map.get_available_space(current_value, &current, &min, &max);
            // println!("Current: {current_pos:?}, Val: {current_value}, neighbours: {:?}", neighbours);
            for neighbour in neighbours.iter(){
                let tentative_gscore = *gscore.get(&current).get_or_insert(&default_gscore) + distance_to_neighbour(&current, neighbour);
                // println!("\t- Neighbout: {:?}, Score: {tentative_gscore}",neighbour);
                if &tentative_gscore < *gscore.get(neighbour).get_or_insert(&default_gscore){
                    // came_from.insert(neighbour, *current);
                    gscore.insert(neighbour.clone(), tentative_gscore);
                    fscore.insert(neighbour.clone(),tentative_gscore + distance_to_end(neighbour, target));
                    if let None = open_set.get(neighbour){
                        open_set.push(neighbour.clone(), *fscore.get(neighbour).expect("We just inserted it...") as u32);
                    }
                }
            }   
        }
        None
    }
    fn search(b_map:&Vec<Blizzard>, current_pos:&[u32;2], target:&[u32;2])-> Option<u32>{
        a_star_search(b_map, current_pos, target)
    }
    pub fn main_1(file_name:&str)->Option<u32>{
        let data_string = read_to_string(file_name).unwrap();
        let (b_map, target) = process_string(&data_string);
        println!("{b_map:?}");
        let time_found = search(&b_map, &[0,1], &target);
        time_found
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
    if file_name == r"src\dummy_input.txt"{
        let expected_answer = 18;
        let ans = count.unwrap();
        assert_eq!(ans,expected_answer,"\n--- Wrong answer! Got {ans}, expected {expected_answer} ---\n")
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}