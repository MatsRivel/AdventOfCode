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
#[derive(Clone,Copy,Debug)]
enum Dir{
    N,
    S,
    E,
    W
}

struct Tile{
    dirs: Vec<Dir>
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
        Self{dirs}
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
fn traverse_tiles(start: &Coord, current:Coord, tiles: &Vec<Vec<Tile>>, memory: &mut HashSet<Coord>, depth:usize)->Option<usize>{
    println!("Current: {current:?}, {}",memory.contains(&current));
    if current == *start{
        return Some(depth);
    }
    if memory.contains(&current){
        return None;
    }
    memory.insert(current.clone());
    let max_x = tiles.len();
    let max_y = tiles[0].len();
    let neighbours = tiles[current[0]][current[1]].get_neighbours(current, max_x, max_y);
    // Note to self: To prevent memory overflow, do not let the function check the previous current node.
    // Make a function that takes in a From coord and a To coord and returns a Dir based on what dir that went?
    // Then exclude that dir from neighbours.
    for neighbour in neighbours{
        if let Some(length) = traverse_tiles(start, neighbour, tiles, memory, depth+1){
            return Some(length);
        }
    }
    return None;
    

}
fn initialize_traverse_tiles(start_coord: &Coord, current: Coord, tiles: &Vec<Vec<Tile>>)->Option<usize>{
    let mut memory: HashSet<[usize; 2]> = HashSet::new();
    memory.insert(start_coord.clone());
    traverse_tiles(start_coord, current, tiles, &mut memory, 2)

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
    let max_x = tiles.len();
    let max_y = tiles[0].len();
    let neighbours: Vec<Coord> = tiles[start_coord[0]][start_coord[1]].get_neighbours(start_coord,max_x,max_y);
    for neighbour in neighbours.into_iter(){
        if let Some(length_of_path) = initialize_traverse_tiles(&start_coord, neighbour, &tiles){
            // We've found one loop leading back to start.
            // This should be the only loop.
            // Now we just measure the length of this loop, and cut it in half.
            println!(">>>>>>>>>>>>> {length_of_path} <<<<<<<<<<<<<<");
            return Some(length_of_path/2)
        }
    }
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
