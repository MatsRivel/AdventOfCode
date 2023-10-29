mod p1{
    use core::panic;
    use std::fs::read_to_string;
    use std::collections::{VecDeque, HashMap};
    use std::thread::current;
    fn two_char_to_int(name:&str) -> i32 { // Only valid if name has exactly 2 characters!
        let numbers:[i32;2] = name.chars().map(|c| c as i32 -65).collect::<Vec<i32>>().try_into().expect(format!("We know name will have only 2 characters: \"{name}\"").as_str());
        if numbers[0] == numbers[1]{
            // println!("{name} => {numbers:?} => {}",numbers[0]);
            return numbers[0];
        }else{
            println!("{name} => {numbers:?} => {}",numbers[0]*26+numbers[1]);
            numbers[0]*26 + numbers[1]
        }
    }
    fn get_data_string(file_name:&str) -> String{
        let string = read_to_string(file_name).expect("We know this file exists!");
        string
    }
    fn get_data<'a>(data_string:&'a str)->impl Iterator<Item = impl Iterator<Item=i32>+'a>+'a{
        let lines = data_string.lines().map(|line|{
            let sections = line
                .split(" ")
                .filter(|section|{
                    let mut result = true;
                    if (!section.starts_with("rate=") && section.len() >3) || section == &"to" || section == &"has"{
                        result = false;
                    }
                    result
                }).map(|section:&str|{
                    let output:i32;
                    if let Some(number) = section.strip_prefix("rate="){
                        let number = number.strip_suffix(";").expect("\"rate=*;\" is a fixed pattern");
                        output = number.parse::<i32>().expect(format!("We know * is a valid number: {number:?}").as_str());
                    }else{
                        let name = match section.strip_suffix(',') {
                            None => section,
                            Some(v) => v
                        };
                        output = two_char_to_int(name);
                    }
                    output
                });
            sections
            });
        lines
    }

    #[derive(Debug,Clone)]
    struct Node{
        name:usize,
        flow_rate:Option<i32>,
        value:Option<i32>,
        distance:Option<i32>,
        neighbours:Vec<usize>
    }
    impl Node{
        fn new(input_iterator: &mut impl Iterator<Item=i32>)->Self{
            let name = input_iterator.next().expect("Iterator should have 2+ elements") as usize;
            let flow_rate = match input_iterator.next(){
                Some(v) => {
                    if v != 0{
                        Some(v)
                    }else{
                        None
                    }
                },
                None => None
            };
            let value = None;
            let distance = None;
            let neighbours = input_iterator.map(|element| element as usize).collect::<Vec<usize>>();
            Self{name, flow_rate, value, distance, neighbours}
        }
    }
    
    fn breadth_first_update(nodes:&mut HashMap<usize,Node>, start_node:usize, steps_left:i32){
        let mut visited: HashMap<usize,bool> = HashMap::new();
        let mut q:VecDeque<usize> = VecDeque::new();
        let mut new_q:VecDeque<usize> = VecDeque::new();
        let mut node_depth_q:VecDeque<usize>;
        let mut depth = 1;
        { // Set distance and value of the starting node.
            nodes.get_mut(&start_node).expect("We know it is here...").distance = Some(depth);
            let flow_rate = match nodes.get(&start_node).unwrap().flow_rate{
                Some(v) => v,
                None => 0
            };
            let node_distance = nodes.get(&start_node).unwrap().distance.expect("We just set it...");
            nodes.get_mut(&start_node).expect("We know it is here...").value = Some((steps_left - node_distance)*flow_rate);
            q.push_back(start_node);
        }
        depth += 1;
        visited.insert(start_node,true);
        while let Some(current_node) = q.pop_front(){
            node_depth_q = VecDeque::new();
            for &neighbour in nodes.get(&current_node).unwrap().neighbours.iter(){
                if !visited.get(&neighbour).or(Some(&false)).expect("We know it is here..."){
                    node_depth_q.push_back(neighbour);
                    new_q.push_back(neighbour);
                    visited.insert(neighbour, true);
                }
            }
            // Update depth of neighbours. Could not do it in loop due borrow + mut borrow ...
            'neighbour_loop: while let Some(visited_neighbour) = node_depth_q.pop_front(){
                if nodes.get(&visited_neighbour).unwrap().flow_rate.is_none(){
                    continue 'neighbour_loop;
                }
                nodes.get_mut(&visited_neighbour).expect("We know it is here...").distance = Some(depth);
                let flow_rate = match nodes.get(&visited_neighbour).unwrap().flow_rate{
                    Some(v) => v,
                    None => 0
                };
                let node_distance = nodes.get(&visited_neighbour).unwrap().distance.expect("We just set it...");
                // println!("{} | Fr: {flow_rate}, D: {node_distance}, {steps_left}-D = {}, {}*{} -> v: {}",(visited_neighbour+ 65) as u8 as char , steps_left-node_distance, steps_left-node_distance,flow_rate, (steps_left-node_distance)*flow_rate);
                nodes.get_mut(&visited_neighbour).expect("We know it is here...").value = Some((steps_left - node_distance)*flow_rate); // Total value a node can generate at the current time step.
            }
            // When current layer is done, increment depth and add neighbours to the q.
            if q.is_empty(){
                // print!("q empty: ");
                // for element in new_q.iter(){
                //     print!("{}, ", (nodes[*element].name + 65) as u8 as char)
                // }
                // println!(" | Depth: {}", depth);
                depth += 1;
                q = new_q;
                new_q = VecDeque::new();
            }

        }
    }

    fn get_best_node(nodes:&HashMap<usize,Node>, steps_left:i32)->usize{
        let (best_node, _best_value) = nodes.iter().fold((0usize,0i32),|acc, (idx, node)|{
            if node.value.is_none() || node.value.expect("Value checked") <= acc.1{
                acc
            }else{
                (*idx,node.value.expect("Value checked"))
            }
        });
        best_node
    }

    pub fn main_1(file_name:&str)->Option<i32>{
        // Goal: Given thirty (30) min, and 1 action (open valve or move) takes one (1) min,
        // And each node has a assosicated "score".
        // Find the path (and actions) that lead to the highest "score".
        // Note: The value of a given node is (time_left - time_to_activate_node)*node_value.
        let data_string = get_data_string(file_name);
        let data_iterator = get_data(&data_string);
        let nodes_base:HashMap<usize,Node> = data_iterator
            .map(|mut line| {
                let node = Node::new(&mut line);
                (node.name, node)
            }).collect::<HashMap<usize,Node>>();

        // We clone the base nodes so that we can try multiple starting nodes consecutively.
        let mut score_list: Vec<i32> = Vec::new();
        for node_name in nodes_base.keys(){
            let mut current_node = two_char_to_int("AA") as usize;
            println!("\n\n########## {} ##########", (node_name+65) as u8 as char);
            let mut nodes = nodes_base.copy();

            let mut total_score = 0;
            let mut steps = 30;

            
            { // Doing one lap from our start-node to our first step node:
                breadth_first_update(&mut nodes, current_node, steps);
                let best_node = *node_name;
                // println!("{:?}",nodes.get(&best_node).unwrap());
                match nodes.get(&best_node){
                    Some(n) =>{ 
                        if n.flow_rate.is_none(){
                            (); // Do not break on preliminary check// Ran out of nodes before we ran out of moves!
                        }
                        match n.distance{
                            Some(v) => {
                                if v > steps{
                                    nodes.get_mut(&best_node).expect("We know it is here...").flow_rate = None;
                                    nodes.get_mut(&best_node).expect("We know it is here...").value = None;
                                    nodes.get_mut(&best_node).expect("We know it is here...").distance = None;
                                    continue;
                                }
                            },
                            None => (), // Best node available has no values. (Aka no nodes available)
                        }
                    },
                    None => panic!("We got a node that we do not have..?")
                }
                steps -= nodes.get(&best_node).expect("We know it is here...").distance.expect("All nodes should be in order by now.");
                total_score += nodes.get(&best_node).expect("We know it is here...").value.expect("All nodes should be in order by now.");
                println!("Steps: {steps}, Best node: {} ({}), value: {:?}, flow: {:?}",
                    (nodes.get(&best_node).expect("We know it is here...").name + 65) as u8 as char,
                    nodes.get(&best_node).expect("We know it is here...").name,
                    nodes.get(&best_node).expect("We know it is here...").value,
                    nodes.get(&best_node).expect("We know it is here...").flow_rate);
                    
                nodes.get_mut(&best_node).expect("We know it is here...").flow_rate = None;
                nodes.get_mut(&best_node).expect("We know it is here...").value = None;
                current_node = best_node;
            }

            // Then do it normally:
            while steps > 0{
                breadth_first_update(&mut nodes, current_node, steps);
                let best_node = get_best_node(&nodes, steps);
                println!("{:?}",nodes.get(&best_node).unwrap());
                match nodes.get(&best_node){
                    Some(n) =>{ 
                        if n.flow_rate.is_none(){
                            break;// Ran out of nodes before we ran out of moves!
                        }
                        match n.distance{
                            Some(v) => if v > steps{
                                nodes.get_mut(&best_node).expect("We know it is here...").flow_rate = None;
                                nodes.get_mut(&best_node).expect("We know it is here...").value = None;
                                nodes.get_mut(&best_node).expect("We know it is here...").distance = None;
                                continue;
                            },
                            None => break, // Best node available has no values. (Aka no nodes available)
                        }
                    },
                    None => panic!("We got a node that we do not have..?")
                }
                steps -= nodes.get_mut(&best_node).expect("We know it is here...").distance.expect("All nodes should be in order by now.");
                total_score += nodes.get_mut(&best_node).expect("We know it is here...").value.expect("All nodes should be in order by now.");
                println!("Steps: {steps}, Best node: {} ({}), value: {:?}, flow: {:?}",
                    (nodes.get(&best_node).expect("We know it is here...").name + 65) as u8 as char,
                    nodes.get(&best_node).expect("We know it is here...").name,
                    nodes.get(&best_node).expect("We know it is here...").value,
                    nodes.get(&best_node).expect("We know it is here...").flow_rate);
                    
                nodes.get_mut(&best_node).expect("We know it is here...").flow_rate = None;
                nodes.get_mut(&best_node).expect("We know it is here...").value = None;
                current_node = best_node;
            }

            println!("{} -> {total_score}",(node_name+65) as u8 as char);
            score_list.push(total_score);
            // println!("Remaining steps: {steps}");

        }
        let best_score = score_list.iter().fold(0,|acc,v|{
            if *v > acc{
                *v
            }else{
                acc
            }
        });
        Some(best_score)
    }
}

mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i32>{
      None
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
    println!("Part 1: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        let correct_value = 1651;
        assert_eq!(count.expect("We do not mind panic here."),correct_value,"{}", {
            if count.expect("We do not mind panic here.") < correct_value{
                "Our value is too low"
            }else if count.expect("We do not mind panic here.") > correct_value{
                "Out value is too high!"
            }else{
                ""
            }
        });
    }else if file_name== r"src\puzzle_input.txt"{
        assert!(count.expect("We do not mind panic here.") < 2001, "Our value is too high!")
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}