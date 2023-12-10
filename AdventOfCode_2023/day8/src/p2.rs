use std::{fs::read_to_string, collections::HashMap};
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
enum Dir{
    Left,
    Right
}
impl Dir{
    fn new(c:char)->Self{
        match c{
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!()
        }
    }
}
#[derive(Clone)]
struct WalkOrder{
    path: std::iter::Cycle<std::vec::IntoIter<Dir>>
}
impl WalkOrder{
    fn new(s:&str)->Self{
        let mut temp = Vec::with_capacity(s.len());
        for c in s.chars(){
            temp.push(Dir::new(c));
        }
        let path = temp.into_iter().cycle();
        WalkOrder{ path }
    }
}
impl Iterator for WalkOrder{
    type Item=Dir;
    fn next(&mut self) -> Option<Self::Item> {
        self.path.next()
    }
}

fn process_data_string(data_string:&str)->(WalkOrder,Vec<[usize;3]>,Vec<usize>){
    let mut lines = data_string.lines();
    let path_str = lines.next().unwrap();
    let path = WalkOrder::new(path_str);
    lines.next(); // Skipping empty line.
    let mut start_nodes_initial = vec![];
    let mut name_to_idx = HashMap::<&str, usize>::new();
    let nodes_initial = lines.enumerate().map(|(idx,line)| {
        let [name,temp]: [&str;2] = line.split(" = (").collect::<Vec<&str>>().try_into().unwrap();

        name_to_idx.insert(name, idx);

        let [left_child, right_child]: [&str;2] = temp
            .split(", ")
            .map(|s| {
                match s.strip_suffix(")"){
                    Some(new_s) => new_s,
                    None => s
                }
            })
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        if name.ends_with("A"){
            start_nodes_initial.push(name)
        }
        (name,[left_child,right_child])
        }).collect::<HashMap<&str,[&str;2]>>(); 
    let mut nodes = vec![[0;3];nodes_initial.len()];
    nodes_initial.into_iter().for_each(|(name,[left_child,right_child])| {
        let name_idx = *name_to_idx.get(name).unwrap();
        let left_idx = *name_to_idx.get(left_child).unwrap();
        let right_idx = *name_to_idx.get(right_child).unwrap();
        let is_end = match name.ends_with("Z"){
            true => 1,
            false => 0
        };
        nodes[name_idx] = [left_idx,right_idx, is_end];
    });
    let start_nodes = start_nodes_initial.iter().map(|s| *name_to_idx.get(s).unwrap()).collect::<Vec<usize>>();
    (path,nodes,start_nodes)
}


fn find_loop_start(mut slow_path: WalkOrder, nodes: &Vec<[usize;3]>, start_node: usize )->usize{
    let mut fast_path = slow_path.clone();
    let mut slow_dir = slow_path.next().unwrap();
    let mut fast_dir = fast_path.next().unwrap();
    let mut slow = match slow_dir{
        Dir::Left => nodes[start_node][0],
        Dir::Right =>nodes[start_node][1],
    };
    let mut fast = match fast_dir{
        Dir::Left => nodes[start_node][0],
        Dir::Right =>nodes[start_node][1],
    };
    fast_dir = fast_path.next().unwrap();
    fast = match fast_dir{
        Dir::Left => nodes[fast][0],
        Dir::Right =>nodes[fast][1],
    };
    let mut n_steps = 1;
    loop {
        n_steps += 1;
        slow_dir = slow_path.next().unwrap();
        slow = match slow_dir{
            Dir::Left => nodes[slow][0],
            Dir::Right =>nodes[slow][1],
        };
        for _ in 0..2{
            fast_dir = fast_path.next().unwrap();
            fast = match fast_dir{
                Dir::Left => nodes[fast][0],
                Dir::Right =>nodes[fast][1],
            }; 
            if fast == slow && fast_dir == slow_dir{
                return n_steps;
            }
        }
    }
}

fn get_n_nodes(path: WalkOrder, nodes: &Vec<[usize;3]>, start_node:usize, n_nodes:usize)->Vec<(usize,Dir)>{
    let mut history = Vec::with_capacity(n_nodes);
    let mut temp_path = path.clone();
    let mut dir = temp_path.next().unwrap();
    let mut node = match dir{
        Dir::Left   => nodes[start_node][0],
        Dir::Right  =>  nodes[start_node][1],
    };
    history.push((node,dir));
    for _ in 0..n_nodes{
        dir = temp_path.next().unwrap();
        node = match dir{
            Dir::Left   => nodes[node][0],
            Dir::Right  =>  nodes[node][1],
        };
    }
    history
}
fn define_loop(path: WalkOrder, nodes: &Vec<[usize;3]>, start_node: usize )->[usize;2]{
    // Get the maximum number of steps we would need to find a loops starting point.
    let n_steps = find_loop_start(path.clone(),nodes,start_node);
    // Here we know that the start of the loop occurs <= n_steps into the path.
    let history = get_n_nodes(path.clone(),nodes,start_node,n_steps);
    // Define the loop sepsifically:
    let mut seen = HashMap::new();
    // Note: The two values below will 100% be overwritten, 
    // but I can not prove it to the compiler, so we set it to 0 for now.
    let mut loop_length = 0;
    let mut loop_start_idx = 0;
    for (idx,state) in history.iter().enumerate(){
        if let Some(seen_at_idx) = seen.get(state){
            loop_start_idx = *seen_at_idx;
            loop_length = idx - seen_at_idx;
            break;
        }else{
            seen.insert(*state, idx);
        }
    }
    [loop_start_idx,loop_length]
}



pub fn main_2(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let (mut path,nodes, mut start_nodes) = process_data_string(&data_string);
    let mut counter = 0;
    let mut all_done = false;
    let loop_sizes = start_nodes
        .iter()
        .map(|start_node| define_loop(path.clone(), &nodes, *start_node))
        .collect::<Vec<[usize;2]>>();
    let max_start_of_loop = loop_sizes.iter().fold(0, |acc, [start,_]| std::cmp::min(*start,acc) );
    // Fist we walk until the start of the latest loop:
    for _ in 0..max_start_of_loop{
        counter += 1;
        let dir = path.next().unwrap();
        for current_idx in start_nodes.iter_mut(){
            match dir{
                Dir::Left   => *current_idx = nodes[*current_idx][0],
                Dir::Right  => *current_idx = nodes[*current_idx][1],
            }
            all_done =  nodes[*current_idx][2]==1 && all_done;
        }
    }
    // Now must take the step of the path with the longest distance between its loop starting and 



    /*
        Idea:
        - Find each loop start.
        - Walk until every node has entered its loop.
        - Find the longest distance from a nodes current position to its goal.
        - Also find the distance from the goal back to the current position. (Remember: They will likely not be the same!)
        - Note: These two values are fixed now. 
        loop:
            - Walk n == 'longest distance to goal' steps.
            - Increment counter.
            if 'all nodes are valid':
                return counter;
            - Walk n == 'distance from goal to loop-start' steps.
            - Increment counter.
            if 'all nodes are valid':
                return counter;
    */

    Some(counter)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
