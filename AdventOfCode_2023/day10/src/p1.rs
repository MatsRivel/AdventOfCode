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
enum Pipe{
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
    fn to_dirs(&self)->Option<[Dir;2]>{
        match self{
            Pipe::NtoE => Some([Dir::N,Dir::E]),
            Pipe::NtoS => Some([Dir::N,Dir::S]),
            Pipe::NtoW => Some([Dir::N,Dir::W]),
            Pipe::EtoS => Some([Dir::E,Dir::S]),
            Pipe::EtoW => Some([Dir::E,Dir::W]),
            Pipe::StoW => Some([Dir::S,Dir::W]),
            Pipe::Missing => None,
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
struct Node{
    pipe: Pipe,
    coord: Coord,
    neighbours:Vec<Coord>
}
impl Node{
    fn new(pipe:Pipe, coord:Coord, xmax:usize,ymax:usize)->Self{
        let dirs = pipe.to_dirs().unwrap();
        let mut neighbours = vec![];
        for dir in dirs.into_iter(){
            let temp = match dir{
                Dir::N => coord.north(),
                Dir::S => coord.south(),
                Dir::E => coord.east(),
                Dir::W => coord.west(),
            };
            if let Some(dir) = temp{
                if dir[0] < xmax && dir[1] < ymax{
                    neighbours.push(dir);
                }
            }
        }
        Node{pipe,coord,neighbours}
    }
    fn are_connected(&self, other:&Self)->bool{
        let dir:Dir = [self.coord,other.coord].into();
        // The pipe in "self" must point in the direction we are going.
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
fn process_data_string(data_string:String)->(Coord, Vec<Vec<Node>>){
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
                Node::new(pipe,[x,y], todo!("Find max x and y before this point"))
                
            })
            .collect::<Vec<Tile>>()} )
        .collect::<Vec<Vec<Tile>>>();
    return (start_idx,tiles)
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
    let (start_coord, tiles): (Coord, Vec<Vec<Tile>>) = process_data_string(data_string);

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
