mod p1{
    use std::fs::read_to_string;
    use std::collections::HashSet;
    use itertools::{Itertools, iproduct};
    pub fn get_data_iter<'a>(data_string:&'a str) -> impl Iterator<Item = [i32;3]>+'a{
        let data = data_string
            .lines()
            .map(|line|{
                let numbers:[i32;3] =line
                    .split(',')
                    .map(|number| number.parse::<i32>().expect("If this fails, the string is split wrong."))
                    .collect::<Vec<i32>>()
                    .try_into()
                    .expect("If this fails, the file is read wrong.");
                numbers
        });
        data
    }
    pub fn get_cube_set(data:impl Iterator<Item = [i32;3]>) -> HashSet<[i32;3]>{
        let cubes: HashSet<[i32;3]> = data
            .map(|cube| (cube))
            .collect::<HashSet<[i32;3]>>();
        cubes
    }
    pub fn get_six_surrounding_cubes(cube:&[i32;3]) -> Vec<[i32;3]>{
        [
        [ cube[0]+1, cube[1],   cube[2]  ],
        [ cube[0]-1, cube[1],   cube[2]  ],
        [ cube[0],   cube[1]+1, cube[2]  ],
        [ cube[0],   cube[1]-1, cube[2]  ],
        [ cube[0],   cube[1],   cube[2]+1],
        [ cube[0],   cube[1],   cube[2]-1],
        ].try_into().expect("arr -> Vec is infallable")
    }

    pub fn get_14_surrounding_cubes(cube:&[i32;3]) -> Vec<[i32;3]>{
        [
        [ cube[0]+1, cube[1],   cube[2]  ],
        [ cube[0]-1, cube[1],   cube[2]  ],
        [ cube[0],   cube[1]+1, cube[2]  ],
        [ cube[0],   cube[1]-1, cube[2]  ],
        [ cube[0],   cube[1],   cube[2]+1],
        [ cube[0],   cube[1],   cube[2]-1],
        [ cube[0]+1, cube[1]-1,   cube[2]  ],
        [ cube[0]+1, cube[1]+1,   cube[2]  ],
        [ cube[0]-1,   cube[1]+1, cube[2]  ],
        [ cube[0]-1,   cube[1]-1, cube[2]  ],
        [ cube[0]+1,   cube[1],   cube[2]+1],
        [ cube[0]-1,   cube[1],   cube[2]+1],
        [ cube[0]+1,   cube[1],   cube[2]-1],
        [ cube[0]-1,   cube[1],   cube[2]-1],

        ].try_into().expect("arr -> Vec is infallable")
    }
    pub fn get_26_surrounding_cubes(cube:&[i32;3]) -> Vec<[i32;3]>{
        let x_range = ((cube[0]-1)..=(cube[0]+1));
        let y_range = ((cube[1]-1)..=(cube[1]+1));
        let z_range = ((cube[2]-1)..=(cube[2]+1));
        let surrounding = iproduct!(x_range,y_range,z_range)
            .filter_map(|(x,y,z)|{
                if &[x,y,z] == cube{
                    None
                }else{
                    Some([x,y,z])
                }
            }
            ).collect::<Vec<[i32;3]>>();
        surrounding
    }
    pub fn main_1(file_name:&str) -> Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let data_iterator = get_data_iter(&data_string);
        let cubes = get_cube_set(data_iterator);
        let mut total_surface_area = 0;

        cubes.iter().for_each(|cube|{
            let neighbours = get_six_surrounding_cubes(&cube);
            for neighbour in neighbours.iter(){
                if !cubes.contains(neighbour) {
                    total_surface_area += 1;
                }
            }
        });
		return Some(total_surface_area);
    }
}
mod p2{
    use core::panic;
    use std::cmp::{max,min};
    use std::{fs::read_to_string, collections::VecDeque};
    use std::collections::{HashSet, HashMap};
    use crate::p1::{get_data_iter, get_cube_set, get_26_surrounding_cubes, get_14_surrounding_cubes,get_six_surrounding_cubes};
    fn get_min_max(cubes:&HashSet<[i32;3]>) -> [i32;6]{
        cubes.iter().fold([i32::MAX,i32::MIN,i32::MAX,i32::MIN,i32::MAX,i32::MIN],|acc,cube|{
            [min(acc[0], cube[0]-1), max(acc[1],cube[0]+1),
             min(acc[2], cube[1]-1), max(acc[3],cube[1]+1),
             min(acc[4], cube[2]-1), max(acc[5],cube[2]+1)]
        })
    }
    fn min_max_check(min_max:[i32;6], current:&[i32;3]) -> bool{
        // Returns true if current does NOT breach min/max limits.
        for i in 0..=2{
            if min_max[(i*2)+1] < current[i] || current[i] < min_max[(i*2)]{
                return false;
            }
        }
        true

    }
    fn depth_first_recursive(current:&[i32;3], end:&[i32;3], traversable_cubes:&HashSet<[i32;3]>, seen:&mut HashSet<[i32;3]>, min_max:[i32;6], depth:i32) -> bool{
        if current == end{
            return true;
        }
        seen.insert(*current);
        for neighbour in get_six_surrounding_cubes(current).iter().filter(|&cube| traversable_cubes.contains(cube) && min_max_check(min_max,cube)){
            if neighbour == &[2,2,5]{
                println!("Stepped into the emtpy space ({neighbour:?}) from {current:?}");
                panic!()
            }
            if !seen.contains(neighbour){
                if depth_first_recursive(neighbour, end, traversable_cubes, seen, min_max,depth+1){
                    return true;
                }
            }
        }
        false
    }
    fn depth_first_search(start:&[i32;3], end:&[i32;3], traversable_cubes:&HashSet<[i32;3]>,min_max:[i32;6])-> bool{
        let mut seen: HashSet<[i32;3]> = HashSet::new();
        let found_end = depth_first_recursive(start, end, traversable_cubes, &mut seen, min_max, 0);
        // println!("\n");
        return found_end;
    }
    fn breadth_first_search(start:&[i32;3], end:&[i32;3], traversable_points:&HashSet<[i32;3]>) -> bool{
        //     1  procedure BFS(G, root) is
        //     2      let Q be a queue
        //     3      label root as explored
        //     4      Q.enqueue(root)
        //     5      while Q is not empty do
        //     6          v := Q.dequeue()
        //     7          if v is the goal then
        //     8              return v
        //     9          for all edges from v to w in G.adjacentEdges(v) do
        //    10              if w is not labeled as explored then
        //    11                  label w as explored
        //    12                  w.parent := v
        //    13                  Q.enqueue(w)
        let mut fronteer: VecDeque<[i32;3]> = VecDeque::new();
        let mut seen = HashSet::<[i32;3]>::new();
        fronteer.push_back(*start);
        seen.insert(*start);
        
        while let Some(current) = fronteer.pop_front(){
            if &current == end{
                return true;
            }
            let neighbours = get_six_surrounding_cubes(&current)
            .iter()
            .filter_map(|&cube|{
                    if traversable_points.contains(&cube) && !seen.contains(&cube){
                        Some(cube)
                    }else{
                        None
                    }
                }).collect::<Vec<[i32;3]>>();
            for neighbour in neighbours.iter(){
                if !seen.contains(neighbour){
                    seen.insert(*neighbour);
                    fronteer.push_back(*neighbour); 
                }
            }
        }
        false
        
    }
    fn can_reach_end_from_start(start:&[i32;3], end:&[i32;3], solid_cubes:&HashSet<[i32;3]>)-> bool{
        let mut min_max = get_min_max(solid_cubes);
        for i in 0..3{ // Include the immediate space around the shape
            min_max[i*2]-=1;
            min_max[i*2+1]+=1;
        }
        let traversable_cubes = get_all_empty_space(&solid_cubes);
        depth_first_search(start, end, &traversable_cubes, min_max,  )
    }

    fn get_empty_neighbours(cubes:&HashSet<[i32;3]>, position:&[i32;3], surrounding_func: fn(&[i32;3])->Vec<[i32;3]>) -> HashSet<[i32;3]>{
        let neighbours = surrounding_func(position);
        neighbours.iter()
            .filter_map(|&neighbour| {
                match cubes.contains(&neighbour){
                    true => None,
                    false => Some(neighbour),
                }
            }).collect::<HashSet<[i32;3]>>()
    }
 
    fn get_all_empty_space(cubes:&HashSet<[i32;3]>)->HashSet<[i32; 3]>{
        let valid_empty_space = cubes.iter().fold(HashSet::new(),|acc,current_cube|{
            let mut empty_current_neighbours = get_empty_neighbours(&cubes, current_cube, get_14_surrounding_cubes);
            empty_current_neighbours.extend(acc);
            empty_current_neighbours
        });
        valid_empty_space
    }
    fn get_all_exterior(start:&[i32;3], traversable_points:&HashSet<[i32;3]>, seen:&mut HashSet<[i32;3]>){
        let mut fronteer: VecDeque<[i32;3]> = VecDeque::new();
        fronteer.push_back(*start);
        seen.insert(*start);
        
        while let Some(current) = fronteer.pop_front(){
            // Get neighbours of 
            let neighbours = get_six_surrounding_cubes(&current)
            .iter()
            .filter_map(|&cube|{
                    if traversable_points.contains(&cube) && !seen.contains(&cube){
                        Some(cube)
                    }else{
                        None
                    }
                }).collect::<Vec<[i32;3]>>();
                for neighbour in neighbours.iter(){
                    if !seen.contains(neighbour){
                        seen.insert(*neighbour);
                        fronteer.push_back(*neighbour); 
                    }
                }
            }
    }
    pub fn main_2(file_name:&str) -> Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        let data_iterator = get_data_iter(&data_string);
        let cubes = get_cube_set(data_iterator);
        let cube_root = cubes.iter().fold([i32::MAX,i32::MAX,i32::MAX], |mut acc, cube3d|{
            if acc[0] > cube3d[0] ||
                (acc[0] == cube3d[0] && acc[1] > cube3d[1]) || 
                    (acc[0] == cube3d[0] && acc[1] == cube3d[1] && (acc[2] > cube3d[2])
                ){
                acc = *cube3d;
            }
            acc
        });
        
        // First we select any face of a cube that is not in a hollow part of the droplet.
        let distant_point = [cube_root[0]-1, cube_root[1]-1, cube_root[2]-1];
        let mut seen = HashSet::<[i32;3]>::new();
        let traversable = get_all_empty_space(&cubes);

        let mut start = None;
        for point in traversable.iter(){
            if can_reach_end_from_start(&distant_point, point, &cubes){
                start = Some(*point);
                break;
            }
        }
        // Updates "seen" to contain everything reachable from the outside of the shape.
        // At least one of the possible points have to be exterior, therefor unwrap is safe.
        println!("Seen initial size: {}", seen.len());
        get_all_exterior(&start.unwrap(), &traversable, &mut seen);
        println!("Seen final size: {}", seen.len());
        // Countable holds every cube that is immediatly adjacent to a solid cube (internal and external)

        let mut countable: HashMap<[i32; 3], i32> = HashMap::new();
        cubes.iter().for_each(|cube|{
            let neighbours = get_six_surrounding_cubes(&cube);
            for neighbour in neighbours.iter(){
                if !cubes.contains(neighbour) {
                    if let Some(val) = countable.get_mut(neighbour){
                        *val += 1;
                    }else{
                    countable.insert(*neighbour,1);

                    }
                }
            }
        });
        let surface_count = countable.iter().filter(|(&cube,_)| seen.contains(&cube) ).fold(0, |mut acc, (cube,count)| acc +count);
        Some(surface_count)

    }

#[cfg(test)]
mod tests{
    use itertools::Itertools;

    use super::*;
    use std::collections::HashSet;
    use crate::p1::get_six_surrounding_cubes;
    #[test]
    fn get_surrounding_test(){
        let current_cube = [1,1,1];
        let correct_answer = [[2,1,1],[0,1,1],[1,2,1],[1,0,1],[1,1,2],[1,1,0]];
        let neighbours = get_six_surrounding_cubes(&current_cube);
        assert_eq!(neighbours,correct_answer);
    }
    #[test]
    fn get_empty_neighbours_test(){
        let current_cube = [1,1,1];
        let mut cubes: HashSet<[i32;3]> = HashSet::new();
        
        cubes.insert([0,0,0]);
        cubes.insert([0,0,1]);
        cubes.insert([0,1,1]);
        // cubes.insert([1,1,1]); // Is current cube
        // cubes.insert([1,1,0]); //Excluded
        cubes.insert([1,0,0]);
        cubes.insert([1,0,1]);
        cubes.insert([0,1,0]);

        cubes.insert([0,0,0]);
        cubes.insert([0,0,2]);
        cubes.insert([0,2,2]);
        cubes.insert([2,2,2]);
        cubes.insert([2,2,0]);
        cubes.insert([2,0,0]);

        // cubes.insert([1,1,1]); // Is current cube
        // cubes.insert([1,1,2]); // Excluded
        cubes.insert([1,2,1]);
        cubes.insert([1,2,2]);
        // cubes.insert([2,2,2]); // Is current cube
        cubes.insert([2,2,1]);
        cubes.insert([2,1,1]);
        cubes.insert([2,1,2]);
        cubes.insert([2,0,2]);
        cubes.insert([0,2,0]);

        cubes.insert([0,1,2]);
        cubes.insert([0,2,1]);
        cubes.insert([2,1,0]);
        cubes.insert([2,0,1]);
        cubes.insert([1,2,0]);
        cubes.insert([1,0,2]);


        let mut correct_answer = HashSet::new();
        correct_answer.insert([1,1,2]);
        correct_answer.insert([1,1,0]);
        let empty_neighbours = get_empty_neighbours(&cubes, &current_cube, get_26_surrounding_cubes);
        assert_eq!(empty_neighbours,correct_answer);
    }
    #[test]
    fn cube_exists_interactions(){
        let current_cube = [1,1,2];
        let mut cubes: HashSet<[i32;3]> = HashSet::new();
        cubes.insert([1,1,1]);
        cubes.insert([1,2,1]);
        cubes.insert([2,1,1]);
        assert!(!cubes.contains(&current_cube));
    }
    #[test]
    fn depth_first_test(){
        let mut cubes = HashSet::new();
        cubes.insert([1,2,2]);
        cubes.insert([2,2,2]);
        cubes.insert([3,2,2]);
        let current_empty = [0,2,2];
        let target_empty = [4,2,2];
        let empty_cubes = get_all_empty_space(&cubes);
        for e in empty_cubes.iter().sorted() {
            if e[2] == 2{
                println!("{e:?}");
            }
        }
        assert!(empty_cubes.contains(&current_empty), "The current cube exists in the space");
        assert!(empty_cubes.contains(&target_empty), "The target cube exists in the space");
        assert!(can_reach_end_from_start(&current_empty, &target_empty, &cubes), "Could not find a path?!");
    }

    fn set_up_test()->(HashSet<[i32; 3]>, [i32; 3], [i32;3]){
        let file_name = r"src\dummy_input.txt";
        let data_string = read_to_string(file_name).unwrap();
        let data_iterator = get_data_iter(&data_string);
        let cubes = get_cube_set(data_iterator);
        let current_cube = cubes.iter().fold([i32::MAX,i32::MAX,i32::MAX], |mut acc, cube3d|{
            if acc[0] > cube3d[0] ||
                (acc[0] == cube3d[0] && acc[1] > cube3d[1]) || 
                    (acc[0] == cube3d[0] && acc[1] == cube3d[1] && (acc[2] > cube3d[2])
                ){
                acc = *cube3d;
            }
            acc
        });
        let search_cube = [current_cube[0]-1, current_cube[1], current_cube[2]];
        (cubes, search_cube, [2,2,5])
    }
    #[test]
    fn reach_hollow_point(){
        let (cubes, search_cube,unreachable_end) = set_up_test();
        let ans = can_reach_end_from_start(&search_cube, &unreachable_end,&cubes);
        assert!(!ans,"{unreachable_end:?} should not be reachable from {search_cube:?}!");
        let search_cube = [3, 2, 1];  // This cube claims, during runtime, that it can not reach a certain point.
        assert!(!ans,"{unreachable_end:?} should not be reachable from {search_cube:?}!");
    }
    #[test]
    fn reach_outer_point(){
        // let search_cube = [current_cube[0]-1, current_cube[1], current_cube[2]];
        let (cubes, search_cube,unreachable_end) = set_up_test();
        for point in get_all_empty_space(&cubes).iter().filter(|&&cube| cube != unreachable_end){
            let ans = can_reach_end_from_start(&search_cube, &point,&cubes);
            assert!(ans,"{point:?} should be reachable from {search_cube:?}!")
        }
        let search_cube = [3, 2, 1];  // This cube claims, during runtime, that it can not reach a certain point.
        for point in get_all_empty_space(&cubes).iter().filter(|&&cube| cube != unreachable_end){
            let ans = can_reach_end_from_start(&search_cube, &point,&cubes);
            assert!(ans,"{point:?} should be reachable from {search_cube:?}!")
        }

        
    }
}
}

use p1::main_1;
use p2::main_2;
use std::time::Instant;
fn main() {
    let file_name = r"src\minimal_input.txt";
    let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
	let start = Instant::now();
    let output = main_1(file_name);
	let duration = start.elapsed();
	println!("Part 1 Result: {output:?}\nRuntime: {duration:?}\n");
    assert!((file_name == r"src\dummy_input.txt" && output == Some(64)) || (file_name == r"src\puzzle_input.txt" && output == Some(4192)) );
	let start = Instant::now();
    let output = main_2(file_name);
	let duration = start.elapsed();
	println!("Part 2 Result: {output:?}\nRuntime: {duration:?}\n");
    assert!((file_name == r"src\dummy_input.txt" && output == Some(58)) || (file_name == r"src\puzzle_input.txt" && output == Some(2520)) );
    // Note: Does not detect holes, as it is allowed to move diagonally.
    // Diagonal movement should only be allowed IFF the spaces adjacent blocks are not there.
    //
    // | a | # | # |    |   | # | # |        
    // |   |   | # | -> |   | a | # | == OK  
    // |   | # | # |    |   |   | # |        
    //
    // | a | # | # |    |   | # | # |        
    // | # |   | # | -> | # | a | # | =/= OK 
    // |   |   | # |    |   |   | # |        
	
}
