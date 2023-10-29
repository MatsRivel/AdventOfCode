mod p1{
    use std::fs::read_to_string;
    use std::cmp::{min, max};
    pub struct Walls{
        pub wall_list: Vec<[[i32;2];2]>,
        pub sand_spawn:[i32;2]
    }
    impl Walls{
        fn merge_walls(&mut self, sort_by_x:bool){
             // add the last element again so that the ".skip(1)" dosn't prevent the last wall from being added.
             // The extra wall will be added into the existing wall.
             self.wall_list.push(self.wall_list[self.wall_list.len() - 1]);
             let mut new_wall_list: Vec<[[i32; 2]; 2]>  = Vec::with_capacity(self.wall_list.len());
             let mut current_wall:[[i32;2];2] = *self.wall_list.first().expect("We know there are walls in the list already.");
             let (idx, alt_idx) = match sort_by_x{
                true => (0,1),
                false => (1,0)
             };
             for wall in self.wall_list.iter().skip(1){
                let [current_left, current_right] = current_wall;
                let [wall_left, wall_right] = *wall;

                // The walls have no overlap. Push and leave.
                if !(current_left[alt_idx] == wall_left[alt_idx] && current_left[alt_idx] == wall_left[alt_idx] && current_right[alt_idx] == wall_left[alt_idx]){
                    new_wall_list.push(current_wall);
                    current_wall = *wall;
                    continue;
                }
                
                // They overlap on current_walls right side: ([)]
                if current_left[idx] <= wall_left[idx] && current_right[idx] <= wall_left[idx] && current_right[idx] >= wall_left[idx]{
                    current_wall = [current_left, wall_left];

                // Upcoming wall just engulfs with the current one: [()]
                }else if current_left[idx] >= wall_left[idx] && current_right[idx] <= wall_right[idx] {
                    current_wall = *wall;
                
                // They overlap on current_walls left side: [(])
                }else if current_left[idx] >= wall_left[idx] && current_right[idx] >= wall_right[idx] && current_left[idx] <= wall_right[idx]{
                    current_wall = [wall_left, current_right];

                }//else { Current wall completely engulfs upcoming wall, so no change.}

             }
             // If we haven't added the last wall yet, add it.
             if current_wall != *new_wall_list.last().expect("In this case, we are fairly certain that something is there. If not, panic is acceptable."){
                new_wall_list.push(current_wall);
             }
                //  self.wall_list.iter()
                //      .zip(self.wall_list.iter().skip(1))
                //          .map(|(wall_a, wall_b)|{
                //              let [a_left, a_right] = wall_a;
                //              let [b_left, b_right] = wall_b;
                //              // Check that the walls are vertical and alligned.
                //              print!("{wall_a:?} + {wall_b:?}");
                //              if !(a_left[0] == b_left[0] && a_left[0] == b_right[0] && a_right[0] == b_right[0]){
                //                 println!(" --> {:?}", *wall_a);
                //                  return *wall_a;
                //              }
                //              let new_wall;
                //              if a_left[1] <= b_left[1] && a_right[1] <= b_right[1]{
                //                  new_wall = [*a_left, *b_right];
                //              }else if a_left[1] >= b_left[1] && a_right[1] <= b_right[1] {
                //                  new_wall = *wall_b;
                //              }else if a_left[1] >= b_left[1] && a_right[1] >= b_right[1]{
                //                  new_wall = [*b_left, *a_right];
                //              }else {// if a_left[1] <= b_left[1] && a_right[1] >= b_right[1] {
                //                  new_wall = *wall_a;
                //              }
                //              println!(" --> {:?}", new_wall);
                //              new_wall
                //      }).collect::<Vec<[[i32;2];2]>>()
                //  };
             self.wall_list = new_wall_list;
        }

        pub fn add_wall(&mut self, wall_input:[[i32;2];2]){
            // Wall_input = [higher_wall, lower_wall];
            let mut walls_merged:bool = false;
            for wall in self.wall_list.iter_mut(){
                // Must overlap on the ends, and both walls must be vertical.
                if wall[0] == wall_input[1] && wall[1][0] == wall_input[0][0]{
                    wall[0] = wall_input[0]; // Effectively added 1 to the height.
                    walls_merged = true;
                    break;
                }
            }
            if !walls_merged{
                self.wall_list.push(wall_input);
            }
            self.wall_list.sort_by(|a,b| a[0][1].cmp(&b[0][1]));
        }
        pub fn overlaps_walls(&self, pos:&[i32;2]) -> bool{ // Probably noooot the best way of doing this... Temporarily a solution, at least. (Low memory, high compute)
            for wall in self.wall_list.iter() {
                for (left_wall, right_wall) in wall.iter().zip(wall.iter().skip(1)) {
                    if is_a_between_b_and_c( pos, left_wall, right_wall){
                        return true;
                    }
                }
            }
            false
        }
        pub fn overlaps_walls_printing(&self, pos:&[i32;2]) -> bool{ // Probably noooot the best way of doing this... Temporarily a solution, at least. (Low memory, high compute)
            for wall in self.wall_list.iter() {
                for (left_wall, right_wall) in wall.iter().zip(wall.iter().skip(1)) {
                    if is_a_between_b_and_c_printing( pos, left_wall, right_wall){
                        return true;
                    }
                }
            }
            false
        }
        pub fn crossing_lines(vertical_wall:[[i32;2];2], other_wall:[[i32;2];2]) -> Option<[i32;2]>{
            let a1 = vertical_wall[0];
            let a2 = vertical_wall[1];
            let b1 = other_wall[0];
            let b2 = other_wall[1];
            let b_left_of_a1 = b1[1] < a1[1] && b2[1] < a1[1];
            let b_left_of_a2 = b1[1] < a2[1] && b2[1] < a2[1];
            let b_right_of_a1 = b1[1] > a1[1] && b2[1] > a1[1];
            let b_right_of_a2 = b1[1] > a2[1] && b2[1] > a2[1];
            if (b_left_of_a1 && b_left_of_a2) || (b_right_of_a1 && b_right_of_a2){
                return None; // No collision
            }
            // Return intersection of two lines.
            todo!()
        }
        pub fn falling_sand_impact_coords(&self,falling_from_point:&[i32;2]) -> Option<[i32;2]>{
            for wall in self.wall_list.iter() {
                for (b,c) in wall.iter().zip(wall.iter().skip(1)){
                     // To check if drop and walls intersect, just check the y-axis of the walls and the x-axis of the drop:
                     let drop_intersection;
                     if b[1] >= falling_from_point[1]{ // The sand falls from 0 to inf, so b being a higher index means its below the sand.
                        drop_intersection = [falling_from_point[0],b[1]];
                     }else{
                        continue;
                     }
                     if is_a_between_b_and_c(&drop_intersection, b, c){
                        return Some(drop_intersection);
                     }
                }
            }
            None
        }

        pub fn print_walls(&self){
            let [min_x, max_x, _, max_y] = {
                let extremities = self.wall_list.iter().fold([i32::MAX,i32::MIN,i32::MAX,i32::MIN], |acc, wall|{
                    let inner_acc = wall.iter().fold(acc, |a, coord|{
                            [min(a[0],coord[0]), max(a[1],coord[0]), min(a[2],coord[1]), max(a[3],coord[1])]
                        });
                    [min(acc[0], inner_acc[0]), max(acc[1], inner_acc[1]), min(acc[2], inner_acc[2]), max(acc[3], inner_acc[3])]
                });
                extremities
            };
            let min_y = 0;
            let spacing = " ";
            for j in (min_y)..=(max_y){
                for i in (min_x-1)..=(max_x+1){
                    let pos = [i,j];
                    // print!("{pos:?} | ");
                    if pos == self.sand_spawn{
                        print!("x{spacing}");
                    }else if self.overlaps_walls(&pos){
                        print!("#{spacing}");
                    }else{
                        print!(".{spacing}");
                    }
                }
                println!();
            }
        }
    }
    

    pub fn is_a_between_b_and_c_printing(a:&[i32;2], b:&[i32;2], c:&[i32;2]) -> bool{
        let output = is_a_between_b_and_c(a, b, c);
        println!("b -> a -> c: {b:?} {a:?} {c:?} | {output}");
        output
    }
    pub fn is_a_between_b_and_c(a:&[i32;2], b:&[i32;2], c:&[i32;2]) -> bool{
        if a == b || a == c{ // Addressing bugg where 1d walls dont work.
            return true;
        }
        let mut axis = None;
        if b[0] == c[0] && a[0] == b[0]{        // They are on the same column:
            axis = Some(0usize); 
        }else if b[1] == c[1] && a[1] == b[1]{  // They are on the same row:
            axis = Some(1usize);
        }
        let output = match axis{
            Some(0) => {
                let v = 1;
                (b[v] <= a[v] && a[v] <= c[v]) || (c[v] <= a[v] && a[v] <= b[v])
            },
            Some(1) => {
                let v = 0;
                (b[v] <= a[v] && a[v] <= c[v]) || (c[v] <= a[v] && a[v] <= b[v])
            },
            _ => false
        };
        output
    }

    pub fn get_wall_segments(file_name:&str) -> Vec<[[i32; 2]; 2]>{
        // Parse stromg tp get walls the way they're shown in the input file.
        let big_walls = {
            read_to_string(file_name)
                .unwrap()
                .lines()
                .map(|line|{
                    let segments = line
                        .split("->")
                        .map(|segment|{
                            let xy = segment
                                .split(",")
                                .map(
                                |axis| { 
                                    let axis = axis.trim_end().trim(); // Remove whitespace that breaks parsing.
                                    axis.parse::<i32>()
                                        .expect("Unwrapping should not fail, as we know the input.")
                                    }
                            ).collect::<Vec<i32>>()
                            .try_into()
                            .expect("Failed to turn innermost vec into arr.");
                            xy
                        }).collect();
                    segments 
                }).collect::<Vec<Vec<[i32;2]>>>()
        };

        // Divide into single lines so that they are sortable.
        let mut small_walls: Vec<[[i32; 2]; 2]> = Vec::with_capacity(big_walls.len()*2);
        for big_wall in big_walls.iter(){
            for (a_wall, b_wall) in big_wall.iter().zip(big_wall.iter().skip(1)){
                let mut wall_line = [*a_wall, *b_wall];
                wall_line.sort_by(|a,b| a[1].cmp(&b[1]));
                small_walls.push(wall_line);
            }
        }
        small_walls
    }

    pub fn main1(file_name:&str) -> i32{
        let sand_source = [500,0];
        let mut wall_segments = get_wall_segments(file_name);
        wall_segments.sort_by(|a,b| a[0][1].cmp(&b[0][1]));

        let mut wall_collection = Walls{
            wall_list: wall_segments,
            sand_spawn: sand_source,
        };
        let mut sand_unit_counter = 0;
        'outer: loop{
            // Set the spawn to originate at the spawn point.
            let mut current_pos = wall_collection.sand_spawn;
            'inner: loop{
 

                // Check if (and where) the sand hits a block
                let sand_hits_block = match wall_collection.falling_sand_impact_coords(&current_pos){
                    Some(v) => v,
                    None => break 'outer // If it never hits a block, it is out of the map, so we are done.
                };
                current_pos = [sand_hits_block[0],sand_hits_block[1]-1];
                let lower_left_right: [[i32; 2]; 2] = [[current_pos[0]-1,current_pos[1]+1], [current_pos[0]+1,current_pos[1]+1]];
                // This vec is never greater than 2 in size.
                let available_space: Vec<[i32; 2]> = {
                    lower_left_right
                        .iter()
                        .filter_map(|pos| {
                            match wall_collection.overlaps_walls(pos){
                                true => None,
                                false => Some(*pos) 
                            }
                        }).collect::<Vec<[i32;2]>>()
                };
                if available_space.is_empty(){ // Sand settles.
                    wall_collection.add_wall([current_pos,[current_pos[0], current_pos[1]+1]]);
                    break 'inner; // Repeat with new sand block.
                }else{ // If not, drop the sand down to the side once.
                    current_pos = available_space[0];

                }
                // Then repeats
            }
            // println!();
            sand_unit_counter += 1; // One more sand block has settled.   
        }
        // wall_collection.print_walls();
        // for wall in wall_collection.wall_list.iter(){
        //     println!("{wall:?}");
        // }
        // println!("Size: {}",wall_collection.wall_list.len());
        // println!();
        // wall_collection.merge_walls(true);
        // for wall in wall_collection.wall_list.iter(){
        //     println!("{wall:?}");
        // }
        // wall_collection.print_walls();
        // println!("Size: {}",wall_collection.wall_list.len());
        // println!("Part 1: {sand_unit_counter}");
        sand_unit_counter
    }
}
mod p2{
    use std::cmp::{max, min};

    use crate::p1::{get_wall_segments, Walls};
    pub fn main2(file_name:&str) -> i32{
        let sand_source = [500,0];
        let mut wall_segments = get_wall_segments(file_name);
        let [min_x,max_x,max_y] = wall_segments.iter().fold([0,0,0], | acc, [[lx,ly],[rx,ry]]|{
            let x_min = *min(lx,rx);
            let x_max  = *max(lx,rx);
            let y_max = *max(ly,ry);
            [min(acc[0], x_min), max(acc [1], x_max), max(acc[2], y_max)]
        });
        // Add a massive floor...
        wall_segments.push([[min_x*2,max_y+2],[max_x*2,max_y+2]]);
        wall_segments.sort_by(|a,b| a[0][1].cmp(&b[0][1]));

        let mut wall_collection = Walls{
            wall_list: wall_segments,
            sand_spawn: sand_source,
        };
        let mut sand_unit_counter = 0;
        'outer: loop{
            // Set the spawn to originate at the spawn point.
            let mut current_pos = wall_collection.sand_spawn;
            'inner: loop{
    

                // Check if (and where) the sand hits a block
                let sand_hits_block = match wall_collection.falling_sand_impact_coords(&current_pos){
                    Some(v) => v,
                    None => break 'outer // If it never hits a block, it is out of the map, so we are done.
                };
                current_pos = [sand_hits_block[0],sand_hits_block[1]-1];
                let lower_left_right: [[i32; 2]; 2] = [[current_pos[0]-1,current_pos[1]+1], [current_pos[0]+1,current_pos[1]+1]];
                // This vec is never greater than 2 in size.
                let available_space: Vec<[i32; 2]> = {
                    lower_left_right
                        .iter()
                        .filter_map(|pos| {
                            match wall_collection.overlaps_walls(pos){
                                true => None,
                                false => Some(*pos) 
                            }
                        }).collect::<Vec<[i32;2]>>()
                };
                if available_space.is_empty(){ // Sand settles.
                    wall_collection.add_wall([current_pos,[current_pos[0], current_pos[1]+1]]);
                    break 'inner; // Repeat with new sand block.
                }else{ // If not, drop the sand down to the side once.
                    current_pos = available_space[0];

                }
                // Then repeats
            }
            sand_unit_counter += 1; // One more sand block has settled.
            if current_pos == wall_collection.sand_spawn{
                break 'outer;
            }   
        }
        // println!("Part 1: {sand_unit_counter}");
        sand_unit_counter
    }
}


use crate::p1::main1;
use crate::p2::main2;
use std::time::Instant;
fn main() {
    // let file_name = r"src/dummy.txt";
    // let file_name = r"src/puzzle.txt";
    for (i, file_name) in [r"src/dummy.txt",r"src/puzzle.txt"].iter().enumerate(){
        let part = i+1;
        let start = Instant::now();
        let count = main1(file_name); // TODO: make an optimized version using a hashmap so we can look up columns instead of looking at all of them.
        // This would automatically make p2 better too...
        let end = start.elapsed();
        println!("\nPart {part} dummy: {count}\nRuntime: {end:?}\n");
        
        let start = Instant::now();
        let count = main2(file_name);
        let end = start.elapsed();
        println!("\nPart {part} main: {count}\nRuntime: {end:?}\n");
    }

}
