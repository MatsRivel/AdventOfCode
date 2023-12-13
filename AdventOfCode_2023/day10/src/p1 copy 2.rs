use std::{fs::read_to_string, collections::HashSet};
type Coord = [usize;2];
trait Adjust{
    fn north(&self)->Option<Self> where Self: Sized;
    fn south(&self)->Option<Self> where Self: Sized;
    fn east(&self) ->Option<Self> where Self: Sized;
    fn west(&self) ->Option<Self> where Self: Sized;
}
impl Adjust for Coord{
    fn north(&self)->Option<Coord> {
        if self[0] == 0{
            None
        }else{
            Some([self[0]-1,self[1]])
        }
    }

    fn south(&self)->Option<Coord> {
        if self[0] == usize::MAX{
            None
        }else{
            Some([self[0]+1,self[1]])
        }
    }

    fn east(&self)->Option<Coord> {
        if self[1] == usize::MAX{
            None
        }else{
            Some([self[0],self[1]+1])
        }
    }

    fn west(&self)->Option<Coord> {
        if self[1] == 0{
            None
        }else{
            Some([self[0],self[1]-1])
        }
    }
}
#[derive(Clone,Copy,Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir{
    N,
    S,
    E,
    W
}
impl Dir{
    fn reverse(&self) -> Self{
        match self{
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::E,
            Dir::W => Dir::W,
        }
    }
}
impl From<[Coord;2]> for Dir{
    fn from(value: [Coord;2]) -> Self {
        // Going from value[0] to value[1] you go in direction Self.
        // Only intended to work on adjacent tiles, not counting diagonal adjacency.
        let [[x1,y1],[x2,y2]] = value;
        let diff = [x2 as i32 - x1 as i32,y2 as i32 - y1 as i32];
        match diff{
            [ 0,-1] => Dir::W,
            [ 0, 1] => Dir::E,
            [-1, 0] => Dir::N,
            [ 1, 0] => Dir::S,
            _ => panic!("Only works for adjacent coords!")
        }
    }
}
#[derive(Clone)]
struct Tile{
    char: char,
    dirs: Vec<Dir>,
}
impl Tile{
    fn len(&self)->usize{
        self.dirs.len()
    }
    fn get_neighbours(&self, coords: Coord, max_x:usize, max_y:usize)->Vec<Coord>{
        // For each Dir this tile includes, check if adjacent Coords are within bounds.
        // If so, they're included in the output.
        self.dirs
            .iter()
            .filter_map(|dir| {
                match *dir{
                    Dir::N => coords.north(),
                    Dir::S => {
                        if coords[0] < max_x{ coords.south()}
                        else{None}
                    },
                    Dir::E => {
                        if coords[1] < max_y{ coords.east()}
                        else{None}
                    },
                    Dir::W => coords.west(),
                }
            }).collect::<Vec<Coord>>()
    }
}
impl From<char> for Tile{
    fn from(value: char) -> Self {
        let dirs = match value{
            '|' => vec![Dir::N,Dir::S],
            'L' => vec![Dir::N,Dir::E],
            'J' => vec![Dir::N,Dir::W],
            '-' => vec![Dir::E,Dir::W],
            'F' => vec![Dir::E,Dir::S],
            '7' => vec![Dir::S,Dir::W],
            'S' => vec![Dir::N,Dir::E,Dir::S,Dir::W],
            _ => vec![] // No neighbours if there is no pipe.
        };
        Self{char:value, dirs}
    }
}

struct Node{
    current: Coord,
    dirs: Vec<Dir>,
    neighbours: Vec<Coord>
}
struct NodeMatrix{
    nodes: Vec<Node>,
    height: usize,
    width: usize
}
impl NodeMatrix{
    fn idx_from_coord(&self,coord:&Coord)->usize{
        self.width * coord[0] + coord[1]
    }
    fn coord_from_idx(&self,idx:usize)->Coord{
        [idx/self.width,idx%self.width]
    }
    fn get_by_coord(&self,coord:&Coord)->&Node{
        let idx = self.idx_from_coord(&coord);
        self.get_by_idx(idx)
    }
    fn get_by_idx(&self,idx:usize)->&Node{
        &self.nodes[idx]
    }
}
fn make_tiles(data_string:String)->(Coord,Vec<Vec<Tile>>){
    let mut start_idx = [0usize;2];
    let tiles = data_string
        .lines()
        .enumerate()
        .map(|(x,line)| {
            line.chars()
            .enumerate()
            .map(|(y,c)| {
                let tile = Tile::from(c);
                if tile.len() == 4{
                    start_idx = [x,y]
                }
                tile
            })
            .collect::<Vec<Tile>>()} )
        .collect::<Vec<Vec<Tile>>>();
    return (start_idx,tiles)
}
// fn neighbour_guard_clause(xy:Coord,ij:[i8;2],xy_max:Coord)->bool{
//     // Returns true if the values break with our conditions. False otherwise.
//     (ij[0] == ij[1] && ij[0] == 0) 
//     || (xy[0] == 0 && ij[0] == -1) 
//     || (xy[1] == 0 && ij[1] == -1) 
//     || (xy[0] == xy_max[0] && ij[0] == 1)
//     || (xy[1] == xy_max[1] && ij[1] == 1)
// }
// fn get_all_neighbours(coords: Coord, max_x:usize, max_y:usize)->Vec<Coord>{
//     let mut neighbours = Vec::new();
//     for i in -1..=1{
//         for j in -1..=1{
//             if neighbour_guard_clause(coords, [i,j], [max_x,max_y]){
//                 continue;
//             }
//             let new_coord = [(coords[0] as i64 - i as i64) as usize,(coords[1] as i64 - j as i64) as usize ];
//             neighbours.push(new_coord);
//         }
//     }
//     neighbours
// }
fn traverse_tiles(start: &Coord, previous:&Coord, current:&Coord, matrix: &NodeMatrix, depth:usize)->Option<usize>{
    // for _ in 0..depth{
    //     print!(" ");
    // }
    // println!("{current:?}");
    if current == start{
        return Some(depth);
    }
    let next_vec = matrix.get_by_coord(current).neighbours.iter().filter(|n| *n != previous).map(|n| n.clone()).collect::<Vec<Coord>>();
    if next_vec.is_empty(){ // Only valid path leads out of the existing grid. (Dead-end)
        return None;
    }else if next_vec.len() != 1{ 
        // This can only happen if we go from the start into a pipe not connected to the start.
        // If so, the path is not valid and we abort.
        return None;
    }
    let next = next_vec.first().unwrap();
    traverse_tiles(start, current, next, matrix, depth+1)

}
fn initialize_traverse_tiles(start_coord: &Coord, current: &Coord, matrix: &NodeMatrix)->Option<usize>{
    // let mut memory: HashSet<[usize; 2]> = HashSet::new();
    // memory.insert(start_coord.clone());
    let previous = start_coord;
    traverse_tiles(start_coord, previous, current, matrix,  2)
}
fn nodes_from_tiles(tiles:Vec<Vec<Tile>>) -> NodeMatrix{
    let height = tiles.len();
    let width = tiles[0].len();
    let mut nodes = Vec::with_capacity(height*width);
    for (xidx, tile_row) in tiles.into_iter().enumerate(){
        for (yidx, tile) in tile_row.into_iter().enumerate(){
            let current = [xidx,yidx];
            let neighbours = tile.dirs.iter().filter_map(|dir|{
                let neigh = match dir{
                    Dir::N => current.north(),
                    Dir::S => current.south(),
                    Dir::E => current.east(),
                    Dir::W => current.west(),
                };
                if let Some(n) = neigh{
                    if n[0] >= height || n[1] >= width{
                        None
                    }else{
                        Some(n)
                    }
                }else{
                    None
                }
            }).collect::<Vec<Coord>>();
            let node = Node{current,neighbours,dirs: tile.dirs};
            nodes.push(node);
        }
    }
    NodeMatrix { nodes, height, width }

}


fn make_directed_graph(matrix:&NodeMatrix,start:&Coord,first_step:&Coord)->Option<Vec<Coord>>{
    #[cfg(test)]
    print!("{start:?}-> {first_step:?}-> ");
    #[cfg(test)]
    let mut counter = 0;

    let mut path = vec![start.clone(),first_step.clone()];
    let mut current = start.clone();
    let mut prev = start.clone();
    loop{
        #[cfg(None)]{
            counter += 1;
            if counter > 25{
                panic!("Running too long!");
            }
        }
        prev = current;
        current = path.last().unwrap().clone();
        // println!("{prev:?},{current:?}");
        if current == prev{
            return None;
        }
        let neighbours = &matrix.get_by_coord(&current).neighbours;
        // A node must have neighbours, else it is either "ground" or at an edge.
        // If b is a neighbour of a, a must be a neighbour of b. If not, invalid path.
        // It should not be the ONLY neighbour, though. That would be a dead-end.

        // println!("Current: {current:?}, neighbours: {neighbours:?}");
        if neighbours.is_empty() || (neighbours.len() == 1 && neighbours[0] == prev){
            #[cfg(test)]
            print!("X");
            return None;
        }
        let only_neighbour;
        // println!("\n >>> {prev:?}, {current:?}, {neighbours:?} <<< \n");

        if neighbours[0] == prev {
            only_neighbour = neighbours[1];
        }else{
            only_neighbour = neighbours[0];
        }
        let neighbours_neighbour = &matrix.get_by_coord(&only_neighbour).neighbours;
        if !neighbours_neighbour.contains(&current){
            #[cfg(test)]
            print!(">>> {neighbours_neighbour:?} <<< #");
            return None;
        }

        if &only_neighbour == path.first().unwrap(){
            #[cfg(test)]
            println!("{:?} Y",only_neighbour);
            return Some(path.into_iter().map(|p| p.clone()).collect::<Vec<Coord>>());
        }
        #[cfg(test)]
        print!("{only_neighbour:?}-> ");
        path.push(only_neighbour);
    }

}
pub fn main_1(file_name:&str)->Option<usize>{
    /*
    Plan:
    Define loop. (Continue until you find an existing node?)
    Follow loop all the way through.
    Cut that distance in half.
    Done.
     */
    let data_string = read_to_string(file_name).unwrap();
    let (start_coord, tiles): (Coord, Vec<Vec<Tile>>) = make_tiles(data_string);
    let adjacency_matrix = nodes_from_tiles(tiles.clone());
    let start_neighbours = &adjacency_matrix.get_by_coord(&start_coord).neighbours;
    #[cfg(test)]
    let mut counter = 0;
    for neighbour in start_neighbours.iter(){
        if let Some(mut graph) = make_directed_graph(&adjacency_matrix,&start_coord,&neighbour){
            #[cfg(test)]{
                println!();
                let mut new_matrix = vec![' ';adjacency_matrix.height*adjacency_matrix.width];
                for i in 0..graph.len(){
                    let coord = graph[i];
                    let new_idx = adjacency_matrix.idx_from_coord(&coord);
                    // new_matrix[new_idx] = *format!("{}",counter % 10).chars().collect::<Vec<char>>().first().unwrap();
                    new_matrix[new_idx] = tiles[coord[0]][coord[1]].char;
                    counter += 1;
                }
                for x in 0..adjacency_matrix.height{
                    print!("\t");
                    for y in 0..adjacency_matrix.width{
                        let coord = [x,y];
                        let idx = adjacency_matrix.idx_from_coord(&coord);
                        print!("{}",new_matrix[idx]);
                    }
                    println!();
                }
                println!();
            }
            // let max_dist = graph
            //     .iter()
            //     .enumerate()
            //     .zip( 
            //         graph.iter().enumerate().rev()
            //     )
            //     .map(|((forward,_nodea),(backward,_nodeb))| {
            //         #[cfg(test)]
            //         println!("{_nodea:?}: ({}, {})",forward+1,backward+1);
            //         std::cmp::min(forward,backward)+1
            //     })
            //     .max()
            //     .unwrap();
            // todo!();
            graph.push(start_coord);
            let diff = graph.iter().zip(graph.iter().skip(1)).map(|(prev,next)|{
                let diffs = [prev[0] as i32 - next[0] as i32, prev[1] as i32 - next[1] as i32];
                diffs
            }).fold([0;2], |acc,val|{
                [acc[0] + val[0], acc[1] + val[1]]
            });
            println!("{diff:?}");
            return Some( graph.len()/2 );
        }
        println!();
    }
    #[cfg(test)]{
        panic!("Tests all have a defined answer, so should never return None!")
    }
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn idx_tester(){
        let nodes = vec![
                Node{current: [0,0],dirs:vec![], neighbours:vec![]},
                Node{current: [0,1],dirs:vec![], neighbours:vec![]},
                Node{current: [0,2],dirs:vec![], neighbours:vec![]},
                Node{current: [1,0],dirs:vec![], neighbours:vec![]},
                Node{current: [1,1],dirs:vec![], neighbours:vec![]},
                Node{current: [1,2],dirs:vec![], neighbours:vec![]}
            ];
        let nm = NodeMatrix{ nodes, height: 2, width: 3 };
        assert_eq!(nm.idx_from_coord(&[0,0]),0);
        assert_eq!(nm.idx_from_coord(&[0,1]),1);
        assert_eq!(nm.idx_from_coord(&[0,2]),2);
        assert_eq!(nm.idx_from_coord(&[1,0]),3);
        assert_eq!(nm.idx_from_coord(&[1,1]),4);
        assert_eq!(nm.idx_from_coord(&[1,2]),5);
    }   
    #[test]
    fn coord_tester(){
        let nodes = vec![
                Node{current: [0,0],dirs:vec![], neighbours:vec![]},
                Node{current: [0,1],dirs:vec![], neighbours:vec![]},
                Node{current: [0,2],dirs:vec![], neighbours:vec![]},
                Node{current: [1,0],dirs:vec![], neighbours:vec![]},
                Node{current: [1,1],dirs:vec![], neighbours:vec![]},
                Node{current: [1,2],dirs:vec![], neighbours:vec![]}
            ];
        let nm = NodeMatrix{ nodes, height: 2, width: 3 };
        assert_eq!(nm.coord_from_idx(0),[0,0]);
        assert_eq!(nm.coord_from_idx(1),[0,1]);
        assert_eq!(nm.coord_from_idx(2),[0,2]);
        assert_eq!(nm.coord_from_idx(3),[1,0]);
        assert_eq!(nm.coord_from_idx(4),[1,1]);
        assert_eq!(nm.coord_from_idx(5),[1,2]);
    }  


}
