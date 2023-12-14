use std::fs::read_to_string;
pub type Coord = [usize;2];
pub trait Adjust{
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
pub enum Dir{
    N,
    S,
    E,
    W
}
impl Dir{
    pub fn reverse(&self) -> Self{
        match self{
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
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
#[derive(Debug,Clone,PartialEq)]
pub enum Pipe{
    NtoE,
    NtoS,
    NtoW,
    EtoS,
    EtoW,
    StoW,
    Start,
    Missing
}
impl Pipe{
    pub fn to_dirs(&self)->Option<Vec<Dir>>{
        match self{
            Pipe::NtoE => Some(vec![Dir::N,Dir::E]),
            Pipe::NtoS => Some(vec![Dir::N,Dir::S]),
            Pipe::NtoW => Some(vec![Dir::N,Dir::W]),
            Pipe::EtoS => Some(vec![Dir::E,Dir::S]),
            Pipe::EtoW => Some(vec![Dir::E,Dir::W]),
            Pipe::StoW => Some(vec![Dir::S,Dir::W]),
            Pipe::Start=> Some(vec![Dir::N, Dir::E, Dir::S,Dir::W]),
            Pipe::Missing => Some(vec![]),
        }
    }
}
impl From<[Dir;2]> for Pipe{
    fn from(value: [Dir;2]) -> Self {
        match value[0]{
            Dir::N => {
                match value[1]{
                    Dir::N => Pipe::Missing,
                    Dir::S => Pipe::NtoS,
                    Dir::E => Pipe::NtoE,
                    Dir::W => Pipe::NtoW,
                }
            },
            Dir::S => {
                match value[1]{
                    Dir::N => Pipe::NtoS,
                    Dir::S => Pipe::Missing,
                    Dir::E => Pipe::EtoS,
                    Dir::W => Pipe::StoW,
                }
            },
            Dir::E => {
                match value[1]{
                    Dir::N => Pipe::NtoE,
                    Dir::S => Pipe::EtoS,
                    Dir::E => Pipe::Missing,
                    Dir::W => Pipe::EtoW,
                }
            },
            Dir::W => {
                match value[1]{
                    Dir::N => Pipe::NtoW,
                    Dir::S => Pipe::StoW,
                    Dir::E => Pipe::EtoW,
                    Dir::W => Pipe::Missing,
                }
            },
        }
    }
}
impl From<char> for Pipe{
    fn from(value: char) -> Self {
        match value{
            '-' => Self::EtoW,
            '|' => Self::NtoS,
            'F' => Self::EtoS,
            'J' => Self::NtoW,
            '7' => Self::StoW,
            'L' => Self::NtoE,
            '.' => Self::Missing,
            'S' => Self::Start,
            _ => panic!("Invalid char-to-pipe! >>{value}<<<")
        }
    }
}
#[derive(Debug)]
pub struct Node{
    pub pipe: Pipe,
    pub coord: Coord,
    pub neighbours:Vec<Coord>
}
impl Node{
    pub fn new(pipe:Pipe, coord:Coord, xmax:usize,ymax:usize)->Self{
        let dirs = pipe.to_dirs().unwrap();
        let mut neighbours = vec![];
        for dir in dirs.into_iter(){
            let adjustment = match dir{
                Dir::N => coord.north(),
                Dir::S => coord.south(),
                Dir::E => coord.east(),
                Dir::W => coord.west(),
            };
            if let Some(dir_coord) = adjustment{
                if dir_coord[0] < xmax && dir_coord[1] < ymax{
                    neighbours.push(dir_coord);
                }
            }
        }
        Node{pipe,coord,neighbours}
    }
    pub fn are_connected(&self, other:&Self)->bool{
        let dir:Dir = [self.coord,other.coord].into();
        // The pipe in "self" must point in the direction we are going.
        if self.pipe == Pipe::Missing || other.pipe == Pipe::Missing{
            return false;
        }
        if !self.pipe.to_dirs().unwrap().contains(&dir){
            return false;
        } 
        // The pipe in "other" must point in the opposite direction of where we are going.
        if !other.pipe.to_dirs().unwrap().contains(&dir.reverse()){
            return false;
        }
        // The pipes must be directly, non-diagonally, adjacent.
        let diff = [self.coord[0] as i32 - other.coord[0] as i32, self.coord[1] as i32 - other.coord[1] as i32];
        if diff[0].abs() + diff[1].abs() > 1{
            return false;
        }
        true
    }
}
pub fn process_data_string(data_string:String)->(Coord, Vec<Vec<Node>>){
    let mut start_idx = [0usize;2];
    let tiles = data_string
        .lines()
        .enumerate()
        .map(|(x,line)| {
            line.chars()
            .enumerate()
            .map(|(y,c)| {
                let pipe = Pipe::from(c);
                if let Pipe::Start = pipe{
                    start_idx = [x,y]
                }
                pipe
            })
            .collect::<Vec<Pipe>>()} )
        .collect::<Vec<Vec<Pipe>>>();
    let xmax = tiles.len();
    let ymax = tiles[0].len();
    let nodes = tiles
        .into_iter()
        .enumerate()
        .map(|(x,line)| {
            line.into_iter()
                .enumerate()
                .map(|(y,pipe)| Node::new(pipe,[x,y],xmax,ymax))
                .collect::<Vec<Node>>()
        }).collect::<Vec<Vec<Node>>>();
    
    (start_idx,nodes)

}

pub fn count_loop_length(nodes: &Vec<Vec<Node>>, start:Coord, prev: Coord, current:Coord, depth:usize)->usize{
    if current == start{
        return depth;
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
        return 0;
    }
    return count_loop_length(nodes, start, current, neighbour, depth+1);
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
    let (start_coord, nodes): (Coord, Vec<Vec<Node>>) = process_data_string(data_string);
    let start_node = &nodes[start_coord[0]][start_coord[1]];
    for neighbour in start_node.neighbours.iter(){
        let neighbour_node = &nodes[neighbour[0]][neighbour[1]];
        if start_node.are_connected(neighbour_node){
            let length = count_loop_length(&nodes, start_coord.clone(), start_coord, *neighbour, 1);
            if length > 0{
                return Some(length/2);
            }
        }
    }
    #[cfg(test)]{
        panic!("Tests all have a defined answer, so should never return None!")
    }
    None

}

mod tests{
    use super::*;
    #[test]
    fn adjacency(){
        let a = Node::new(Pipe::EtoW,[3,2], usize::MAX,usize::MAX);
        let b = Node::new(Pipe::NtoW,[3,3], usize::MAX,usize::MAX);
        println!("{:?} -> {:?}", a.pipe, b.pipe);
        assert!(a.are_connected(&b), "'-J' is a valid connection, but evaluates to 'false'!")
    }
    #[test]
    fn dir_reversing(){
        let [n,s,e,w] = [Dir::N, Dir::S, Dir::E, Dir::W];
        assert_eq!(n, s.reverse());    
        assert_eq!(s, n.reverse());
        assert_eq!(e, w.reverse());
        assert_eq!(w, e.reverse());    
    }
}
