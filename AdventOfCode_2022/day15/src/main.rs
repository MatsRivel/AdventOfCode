mod p1{
    use std::fs::read_to_string;
    pub fn a_to_b_distance(a:[i32;2], b: [i32;2]) -> i32{
        // Manhatten distance
        (b[0] - a[0]).abs() + (b[1] - a[1]).abs()
    }
    fn get_sensor_data(file_name: &str, target_row: &i32) -> (Vec<[i32;3]>,i32){
        let string = read_to_string(file_name)
            .expect("We know the file is there; We put it there...");
        let lines = string
            .lines()
            .map(|line| {
                let segments = line.split(' ');
                let xys = segments
                    .filter_map(|s|{
                        if !s.starts_with('x') && !s.starts_with('y'){
                            return None;
                        }
                        let coord_string = {
                            let chars = s.chars();
                            let number:String = chars.filter(|&c| {
                                c == '-' || (c as i32 >= 48 && c as i32 <= 57)
                            }).collect::<String>();
                            number
                        };
                        let coord = match coord_string.parse::<i32>(){
                            Ok(v) => v,
                            Err(e) => {
                                println!("{coord_string:?} <= Failed to parse");
                                panic!("{e:?}")
                            }
                        };
                        Some(coord)

                    });
                xys.collect::<Vec<i32>>() 
            });
        // == Vec<[sx,sy,bx,by,distance]>;
        let mut beacon_at_target_row = 0;
        let sensors: Vec<[i32;3]>= lines.map(|line|{
                let arr:[i32;4] = line.try_into().expect("Failed to convert to vec to [i32;4]");
                let sensor = [arr[0],arr[1]];
                let beacon = [arr[2],arr[3]];
                let distance = a_to_b_distance(sensor, beacon);
                if &beacon[1] == target_row {
                    beacon_at_target_row += 1;
                }
                [sensor[0], sensor[1],distance]
            }).collect::<Vec<[i32;3]>>();
        (sensors, beacon_at_target_row)
        
        
        
    }

    pub fn main_1(file_name:&str, target_row:i32) -> i32{
        let (sensors, target_axis_beacons) = get_sensor_data(file_name, &target_row);

        // Subtract 1 for each beacon that lays directly on the target_row.
        let mut no_beacon_counter = -target_axis_beacons;
        // Go through each sensor and record the left and right edges of the line where its area intersects target_row.
        let mut target_axis_ranges = {
            sensors.iter().map(|[x,y, rad]|{
                let vertical_component = a_to_b_distance([*x,target_row], [*x,*y]);
                let horizontal_component = rad-vertical_component;
                [*x-horizontal_component, *x+horizontal_component]
            }).collect::<Vec<[i32;2]>>()
        };

        // Sort the ranges for easy merging
        target_axis_ranges.sort_by(|a,b|a[0].cmp(&b[0]));
        
        // Merge all overlapping ranges into one large one.
        let ranges = {
            target_axis_ranges.iter().fold(vec![], |mut acc, [ left_bound,  right_bound]|{
  
                // If no range, add one.
                if acc.is_empty(){ 
                    let mut new_section = [left_bound, right_bound];
                    new_section.sort();
                    acc.push(new_section);
                    return acc;
                }
                // println!("[{left_bound:?}, {right_bound:?}], {acc:?}");
                // If any range exists, try to merge the new one into it.
                let end = acc.len()-1;
                if left_bound <= acc[end][1] && (right_bound > acc[end][1]){
                    // New range overlaps on old ranges right
                    acc[end][1] = right_bound;
                }else if left_bound < acc[end][0] && right_bound <= acc[end][1]{
                    // New range overlaps on old ranges left.
                    acc[end][0] = left_bound;
                }else if left_bound >= acc[end][0] && right_bound <= acc[end][1]{
                    // New range exists withing old range.

                }else if left_bound < acc[end][0] && right_bound > acc[end][1]{
                    // New range overlaps old range on both sides.
                    acc[end][1] = right_bound;
                    acc[end][0] = left_bound;
                }else if left_bound > acc[end][1] || right_bound < acc[end][0]{
                    // If new range exists separate on either side of old range, add it to the list.
                    // Note: As list is sorted, right_bound < acc[end][0] will allways be false.
                    // Below we check whether the last list brushes up against the new list. If so, merge them.
                    let mut new_section = [left_bound, right_bound];
                    new_section.sort();
                    if acc[end][1]+1 == *new_section[0]{
                        acc[end][1] = new_section[1];
                    }else{
                        acc.push(new_section);
                    }
                }
                acc
            })
        };
        // Add the width of each range to the number of checked points.
        for range in ranges{
            no_beacon_counter += range[1]-range[0]+1;
        }
        no_beacon_counter
        
    }
}
mod p2{
    use std::{fs::read_to_string, cmp::{max, min}};
    fn get_file_content(file_name: &str)-> String{
        let string:String = read_to_string(file_name)
            .expect("We know the file is there; We put it there...");
        string
    }
    fn get_sensor_data_lines<'a>(file_content:&'a String) -> impl Iterator<Item = Vec<i32>>+'a{
        let lines = file_content
            .lines()
            .map(|line:&str| {
                let segments = line.split(' ');
                let xys = segments
                    .filter_map(|s:&str|{
                        if !s.starts_with('x') && !s.starts_with('y'){
                            return None;
                        }
                        let coord_string = {
                            let chars = s.chars();
                            let number:String = chars.filter(|&c| {
                                c == '-' || (c as i32 >= 48 && c as i32 <= 57)
                            }).collect::<String>();
                            number
                        };
                        let coord = match coord_string.parse::<i32>(){
                            Ok(v) => v,
                            Err(e) => {
                                println!("{coord_string:?} <= Failed to parse");
                                panic!("{e:?}")
                            }
                        };
                        Some(coord)

                    });
                xys.collect::<Vec<i32>>() 
            });
        lines
    }
    fn interperet_sensor_lines<'a>(lines:impl Iterator<Item = Vec<i32>>+'a)->Vec<[i32;3]>{
        // == Vec<[sx,sy,bx,by,distance]>;
        let sensors: Vec<[i32;3]>= lines.map(|line|{
                let arr:[i32;4] = line.try_into().expect("Failed to convert to vec to [i32;4]");
                let sensor = [arr[0],arr[1]];
                let beacon = [arr[2],arr[3]];
                let distance = a_to_b_distance(sensor, beacon);
                [sensor[0], sensor[1],distance]
            }).collect::<Vec<[i32;3]>>();
        sensors
    }
        
    use crate::p1::a_to_b_distance;
    pub fn main_2(file_name:&str, max_idx:i32) -> f64{
        // Sensors only lock on to closest beacon.
        // To know that there is a spot that we dont cover, we must know that there are spots we cover on either side of it.
        // A spot on the edge might as well be infinetly far away.
        let file_content = get_file_content(file_name);
        let lines = get_sensor_data_lines(&file_content);
        let sensors= interperet_sensor_lines(lines);
        let [mut y_min, mut y_max] = sensors.iter().fold([i32::MAX,i32::MIN], |mut acc, [_,y,_]|{
            if acc[0] > *y{
                acc[0] = *y;
            }else if acc[1] < *y{
                acc[1] = *y
            }
            acc
        });
        y_max = min(y_max,max_idx);
        y_min = max(0,y_min);
        println!("Running part2: \n[{y_min},{y_max}]\nHave to check {} rows...", y_max-y_min+1);
        let middle = (y_max-y_min)/2 + y_min;
        println!("Middle: {middle}");
        let mut diff_counter = 0;
        'outer_loop: loop{
            'inner_loop: for i in (-1..=1).step_by(2){
                let target_row = middle+(diff_counter*i);
                if target_row < y_min{
                    continue 'inner_loop;
                }else if target_row > y_max{
                    break 'outer_loop;
                }

                // Subtract 1 for each beacon that lays directly on the target_row.
                // Go through each sensor and record the left and right edges of the line where its area intersects target_row.
                let mut target_axis_ranges = {
                    sensors.iter().map(|[x,y, rad]|{
                        let vertical_component = a_to_b_distance([*x,target_row], [*x,*y]);
                        let horizontal_component = rad-vertical_component;
                        [*x-horizontal_component, *x+horizontal_component]
                    }).collect::<Vec<[i32;2]>>()
                };

                // Sort the ranges for easy merging
                target_axis_ranges.sort_by(|a,b|a[0].cmp(&b[0]));
                
                // Merge all overlapping ranges into one large one.
                let ranges = {
                    target_axis_ranges.iter().fold(vec![], |mut acc, [mut left_bound, mut right_bound]|{
                        left_bound = min(max(left_bound,0),max_idx);
                        right_bound = min(max(right_bound,0),max_idx);
        
                        // If no range, add one.
                        if acc.is_empty(){ 
                            let mut new_section = [left_bound, right_bound];
                            new_section.sort();
                            acc.push(new_section);
                            return acc;
                        }
                        // println!("[{left_bound:?}, {right_bound:?}], {acc:?}");
                        // If any range exists, try to merge the new one into it.
                        let end = acc.len()-1;
                        if left_bound <= acc[end][1] && (right_bound > acc[end][1]){
                            // New range overlaps on old ranges right
                            acc[end][1] = right_bound;
                        }else if left_bound < acc[end][0] && right_bound <= acc[end][1]{
                            // New range overlaps on old ranges left.
                            acc[end][0] = left_bound;
                        }else if left_bound >= acc[end][0] && right_bound <= acc[end][1]{
                            // New range exists withing old range.

                        }else if left_bound < acc[end][0] && right_bound > acc[end][1]{
                            // New range overlaps old range on both sides.
                            acc[end][1] = right_bound;
                            acc[end][0] = left_bound;
                        }else if left_bound > acc[end][1] || right_bound < acc[end][0]{
                            // If new range exists separate on either side of old range, add it to the list.
                            // Note: As list is sorted, right_bound < acc[end][0] will allways be false.
                            // Below we check whether the last list brushes up against the new list. If so, merge them.
                            let mut new_section = [left_bound, right_bound];
                            new_section.sort();
                            if acc[end][1]+1 == new_section[0]{
                                acc[end][1] = new_section[1];
                            }else{
                                acc.push(new_section);
                            }
                        }
                        acc
                    })
                };
                if ranges.len() == 1{ // Continuous coverage -> No beacon.
                    continue;
                }
                let horizontal_position = ranges[1][0]-1;
                println!("Horizontal position: {horizontal_position}");
                return (horizontal_position as f64) *4_000_000f64 + (target_row as f64);
            }
            diff_counter += 1;
        }
        -1f64

    }
}

use p1::main_1;
use p2::main_2;
use std::time::Instant;
fn main() {
    // let (file_name, target_row, max_idx) = (r"src\dummy_input.txt", 10, 20); // Part 1: 24, Part 2: 56000011
    let (file_name, target_row, max_idx) = (r"src\puzzle_input.txt", 2_000_000, 4_000_000);// Part 1: 4873353
    let start = Instant::now();
    let count = main_1(file_name, target_row);
    let end = start.elapsed();
    println!("\nPart 1: {count}\nRuntime: {end:?}");

    let start = Instant::now();
    let count = main_2(file_name, max_idx);
    let end = start.elapsed();
    println!("\nPart 2: {count}\nRuntime: {end:?}");

}
