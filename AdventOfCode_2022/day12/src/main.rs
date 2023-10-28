mod p1{
    use std::{fs::read_to_string, collections::HashMap};
    use priority_queue::DoublePriorityQueue;
    pub enum NodeType{
        Start, 
        End,
        Other
    }

    pub struct Node{
        pub node_type: NodeType,
        pub height: u8,
        pub coords: [usize;2],
        pub top: Option<[usize;2]>,
        pub left: Option<[usize;2]>,
        pub right: Option<[usize;2]>,
        pub bottom: Option<[usize;2]>,
    }
    impl Node{
        fn new(node_type:NodeType, height:u8, coords:[usize;2]) -> Self{
            Self{node_type, height, coords, top:None, left:None, right:None, bottom:None,}
        }
        pub fn update(&mut self, top: Option<[usize;2]>, left: Option<[usize;2]>, bottom: Option<[usize;2]>, right: Option<[usize;2]>) {
            self.top = top;
            self.left = left;
            self.right = right;
            self.bottom = bottom;
        }  
        pub fn get_neightbours(&self) -> Vec::<[usize;2]>{
            let mut output = Vec::<[usize;2]>::with_capacity(4);
            if let Some(coord) = self.top{
                output.push(coord);
            }
            if let Some(coord) = self.left{
                output.push(coord);
            }
            if let Some(coord) = self.right{
                output.push(coord);
            }
            if let Some(coord) = self.bottom{
                output.push(coord);
            }
            output
        }
    }
    fn is_height_adjacent(node_a:&Node, node_b:&Node) -> bool{
        (node_b.height as i32 - node_a.height as i32)<=1
    }
    pub fn get_nodes(file_name:&str)->Vec<Vec<Node>>{
        read_to_string(file_name)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(y_idx,line)|{
                let y_idx_copy = y_idx;
                let node = line.chars().enumerate().map(move |(x_idx, c)|{
                    let coords = [x_idx, y_idx_copy];
                    let (height, node_type) = match c{
                        'S' => { 
                            ('a' as u8 - 97, NodeType::Start)
                        }
                        'E' => { 
                            ('z' as u8 - 97,  NodeType::End)
                        },
                        v => (v as u8 - 97,  NodeType::Other)
                    };
                    // println!("{coords:?}");
                    Node::new(node_type, height,coords)
                }).collect::<Vec<Node>>();
                node
            }).collect::<Vec<Vec<Node>>>()
    }
    fn update_nodes(nodes:&mut Vec<Vec<Node>>) ->([usize;2], [usize;2]){
        let mut start_coord:[usize;2] = [0,0];
        let mut end_coord:[usize;2] = [1,1];
        for y in 0..nodes.len(){
            for x in 0..nodes[0].len(){
                let mut top:Option<[usize;2]> = None;
                let mut left:Option<[usize;2]> = None;
                let mut right:Option<[usize;2]> = None;
                let mut bottom:Option<[usize;2]> = None;
                match nodes[y][x].node_type{
                    NodeType::Start => start_coord = nodes[y][x].coords,
                    NodeType::End => end_coord = nodes[y][x].coords,
                    NodeType::Other => ()
                }
                if y > 0 && is_height_adjacent(&nodes[y][x], &nodes[y-1][x]){
                    top = Some([x, y-1])
                }
                if y < nodes.len()-1 && is_height_adjacent(&nodes[y][x], &nodes[y+1][x]){
                    bottom = Some([x, y+1])
                }
                if x > 0 && is_height_adjacent(&nodes[y][x], &nodes[y][x-1]){
                    left = Some([x-1, y])
                }
                if x < nodes[0].len()-1 && is_height_adjacent(&nodes[y][x], &nodes[y][x+1]){
                    right = Some([x+1, y])
                }
                nodes[y][x].update(top, left, bottom, right);
            }
        }
        (start_coord, end_coord)
    } 
    pub fn reconstruct_path(came_from:HashMap<[usize;2],[usize;2]>, current_in:[usize;2]) -> Vec<[usize;2]>{
        let mut current = current_in.clone();
        let mut path = vec![current];
        while came_from.contains_key(&current){
            current = *came_from.get(&current).expect("If current not in keys, this loop would have broken by now");
            path.push(current);
        }
        path
    }
    // fn h(start:[usize;2], end:[usize;2]) -> f32{
    //     let x_dir = end[0] as i32 - start[0] as i32;
    //     let y_dir = end[1] as i32 - start[1] as i32;
    //     (x_dir+y_dir) as f32
    // }

    fn create_functions(end:[usize;2]) -> (impl Fn([usize;2], [usize;2]) -> f32, impl Fn([usize;2]) -> f32) {
        let d = move |from: [usize;2], to: [usize;2]| -> f32 {
            ((from[0] as f32 - to[0] as f32).powf(2.0) + (from[1] as f32 - to[1] as f32 ).powf(2.0)).sqrt()
            //  ((from[0] as i32 -to[0] as i32).abs() + (from[1] as i32 + to[1] as i32).abs()) as f32 
        };
        let h = move |from: [usize;2]| -> f32 { d(from,end) };
        (d, h)
    }
    pub fn a_star(start:[usize;2], end:[usize;2],nodes: Vec<Vec<Node>>) -> Option<Vec<[usize;2]>>{
        let default_gscore = std::f32::INFINITY;
        let mut open_set: DoublePriorityQueue<[usize;2], i32> = DoublePriorityQueue::new();
        let mut came_from: HashMap<[usize;2],[usize;2]> = HashMap::new();
        let mut gscore: HashMap<[usize;2],f32> = HashMap::new();
        let mut fscore: HashMap<[usize;2],f32> = HashMap::new(); // fscore[n] == gscore[n] + h(n)
        let (distance_to_neighbour,distance_to_end) = create_functions(end);
        // h -> Distance from node to end.
        // d -> Distance from node to other_node
        open_set.push(start, default_gscore as i32);
        gscore.insert(start, 0f32);
        fscore.insert(start, distance_to_end(start));
        // println!("Dims: [{},{}]",nodes.len(), nodes[0].len());
        // println!("End = {end:?}");
        while !open_set.is_empty(){
            let (current, _current_value) = open_set.pop_min().expect("Can not fail, as empty set breaks the loop first.");
            if current == end{
                return Some(reconstruct_path(came_from, current));
            }
            let neighbours = nodes[current[1]][current[0]].get_neightbours();
            // println!("Current: {:?}, Val: {current_value}, n_neighbours: {}",current, neighbours.len());
            for neighbour in neighbours.iter(){
                let tentative_gscore = *gscore.get(&current).get_or_insert(&default_gscore) + distance_to_neighbour(current, *neighbour);
                // println!("\t- Neighbout: {:?}, Score: {tentative_gscore}",neighbour);
                if &tentative_gscore < *gscore.get(neighbour).get_or_insert(&default_gscore){
                    came_from.insert(*neighbour, current);
                    gscore.insert(*neighbour, tentative_gscore);
                    fscore.insert(*neighbour,tentative_gscore + distance_to_end(*neighbour));
                    if let None = open_set.get(neighbour){
                        open_set.push(*neighbour, *fscore.get(neighbour).expect("We just inserted it...") as i32);
                    }
                }
            }   
        }
        None
    }
    pub fn main_1(file_name:&str){
        let mut nodes = get_nodes(file_name);
        // let x_dim = nodes[0].len();
        // let y_dim = nodes.len();
        let (start_coord, end_coord) = update_nodes(&mut nodes);
        if let Some(val) = a_star(start_coord, end_coord, nodes) {
            // let mut order_grid:Vec<Vec<i32>> = Vec::new();
            // for i in 0usize..y_dim {
            //     order_grid.push(Vec::new());
            //     for _ in 0..x_dim{
            //         order_grid[i].push(0)
            //     }
            // }
            // for (idx, step) in val.iter().rev().enumerate(){
            //     order_grid[step[1]][step[0]] = idx as i32 +1;
            //     // println!("{step:?}");
            // }
            // println!();
            // for row in order_grid{
            //     for element in row.iter() {
            //         if element == &0{
            //             print!(" ");
            //             continue
            //         }
            //         print!("{}", ".");
            //     }
            //     println!()
            // }
            println!("\nPart 1 - Shortest Route: {}",val.len()-1);
        }else{
            println!("Failed :(");
        }
    }
}
mod p2{
    use crate::p1::{get_nodes, reconstruct_path, Node, NodeType};
    use std::{fs::read_to_string, collections::HashMap};
    use std::collections::{VecDeque,HashSet};
    fn is_height_adjacent(node_a:&Node, node_b:&Node) -> bool{ // Updated to be reverse from p1.
        (node_b.height as i32 - node_a.height as i32)>=-1
    }
    fn update_nodes(nodes:&mut Vec<Vec<Node>>) ->([usize;2], [usize;2]){
        let mut start_coord:[usize;2] = [0,0];
        let mut end_coord:[usize;2] = [1,1];
        for y in 0..nodes.len(){
            for x in 0..nodes[0].len(){
                let mut top:Option<[usize;2]> = None;
                let mut left:Option<[usize;2]> = None;
                let mut right:Option<[usize;2]> = None;
                let mut bottom:Option<[usize;2]> = None;
                match nodes[y][x].node_type{
                    NodeType::Start => start_coord = nodes[y][x].coords,
                    NodeType::End => end_coord = nodes[y][x].coords,
                    NodeType::Other => ()
                }
                if y > 0 && is_height_adjacent(&nodes[y][x], &nodes[y-1][x]){
                    top = Some([x, y-1])
                }
                if y < nodes.len()-1 && is_height_adjacent(&nodes[y][x], &nodes[y+1][x]){
                    bottom = Some([x, y+1])
                }
                if x > 0 && is_height_adjacent(&nodes[y][x], &nodes[y][x-1]){
                    left = Some([x-1, y])
                }
                if x < nodes[0].len()-1 && is_height_adjacent(&nodes[y][x], &nodes[y][x+1]){
                    right = Some([x+1, y])
                }
                nodes[y][x].update(top, left, bottom, right);
            }
        }
        (start_coord, end_coord)
    }
    fn breadth_first(start:[usize;2], end_value:i32, nodes:&Vec<Vec<Node>>) -> Option<Vec<[usize;2]>>{
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
        let mut fronteer = VecDeque::<[usize;2]>::new();
        let mut seen = HashSet::<[usize;2]>::new();
        let mut came_from: HashMap<[usize;2],[usize;2]> = HashMap::new();
        seen.insert(start);
        fronteer.push_back(start);
        while !fronteer.is_empty(){
            let current_coord = fronteer.pop_front().expect("Loop terminates before this if empty");
            let current_node = &nodes[current_coord[1]][current_coord[0]];
            if current_node.height as i32 == end_value{
                return Some(reconstruct_path(came_from, current_coord));
            }
            for neighbour in current_node.get_neightbours(){
                if !seen.contains(&neighbour){
                    seen.insert(neighbour);
                    came_from.insert(neighbour, current_coord);
                    fronteer.push_back(neighbour);
                }
            }
        }
        None
    }

    pub fn main_2(file_name:&str){
        let mut nodes = get_nodes(file_name);
        let (_, start_coord) = update_nodes(&mut nodes);
        let end_value = 0;
        if let Some(val) = breadth_first(start_coord,end_value,&nodes) {
            println!("\nPart 2 - Shortest Route: {}",val.len()-1);
        }else{
            println!("Failed :(");
        }
    }
}

use p1::main_1;
use p2::main_2;
fn main() {
    let file_name = r"src\dummy_input.txt";
    // let file_name = r"src\smaller_puzzle_input.txt";
    // let file_name = r"src\small_puzzle_input.txt";
    let file_name = r"src\puzzle_input.txt";
    // main_1(file_name);
    main_2(file_name);
}
