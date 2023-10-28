
mod p1{
    use core::{num, panic};
    use std::cmp::max;
    use std::fs::read_to_string;
    use std::collections::{HashMap, VecDeque};

    use crate::MAX_RUNTIME;


    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub enum Material{
        Ore,
        Clay,
        Obsidian,
        Geode,
        Air
    }

    fn can_afford(costs: &[[i32;3];4],robot:&Material,stores:&[i32;4]) -> bool{
        if robot == &Material::Air{
            return true;
        }
        let exctracted_costs = costs[material_to_cost_idx(robot)];
        // println!("Robo costs: {exctracted_costs:?}");
        stores.iter().zip( exctracted_costs.iter() ).fold(true,|acc,(store,cost)| acc && store >= cost)
    }        
    pub fn process_line(line:&str)-> (i32, [[i32;3];4], [i32;4]){
        let mut number = 0;
        let mut costs: [[i32;3];4] = get_real_materials().iter().map(|_| [0,0,0]).collect::<Vec<[i32;3]>>().try_into().unwrap();
        let mut robots: [i32;4] = get_real_materials().iter().map(|_| 0).collect::<Vec<i32>>().try_into().unwrap();
        let mut current_robot = Material::Ore;
        let sections = line.split(' ').collect::<Vec<&str>>();
        for (idx, &section) in sections.iter().enumerate().skip(1){
            if sections[idx-1].starts_with("Blueprint"){
                number = section.strip_suffix(':').unwrap().parse::<i32>().unwrap();
                continue;
            }
            if sections[idx-1].starts_with("Each"){                             // If the last section was "each" and the current is a material, assign the current type of robot to be that material.
                if let Some(r) = get_robot_from_str(section){
                    current_robot = r;
                }
                continue;
            }
            if let Ok(price) = sections[idx-1].parse::<i32>(){              // If the last section was a number
                if let Some(material) = get_robot_from_str(section){    // And this section has a material
                    let mat_idx = material_to_cost_idx(&material);          // Assign the current robots cost of that material to that number
                    let rob_idx = material_to_cost_idx(&current_robot);
                    costs[rob_idx][mat_idx] = price;

                }
            }
        }
        robots[0] = 1;
        (number, costs, robots)
    }
    fn get_construction_materials() -> [Material;3]{
        [Material::Ore, Material::Clay, Material::Obsidian]
    }
    fn get_real_materials() -> [Material;4]{
        [Material::Ore, Material::Clay, Material::Obsidian, Material::Geode]
    }
    fn get_robot_from_str(s:&str) -> Option<Material>{
        let s = match s.strip_suffix("."){
            None => s,
            Some(v) => v
        };
        match s{
            "ore" => Some(Material::Ore),
            "clay" => Some(Material::Clay),
            "obsidian" => Some(Material::Obsidian),
            "geode" => Some(Material::Geode),
            _ => None

        }
    }
    fn material_to_cost_idx(material:&Material) -> usize{
        match material{
            Material::Ore => 0,
            Material::Clay => 1,
            Material::Obsidian => 2,
            Material::Geode => 3,
            _ => panic!("Air should not be indexed!"),
        }
    }
    fn get_bottle_neck_costs(costs: &[[i32;3];4]) -> [i32;3]{
        costs.iter().fold([0i32;3], |mut acc, cost|{
            for i in 0..3{
                acc[i] = max(acc[i], cost[i]);
            }
            acc
        })
    }
    fn depth_first_recursice(costs: &[[i32;3];4], robots:&[i32;4], stores:&[i32;4], runtime:i32, last_action:&Material, memory:&mut HashMap<[i32;4], [i32;2]>)->Option<(i32,Material,VecDeque<Material>)>{
        if runtime == 1{
            if (last_action == &Material::Air || last_action == &Material::Geode) && stores[3] > 0{
                // println!("- - - - -| ROOT: Robots: {robots:?} (counts: {})",stores[3]);
                return Some((stores[3],Material::Air, VecDeque::<Material>::new()));
            }
            return None;
        }


        // Update the amount of stuff in storage, created last action.
        let current_stores:[i32;4] = stores.iter().enumerate().map(|(idx,n_materials)| n_materials+robots[idx] ).collect::<Vec<i32>>().try_into().unwrap();
        // Update the robot count from last action:
        let current_robots = {
            let mut robs = robots.clone();
            match last_action{
                Material::Air => (),
                v => {
                    let idx = material_to_cost_idx(&v);
                    robs[idx] += 1;
                }
            }
            robs
        };
        
        // If we've seen this state before, but earlier, don't go further.
        if let Some([best_steps_left, best_geodes]) = memory.get(&current_robots){
            // println!("### {current_robots:?} (t: {runtime}) (last: {last_action:?}) ###");
            if best_steps_left > &runtime  && last_action != &Material::Air  { // Added a buffer so we don't give up before we've seen anything.
                // println!("X");
                return None;
            }
        }
        if last_action != &Material::Air{
            // If the current is better OR its never seen, add it: (ignore Air, as there will be no change in bots)
            memory.insert(current_robots, [runtime,stores[3]]);
        }
        

        let bottle_neck_costs = get_bottle_neck_costs(costs);
        let mut is_bottle_neck_vec = (0..3).map(|i| robots[i] < bottle_neck_costs[i]).collect::<Vec<bool>>();
        is_bottle_neck_vec.push(false); // Geode is never bottle neck!
        let is_bottle_neck:[bool;4] = is_bottle_neck_vec.try_into().unwrap();

        // If we have enough of a material to make 1geode bot each round, stop making that material.
        // Also only consider materials we can afford.
        // This should drastically reduce amount of paths.
        // println!("\nStores: {current_stores:?}");
        let mut materials_to_consider = {[Material::Geode, Material::Obsidian, Material::Clay, Material::Ore]
            .iter()
            .enumerate()
            .filter(|(idx, m)| {
                let need_more = match m{
                    Material::Air | Material::Geode => true,
                    _ =>{
                        let adj_idx = 3-idx; // adjust to ignore Air
                        is_bottle_neck[adj_idx] // If the value is not a bottle neck, ignore it. If it is, keep making it.
                    } 
                };
                // If we cant afford the robot, then just move on.
                let can_afford = can_afford(costs, m, &current_stores);
                // println!("{m:?}: Need more: {need_more}, Can afford: {can_afford}");
                need_more && can_afford
            }).map(|(i,m)|m.clone())
            .collect::<Vec<Material>>()
        };

        // let mut materials_to_consider = vec![Material::Geode, Material::Obsidian, Material::Clay, Material::Ore, Material::Air];
        
        let mut best_count = None;
        let mut best_q = VecDeque::<Material>::new();
        materials_to_consider.push(Material::Air);

        for material in materials_to_consider.iter(){
            if !can_afford(costs, &material, &current_stores){
                continue;
            }
            let is_geode = material == &Material::Geode;
            let inner_stores = match material{
                Material::Air => current_stores.clone(),
                v => {
                    let rob_idx = material_to_cost_idx(v);
                    [current_stores[0]-costs[rob_idx][0],
                    current_stores[1]-costs[rob_idx][1],
                    current_stores[2]-costs[rob_idx][2],
                    current_stores[3]]}
            };
            if let Some((geodes_found, leaf_action, q)) = depth_first_recursice(costs, &current_robots, &inner_stores, runtime-1, material, memory){
                // Last material has to be air, as nothing else matters after that.
                if leaf_action==Material::Air && (best_count == None || geodes_found > best_count.unwrap()){
                        best_count = Some(geodes_found);
                        best_q = q;
                }
                // If we can affort geode (and it lead to something), we make geode, and nothing else.
                if is_geode{
                    // println!("g");
                    break;
                }
            }
        }
        if let Some(count) = best_count{
            best_q.push_back(last_action.clone());
            return Some((count,Material::Air,best_q));
        }
        None 
    }

    pub fn depth_first_initializer( costs: &[[i32;3];4], robots:&[i32;4],runtime:i32)->Option<(i32,Material, VecDeque<Material>)>{
        let mut memory:HashMap<[i32;4], [i32;2]> = HashMap::new();
        // Hashmap<Robot_counts, steps left>
        depth_first_recursice(costs, robots,&[0,0,0,0],runtime, &Material::Air, &mut memory)
    }
    fn check_solution(costs: &[[i32;3];4], mut path: VecDeque<Material>) -> i32 {
        let mut robs = [1,0,0,0];
        let mut stores = [0,0,0,0];
        let mut steps = 1;
        println!("");
        while let Some(p) = path.pop_back() {
            steps += 1;
            print!("{p:?} -> ");

            // Add produced stuff to the stores.
            stores.iter_mut().zip(robs.iter()).for_each(|(mut s,r)| *s += r );
                        // Remove materials needed for production:
            // Increase robot count.
            match p{
                Material::Air => (),
                v => {
                    let mat_idx = material_to_cost_idx(&v);
                    stores.iter_mut().zip(costs[mat_idx].iter()).for_each(|(s,r)| *s += r );
                    robs[mat_idx] += 1;
                },
            }

        }
        if steps != MAX_RUNTIME {
            println!("\nOoops, too many/few steps! {steps}");
        }
        return stores[3];

    }

    pub fn main_1(file_name:&str)->Option<i32>{
        let string = read_to_string(file_name).unwrap();
        let mut total_quality = 0;
        let max_runtime = super::MAX_RUNTIME;
        for (num, costs, robots) in string.lines().map(|line| process_line(line)){
            // println!("Blueprint: {num}, Costs: {costs:?}, Robots: {robots:?}");
            if let Some((count,_, path)) = depth_first_initializer(&costs, &robots, max_runtime){
                total_quality += count*num;
                let checked_count = check_solution(&costs, path);
                assert_eq!(count,checked_count);
            }
        }
        Some(total_quality)
    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn need_more_test(){
        let string = read_to_string(r"src\trivial_case.txt").unwrap();
        let stores = [1,0,0,0]; // We have one ore.
        let robots = [1,0,0,0]; // We have only one ore robot.
        if let Some((_, costs, _)) = string.lines().map(|line| process_line(line)).next(){
            let robot = Material::Ore;
            let ans = can_afford(&costs, &robot, &stores);
            assert!(ans,"We should be able to affort the ore bot with 1 ores! It only costs 1!");

            let bottle_neck_costs = get_bottle_neck_costs(&costs);
            let mut is_bottle_neck_vec = (0..3).map(|i| robots[i] < bottle_neck_costs[i]).collect::<Vec<bool>>();
            is_bottle_neck_vec.push(false); // Geode is never bottle neck!
            let is_bottle_neck:[bool;4] = is_bottle_neck_vec.try_into().unwrap();

            println!();
            assert!(!is_bottle_neck[0], "We need need more ore, but we have enough clay robots!");
            assert!(is_bottle_neck[1], "We need more clay, and we have no clay robots, so we need clay robots!");
            assert!(is_bottle_neck[2], "We need more obsidian, and we have no obsidian robots, so we need obsidian robots!");

        }
	}
	#[test]
	fn cant_afford_test(){
        let string = read_to_string(r"src\trivial_case.txt").unwrap();
        let stores = [0,0,0,0];
        if let Some((_, costs, _)) = string.lines().map(|line| process_line(line)).next(){
            let robot = Material::Ore;
            let ans = can_afford(&costs, &robot, &stores);
            assert!(!ans,"We should'nt be able to affort the ore bot with 0 ores! It costs 1!");
        }
	}
    #[test]
    fn can_afford_test(){
        let string = read_to_string(r"src\trivial_case.txt").unwrap();
        let stores = [1,0,0,0];
        if let Some((_, costs, _)) = string.lines().map(|line| process_line(line)).next(){
            let robot = Material::Ore;
            let ans = can_afford(&costs, &robot, &stores);
            assert!(ans,"We should be able to affort the ore bot with 1 ores!");
        }
    }
}

}
mod p2{
    use std::fs::read_to_string;

    use crate::p1::{depth_first_initializer, process_line};
    pub fn main_2(file_name:&str)->Option<i32>{
        let string = read_to_string(file_name).unwrap();
        let max_runtime = super::MAX_RUNTIME2;
        let mut best_vec: Vec<i32> = Vec::with_capacity(3);
        for (num, costs, robots) in string.lines().map(|line| process_line(line)){
            // println!("Blueprint: {num}, Costs: {costs:?}, Robots: {robots:?}");
            if let Some((count,_, path)) = depth_first_initializer(&costs, &robots, max_runtime){
                println!("\nBlueprint: {num}, n_geodes: {count}");
                // let checked_count = check_solution(&costs, path);
                // assert_eq!(count,checked_count);
                best_vec.push(count);
            }else{
                println!("\nBlueprint: {num} No Answer!");
            }
            if num >= 3{
                println!(" - {num} -");
                break;
            }
        }
        Some(best_vec.iter().fold(1, |acc,v| acc*v))
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
const MAX_RUNTIME:i32 = 25;
const MAX_RUNTIME2:i32 = 33;
fn main() {
    let file_name = r"src\trivial_case.txt";
    let file_name = r"src\small_input.txt";
    let file_name = r"src\dummy_input.txt";
    let file_name= r"src\puzzle_input.txt";
    
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt" && MAX_RUNTIME == 25{
        assert!(count.unwrap() == 33)
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    assert!(count.unwrap() > 124 || file_name != r"src\puzzle_input.txt","\n ----- Answer is too low! -----");
    assert!(count.unwrap() < 194432 || file_name != r"src\puzzle_input.txt","\n ----- Answer is too high! -----");
}