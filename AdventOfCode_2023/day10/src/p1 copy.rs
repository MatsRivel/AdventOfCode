use std::fs::read_to_string;
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

#[derive(Clone,Copy,Debug)]
enum Tile{
    Ground,
    Start,
    Pipe(Pipe)
}
impl Tile{
    fn new(c:char)->Self{
        match c{
            'S' => Self::Start,
            '.' => Self::Ground,
            other => Self::Pipe(Pipe::new(other))
        }
    }
}
#[derive(Clone,Copy,Debug)]
enum Dir{
    N,
    S,
    E,
    W
}
#[derive(Clone,Copy,Debug)]
enum Pipe{
    NtoE,    
    NtoS,
    NtoW,
    EtoS,
    EtoW,
    StoW,
    Invalid
}
impl Pipe{
    fn new(c:char)->Self{
        match c{
            '|' => Self::NtoS,
            '-' => Self::EtoW,
            'L' => Self::NtoW,
            'J' => Self::NtoW,
            '7' => Self::StoW,
            'F' =>  Self::EtoS,
            _ => panic!()
        }
    }
    fn get_dirs(&self) -> [Dir;2]{
        match self{
            Pipe::NtoE => [Dir::N,Dir::E],
            Pipe::NtoS => [Dir::N,Dir::S],
            Pipe::NtoW => [Dir::N,Dir::W],
            Pipe::EtoS => [Dir::E,Dir::S],
            Pipe::EtoW => [Dir::E,Dir::W],
            Pipe::StoW => [Dir::S,Dir::W],
            Pipe::Invalid => todo!("Should never happen"),
        }
    }
}
impl From<[Dir;2]> for Pipe{
    fn from(value: [Dir;2]) -> Self {
        match value{
            [Dir::N, Dir::E] | [Dir::E, Dir::N] => Self::NtoE,
            [Dir::N, Dir::S] | [Dir::S, Dir::N] => Self::NtoS,
            [Dir::N, Dir::W] | [Dir::W, Dir::N] => Self::NtoW,
            [Dir::S, Dir::E] | [Dir::E, Dir::S] => Self::EtoS,
            [Dir::S, Dir::W] | [Dir::W, Dir::S]=> Self::StoW,
            [Dir::E, Dir::W] | [Dir::W, Dir::E]=> Self::EtoW,
            _ => Self::Invalid,
        }
    }
}
#[derive(Clone,Copy,Debug)]
struct Node{
    coords: Coord,
    pipe: Pipe,
    neighbours: [Option<Coord>;2]
}
impl Node{
    fn new(&self,coords: Coord, pipe:Pipe)->Self{
        let neighbours = match self.pipe{
            Pipe::NtoE => [coords.north(), coords.east() ],
            Pipe::NtoS => [coords.north(), coords.south()],
            Pipe::NtoW => [coords.north(), coords.west() ],
            Pipe::EtoS => [coords.east(),  coords.south()],
            Pipe::EtoW => [coords.east(),  coords.west() ],
            Pipe::StoW => [coords.south(), coords.west() ],
            Pipe::Invalid => panic!("Should never exist. Only an option for satisfying the \"From\" trait")
        };
        Self { coords, pipe, neighbours }
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
                let tile = Tile::new(c);
                if let Tile::Start = tile{
                    start_idx = [x,y]
                }
                tile
            })
            .collect::<Vec<Tile>>()} )
        .collect::<Vec<Vec<Tile>>>();
    return (start_idx,tiles)
}

fn traverse_tiles(tiles: &Vec<Vec<Tile>>, start:&Coord, current:&Coord,last:[&usize;2])->Option<Coord>{
    todo!()
}
fn initialize_traverse_tiles(tiles: &Vec<Vec<Tile>>, start:&Coord, current:&Coord,last:[&usize;2])->Option<Coord>{
    // First we get all possible neighbours around the starting coordinates.
    let neighbours = (-1..=1i64)
        .zip( -1..=1i64 )
        .filter_map(|(x,y)|{
            if start[0] == 0 && x == -1{
                None
            }else if start[1] == 0 && y == -1{
                None
            }else if start[0] == tiles.len()-1 && x == 1{
                None
            }else if start[1] == tiles[0].len()-1 && y == 1{
                None
            }else if x == 0 && y == 0{
                None
            }else{
                Some([(start[0] as i64 + x) as usize, (start[1] as i64 +y) as usize])
            }
        });
    
    for neighbour in neighbours{
        // Then we check that the neighbour has a pipe that connects to the start tile.
        if let Tile::Pipe(pipe) =  tiles[neighbour[0]][neighbour[1]]{
            let dirs = pipe.get_dirs();
            if let Some(other_neighbour) = traverse_tiles(tiles, start, current, last){
                if let Tile::Pipe(neihgbour_pipe) = tiles[other_neighbour[0]][neighbour[1]]{
                    let neighbour_dirs = neihgbour_pipe.get_dirs();
                    // Now we have the directions 
                }
            }
        }


    }
    todo!()
    
}
pub fn main_1(file_name:&str)->Option<i32>{
    /*
    Plan:
    Define loop. (Continue until you find an existing node?)
    Follow loop all the way through.
    Cut that distance in half.
    Done.
     */
    let data_string = read_to_string(file_name).unwrap();
    let (start_coord, tiles) = make_tiles(data_string);

    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
