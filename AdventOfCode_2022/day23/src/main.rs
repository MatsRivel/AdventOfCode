mod p1{
    use std::{fs::read_to_string, collections::{HashMap, HashSet}, cmp::{min, max}};

    use crate::PRINT1;

    // If no occupied tiles aound, do nothing.
    // Else:
    // Else if no occupied tiles in NW, N, NE:
    // - Propose step N.
    // Else if no occupied tiles in SW, S, SE:
    // - Propose step S.
    // Else if no occupied tiles in SW, W, NW:
    // - Propose step W.
    // Else if no occupied tiles in NE, E, SE:
    // - Propose step E.
    // 
    // If any two proposed moves would leave a tile double-occupied:
    // - None of the moves leading to that tile are performed.
    // 
    // After all moves, move the first section in the "else if no occupied ..." list to the bottom of the list.
    // (So that would move NW,N,NE down to the bottom, and now SW,S,SE is on the top.)
    // 
    // After R rounds, count the number of empty tiles in the smallest square covering all occupied tiles.
    #[derive(Debug)]
    pub enum MovePolicy{
        N,
        E,
        S,
        W,
    }
    fn act_on_policy<'a>(policies: &mut impl Iterator<Item = &'a MovePolicy>, pos:&[i32;2], map:&HashSet<[i32;2]>) -> Option<[i32;2]>{
        // Returns a position of the tile-occupier moves. Returns None if it remains.
        let mut output = None;
        if !has_any_neighbour(&pos, map){
            return output;
        }
        'policies_loop: for _ in 0..4{
            let pol = policies.next().unwrap(); // Policies are cyclic, so no worries there.
            if output.is_none(){ // Only act if we have not yet selected where to go
                let [x,y] = *pos;
                let neighbours = match pol{
                    MovePolicy::N => [[x-1, y-1], [x-1,y], [x-1,y+1]],
                    MovePolicy::E => [[x-1, y+1], [x, y+1], [x+1, y+1]],
                    MovePolicy::S => [[x+1, y-1], [x+1,y], [x+1,y+1]],
                    MovePolicy::W => [[x-1, y-1], [x, y-1], [x+1, y-1]],
                };
                // Check if all neighbours (kinda) are free
                for nei in neighbours.iter(){
                    if map.contains(nei){
                        continue 'policies_loop;
                    }
                }
                output = Some(neighbours[1]); // No objections -> We've found our spot!
            } 
            // Rotate the policy so that we make sure it is in the right spot for the next act.
            // This is why we do not early return!
        }
        return output;
    }
    pub fn process_datastring(data_string: &str) -> HashSet<[i32;2]>{
        let positions = data_string
            .lines()
            .enumerate()
            .map(|(outer_idx, line)|{
                let row = line
                    .char_indices()
                    .filter_map(|(inner_idx, c)| {
                        match c{
                            '#' => Some([outer_idx as i32, inner_idx as i32]),
                            _ => None
                        }
                    }).collect::<HashSet<[i32;2]>>();
                row
            }).fold(HashSet::<[i32;2]>::new(), |mut acc, row|{
                acc.extend(row);
                acc
            });
        
        positions
    }
    fn print_map_1(map:&HashSet<[i32;2]>){
        if !PRINT1{
            return;
        }
        print_map(map)
    }
    pub fn print_map(map:&HashSet<[i32;2]>){
        let spacing = " ";
        let padding = 2;
        let [min_x, max_x, min_y, max_y] = map.iter()
        .fold([i32::MAX, i32::MIN, i32::MAX, i32::MIN], |acc,[x,y]|{
            [min(acc[0],*x), max(acc[1],*x), min(acc[2],*y), max(acc[3],*y)]
        }
        );
        for i in (min_x-padding)..=(max_x+padding){
            for j in (min_y-padding)..=(max_y+padding){
                if [i,j] == [0,0]{
                    print!("O{spacing}");

                }else if map.contains(&[i,j]){
                    print!("#{spacing}");
                }else{
                    print!(".{spacing}");
                }
            }
            println!();
        } 
        println!("\n");
    }
    pub fn get_move_propositions<'a>(occupied_tiles:&HashSet<[i32; 2]>, mut policies: &mut impl Iterator<Item = &'a MovePolicy>) -> HashMap<[i32; 2], [i32; 2]>{
        occupied_tiles
            .iter()
            .map(|pos|{
                match act_on_policy(&mut policies, pos, &occupied_tiles){
                    Some(new_pos) => (*pos,new_pos),
                    None => (*pos,*pos)
                }
            }).collect::<HashMap<[i32;2],[i32;2]>>()
    }
    pub fn get_collisions(move_propositions:&HashMap<[i32; 2], [i32; 2]>) -> HashMap<[i32; 2], bool>{
        let mut col = HashMap::<[i32;2],bool>::new();
        move_propositions.iter()
            .for_each(|(_,new_pos)|{
                let collides = col.contains_key(new_pos);
                col.insert(*new_pos,collides);
            }
        );
        col
    }
    pub fn get_post_move_tiles(occupied_tiles:&HashSet<[i32; 2]>,move_propositions:&HashMap<[i32; 2], [i32; 2]>, collisions:&HashMap<[i32; 2], bool>) -> HashSet<[i32; 2]>{
        occupied_tiles.iter()
            .map(|old_pos|{
                let new_pos = move_propositions.get(old_pos).expect("Know key is present");
                if old_pos != new_pos && !*collisions.get(new_pos).expect("Certain that key is in the other table too."){
                    // If there is movement suggested, and it does not have a collision:
                    // Move
                    *new_pos
                }else{
                    *old_pos
                }
            }).collect::<HashSet<[i32;2]>>()
    }
    fn get_min_max(occupied_tiles:&HashSet<[i32; 2]>) -> [i32;4]{
        occupied_tiles.iter()
        .fold([i32::MAX, i32::MIN, i32::MAX, i32::MIN], |acc,[x,y]|{
            [min(acc[0],*x), max(acc[1],*x), min(acc[2],*y), max(acc[3],*y)]
        })
    }
    fn get_min_covering_square_area(occupied_tiles:&HashSet<[i32; 2]>) -> i32{
        let [min_x, max_x, min_y, max_y] = get_min_max(occupied_tiles);
        let mut area = (1 + max_x - min_x) * (1 + max_y-min_y);
        area
    }
    fn has_any_neighbour(pos:&[i32;2], occupied_tiles:&HashSet<[i32; 2]>) -> bool{
        for i in -1..=1{
            for j in -1..=1{
                if i == 0 && j == 0{
                    continue;
                }
                let new_pos = [pos[0]+i, pos[1]+j];
                if occupied_tiles.contains(&new_pos){
                    return true;
                }
            }
        }
        false
    }
    pub fn main_1(file_name:&str, rounds:u32)->Option<i32>{
        // Read tiles. Only save positions of occupied tiles. HashSet of all occupied positions.
        let data_string = read_to_string(file_name).unwrap();
        let mut occupied_tiles = process_datastring(&data_string);

        print_map_1(&occupied_tiles);
        // Keep a cyclic iter of movement policies?
        let mut policies = [MovePolicy::N,MovePolicy::S,MovePolicy::W,MovePolicy::E].iter().cycle();
        //Make a HashMap of suggested positions, where the key is the pos and the val is the tile suggesting to move there.
        // For each key in the hashmap, if the val is only ONE position, move that position to the keys position.
        for r in 0..rounds{
            // Make a list of acceptable new positions.
            let move_propositions = get_move_propositions(&occupied_tiles, &mut policies);
            
            // For each of the potential new positions, check if it is acceptable.
            let collisions= get_collisions(&move_propositions);
            
            // For each move that does not result in a collision, perform that move.
            occupied_tiles = get_post_move_tiles(&occupied_tiles, &move_propositions, &collisions);
            policies.next(); // Cycle policies by 1 to put current first into the back.
            print_map_1(&occupied_tiles);
        }
        println!();
        // Hashmap can now be dropped.
        // Repeat cycle until "rounds" is reached.
        let mut area = get_min_covering_square_area(&occupied_tiles);
        // println!("Area: {area}");
        area -= occupied_tiles.len() as i32;
        // println!("Free Tiles: {area}");
        // Calculate square.
        // Empty space = x_side * y_size - n_occupied_tiles at start.
        Some(area)

    

    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn policy_test(){
        let file_name = r"src\test_input.txt";
        let data_string = read_to_string(file_name).unwrap();
        let mut occupied_tiles = process_datastring(&data_string);
        let rounds = 1;
        print_map(&occupied_tiles);
        // Keep a cyclic iter of movement policies?
        let mut policies = [MovePolicy::N,MovePolicy::S,MovePolicy::W,MovePolicy::E].iter().cycle();
        //Make a HashMap of suggested positions, where the key is the pos and the val is the tile suggesting to move there.
        // For each key in the hashmap, if the val is only ONE position, move that position to the keys position.
        for r in 0..rounds{
            // Get all potential new positions:
            for _ in 0..4{
                print!("{:?} -> ", policies.next().unwrap());
            }
            // Make a list of acceptable new positions.
            let move_propositions = get_move_propositions(&occupied_tiles, &mut policies);
            
            // For each of the potential new positions, check if it is acceptable.
            let collisions= get_collisions(&move_propositions);
            
            // For each move that does not result in a collision, perform that move.
            occupied_tiles = get_post_move_tiles(&occupied_tiles, &move_propositions, &collisions);
            policies.next(); // Cycle policies by 1 to put current first into the back.
            println!("End of round {}:",r+1);
            print_map(&occupied_tiles);
            // Hashmap can now be dropped.
            // Repeat cycle until "rounds" is reached.
            let area = get_min_covering_square_area(&occupied_tiles);
            let free_area = area- occupied_tiles.len() as i32;
            // Calculate square.
            // Empty space = x_side * y_size - n_occupied_tiles at start.
            match r{
                0 => {
                    let target_area = 4;
                    assert_eq!(area,target_area,"Area is {area} but should be {target_area}!");
                },
                1 => {
                    let target_area = 4;
                    assert_eq!(area,target_area,"Area is {area} but should be {target_area}!");
                },
                _ =>(),
            }
        }

	}
	
}

}
mod p2{
    use std::collections::HashSet;
    use std::time;
    use std::{fs::read_to_string, thread};
    use crate::PRINT2;

    use crate::p1::{process_datastring, print_map, get_move_propositions, get_collisions, get_post_move_tiles, MovePolicy};
    fn print_map_2(map:&HashSet<[i32;2]>){
        if !PRINT2{
            return;
        }
        print_map(map)
    }
    pub fn main_2(file_name:&str)->Option<i32>{
        // Read tiles. Only save positions of occupied tiles. HashSet of all occupied positions.
        let data_string = read_to_string(file_name).unwrap();
        let mut occupied_tiles = process_datastring(&data_string);

        print_map_2(&occupied_tiles);
        // Keep a cyclic iter of movement policies?
        let mut policies = [MovePolicy::N,MovePolicy::S,MovePolicy::W,MovePolicy::E].iter().cycle();
        //Make a HashMap of suggested positions, where the key is the pos and the val is the tile suggesting to move there.
        // For each key in the hashmap, if the val is only ONE position, move that position to the keys position.
        for r in 0..{
            if PRINT2{
                for _ in 0..4{
                    print!("{:?} ",policies.next().expect("Cyclical iterator is infallable."));
                }
                println!();
            }
            // Make a list of acceptable new positions.
            let move_propositions = get_move_propositions(&occupied_tiles, &mut policies);
            
            // For each of the potential new positions, check if it is acceptable.
            let collisions= get_collisions(&move_propositions);

            // let duration = time::Duration::from_millis(100);
            // thread::sleep(duration); 

            // For each move that does not result in a collision, perform that move.
            let post_move_occupied_tiles = get_post_move_tiles(&occupied_tiles, &move_propositions, &collisions);
            let no_moves = post_move_occupied_tiles.iter().fold(true, |acc, tile|{
                let no_move = occupied_tiles.contains(tile);
                // If every tile is present in the old tile-set, we made no moves. Therefor we stop searching.
                // If a single tile moved, this no_moves evaluates to false.
                acc && no_move
            });
            if no_moves{
                return Some(r+1);
            }
            occupied_tiles = post_move_occupied_tiles;
            policies.next(); // Cycle policies by 1 to put current first into the back.
            print_map_2(&occupied_tiles);
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

use p1::main_1;
use p2::main_2;
use std::time::Instant;
const PRINT1:bool = false;
const PRINT2:bool = false;
fn main() {
    let file_name = r"src\test_input.txt";
    let file_name = r"src\dummy_input.txt";
    let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name, 10);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}\n___________________________________________");

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        let expected_result = 20;
        let actual_result = count.unwrap();
        assert_eq!(actual_result,expected_result,"Result was {actual_result}, expected {expected_result}" );
    }
}

