use std::{fs::read_to_string, collections::{HashSet, VecDeque}};
use crate::p1::{Adjust,Pipe,Dir,process_data_string,Node, Coord, count_loop_length};
pub fn other_end_of_loop(nodes: &Vec<Vec<Node>>, start:Coord, prev: Coord, current:Coord)->Coord{
    if current == start{
        return prev;
    }
    // Only one neighbour is valid.
    // By avoiding loop and options we can bait the compiler into tail-recursion.
    // This lets us recurse near indefinetly.
    let current_node = &nodes[current[0]][current[1]];
    let neighbour = *current_node.neighbours
        .iter()
        .filter(|neigh| **neigh != prev)
        .last()
        .or(Some(&[usize::MAX,usize::MAX]))
        .unwrap();
    // This check check does not hinder tail-recursion, apparently.
    // Using an option DOES hinder it, which is why I've decided to do this.
    if neighbour == [usize::MAX,usize::MAX]{
        return start;
    }
    return other_end_of_loop(nodes, start, current, neighbour);
}

pub fn build_loop(nodes: &Vec<Vec<Node>>, start:Coord, prev: Coord, current:Coord)->Option<Vec<Coord>>{
    if current == start{
        return Some(vec![current]);
    }
    // Only one neighbour is valid.
    let current_node = &nodes[current[0]][current[1]];
    let maybe_neighbour = current_node.neighbours
        .iter()
        .filter(|neigh| **neigh != prev)
        .last();
    
    let neighbour = match maybe_neighbour{
            Some(v) => *v,
            None => return None
        };
    if let Some(mut output) = build_loop(nodes, start, current, neighbour){
        output.push(current);
        return Some(output);
    };
    None
}

struct PathIterator{
    nodes: Vec<Vec<Node>>,
    start: Coord,
    current: Coord,
}
impl Iterator for PathIterator{
    type Item=Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let [x,y] = self.current;
        let neighbour = (&self.nodes[x][y].neighbours).into_iter().filter(|n| **n != [x,y]).last().unwrap();
        self.current = neighbour.clone();
        if self.current != self.start{
            return Some(neighbour.clone());
        }
        None
    }
}
fn build_path(nodes: &Vec<Node>, start:usize, mut next: usize,ymax:usize)->VecDeque<usize>{
    println!("Build start:");
    let mut current = start.clone();
    let mut output = VecDeque::with_capacity(nodes.len());
    output.push_back(current);
    while next != start{
        output.push_back(next);
        let neighbour = (&nodes[next].neighbours).into_iter().map(|n| n[0]*ymax+n[1]).filter(|n| *n != current).last().unwrap();
        current = next;
        next = neighbour.clone();
    }
    println!("Build end.");
    output
}
impl Dir{
    fn turn_left(&self)->Self{
        match self{
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }
    fn turn_right(&self)->Self{
        self.turn_left().reverse()
    }
}
#[derive(Debug,PartialEq,Clone)]
pub enum HandSide{
    Left,
    Right,
    Occupied
}

pub fn width_first_covering(type_matrix:&mut Vec<Vec<Option<u32>>>, start:Coord){
    let mut seen = HashSet::<Coord>::new();
    let mut to_visit = VecDeque::<Coord>::new();
    to_visit.push_back(start);
    let xmax = type_matrix.len();
    let ymax = type_matrix[0].len();
    while let Some(current) = to_visit.pop_front(){
        seen.insert(current);
        if type_matrix[current[0]][current[1]].is_none(){
            type_matrix[current[0]][current[1]] = type_matrix[start[0]][start[1]].clone();
        }
        let neighbours = {
            [current.north(), current.south(), current.east(), current.west()]
                .into_iter()
                .filter_map(|neighbour|{ // TODO: Horriffic section. Should rewrite
                    if neighbour.is_none(){
                        None
                    }else{
                        let [x,y] = neighbour.unwrap();
                        if x >= xmax || y >= ymax{
                            None
                        }else{
                            if type_matrix[x][y].is_none(){
                                Some([x,y])
                            }else{
                                None
                            }
                        }
                    }
                }).collect::<Vec<Coord>>()
        };
        println!("Current: {:?} -> {:?}", current, neighbours);
        for neighbour in neighbours.into_iter(){
            if seen.contains(&neighbour){
                continue;
            }
            to_visit.push_back(neighbour);
        }
    }
}


pub fn loop_tail(nodes: &Vec<Node>, start:usize, prev: usize, current:usize, ymax:usize)->usize{
    if current == start{
        return prev;
    }
    // Only one neighbour is valid.
    // By avoiding loop and options we can bait the compiler into tail-recursion.
    // This lets us recurse near indefinetly.
    
    let neighbour = nodes[current].neighbours
        .iter()
        .map(|[x,y]| x*ymax + y)
        .filter(|neigh| *neigh != prev)
        .last()
        .or(Some(usize::MAX))
        .unwrap();
    if neighbour == usize::MAX{
        return neighbour;
    }
    return loop_tail(nodes, start, current, neighbour,ymax);
}
pub fn main_2(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let (start_coord, nodes): (Coord, Vec<Vec<Node>>) = process_data_string(data_string);
    let [xmax,ymax] = [nodes.len(),nodes[0].len()]; 
    let flat_nodes = nodes.into_iter().flatten().collect::<Vec<Node>>();
    let start_idx = start_coord[0] * ymax + start_coord[1];
    let start_node = &flat_nodes[start_idx];
    let mut tail_list = vec![];
    for neighbour in start_node.neighbours.iter(){
        let neighbour_idx = neighbour[0]*ymax + neighbour[1];
        let neighbour_node = &flat_nodes[neighbour_idx];
        if start_node.are_connected(neighbour_node){
            let tail = loop_tail(&flat_nodes, start_idx, start_idx, neighbour_idx,ymax);
            if tail != usize::MAX{
                tail_list.push((neighbour_idx,tail));
            }
        }
    }
    println!("{tail_list:?}");
    // Now we KNOW there is a finite loop, and it starts at 'start_coord'.
    // We can now make a vec containing all elements in the loop.
    let start_node = &flat_nodes[start_idx];
    let randon_valid_neighbour = start_node.neighbours.iter().last().unwrap(); // We know either neighbour is valid.
    // let path2 = build_loop(&nodes, start_coord, start_coord.clone(), randon_valid_neighbour.clone()).unwrap();
    // println!("{path2:?}");
    let rvn_idx = randon_valid_neighbour[0]*ymax + randon_valid_neighbour[1];
    let path = build_path(&flat_nodes, start_idx,rvn_idx,ymax);
    println!("\n{path:?}");
    // println!("Size of path: {}", path.len());
    // println!("{path:?}");
    #[cfg(None)]{
    let path_pairs = path.iter().zip(path.iter().rev()).map(|(left,right)|[*left,*right] ).collect::<Vec<[Coord;2]>>();
        let mut type_matrix = nodes
            .iter()
            .map(|inner_nodes| {
                inner_nodes.into_iter()
                    .map(|node| {
                        if node.pipe == Pipe::Missing{
                            None
                        }else{
                            Some(0)
                        }
                    }).collect::<Vec<Option<u32>>>()
            }).collect::<Vec<Vec<Option<u32>>>>();
        
        for [a,b] in path_pairs.iter(){
            // For each pair, draw a line from 'a' to 'b'.
            // Count the times a pipe is crossed to get to the end.
            let mut cross_count = 1;
            let mut c = a.clone();
            while &c != b{
                let mut diff = [b[0] as i32 - c[0] as i32, b[1] as i32 - c[1] as i32];
                diff[0] = diff[0] / std::cmp::max(diff[0].abs(), diff[1].abs());
                diff[1] = diff[1] / std::cmp::max(diff[0].abs(), diff[1].abs());
                // Tie-breaker :)
                if diff[0].abs() == diff[1].abs(){
                    diff[1] = 0
                }
                // Now one value is +-1, and one is 0.
                // Adjust c so that it approaches b:
                c = [(c[0] as i32 + diff[0]) as usize, (c[1] as i32 + diff[1]) as usize ];
                if type_matrix[c[0]][c[1]].is_none(){
                    // Unlabeled space gets labeled.
                    type_matrix[c[0]][c[1]] = Some(cross_count);
                    // Once ONE space in an area is identified, every space in that area is also identified.
                    // (We just dont know if it is INISDE or OUTSIDE yet).
                    // So any adjacent empty tiles of the existing one will also be covered.
                    width_first_covering(&mut type_matrix,c);
                }else if type_matrix[c[0]][c[1]] == Some(0){
                    // We crossed a pipe, so we flip the type.
                    cross_count += 1;
                }
            }
        }
        #[cfg(test)]{
            for row in type_matrix.iter(){
                for point in row.iter(){
                    if let Some(p) = point{
                        if *p == 0{
                            print!("X");
                        }else{
                            print!("{}",1+(p%9));
                        }
                    }else{
                        print!(" ")
                    }
                }
                println!();
            }
            println!();
        }
        }
    let mut is_left = flat_nodes
        .iter()
        .map(|node| {
            if node.pipe == Pipe::Missing{
                None
            }else{
                Some(HandSide::Occupied)
            }
        }).collect::<Vec<Option<HandSide>>>();
    
    for (current, next) in path.clone().into_iter().zip(path.into_iter().skip(1)){
        let current_coord =[ (current / ymax), (current % ymax)];
        let next_coord = [(next / ymax), (next % ymax)];
        let dir = Dir::from([current_coord,next_coord]);
        let left_of_dir = dir.turn_left();
        let pos_left_of_current = match left_of_dir{
            Dir::N => current_coord.north(),
            Dir::S => current_coord.south(),
            Dir::E => current_coord.east(),
            Dir::W => current_coord.west(),
        };
        if let Some([x,y]) = pos_left_of_current{
            if x < xmax && y < ymax{
                let idx = x*ymax + y;
                if is_left[idx].is_none(){
                    is_left[idx] = Some(HandSide::Left);
                }
            }
        }
        // let right_of_dir = dir.turn_right();
        // let pos_right_of_current = match right_of_dir{
        //     Dir::N => current.north(),
        //     Dir::S => current.south(),
        //     Dir::E => current.east(),
        //     Dir::W => current.west(),
        // };
        // if let Some([x,y]) = pos_right_of_current{
        //     if x < xmax && y < ymax{
        //         if is_left[x][y].is_none(){
        //             is_left[x][y] = Some(HandSide::Right);
        //         }
        //     }
        // }
    }
    #[cfg(test)]{
        for x in 0..xmax{
            for y in 0..ymax{
                let point = is_left[x*ymax + y ];
                if let Some(p) = point{
                    if *p == HandSide::Occupied{
                        print!("X");
                    }else if *p == HandSide::Left{
                        print!("L");
                    }
                    else{
                        print!("R");
                    }
                }else{
                    print!(" ")
                }
            }
            println!();
        }
        println!();
    }
    let total_count = is_left.len();
    let left_count = is_left
        .iter()
        .flat_map(|row| row)
        .filter(|tile| tile.is_some())
        .filter(|tile| {
            match tile{
                Some(HandSide::Left) => true,
                _ => false,
            }
        })
        .count();
    let right_count = is_left
        .iter()
        .flat_map(|row| row)
        .filter(|tile| tile.is_none())
        .count();
    println!("total: {total_count}, left: {left_count}, right: {}", right_count);
    Some(left_count)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn left_right(){
        let lefts   = vec![Dir::W, Dir::E, Dir::N, Dir::S];
        let dirs    = vec![Dir::N, Dir::S, Dir::E, Dir::W];
        let rights  = vec![Dir::E, Dir::W, Dir::S, Dir::N];
        for (dir,(left,right)) in dirs.iter().zip(lefts.iter().zip(rights.iter())){
            assert_eq!(dir.turn_left(),*left);
            assert_eq!(dir.turn_right(),*right);
        }
    }

}
