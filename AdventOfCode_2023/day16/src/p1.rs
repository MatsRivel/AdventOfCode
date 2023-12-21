use core::panic;
use std::{fs::read_to_string, fmt::Display, collections::{HashMap, VecDeque}};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Direction{
    North,
    South,
    East,
    West
}impl Direction{
    fn step(&self)->[i64;2]{
        match self{
            Direction::North => [-1, 0],
            Direction::South => [ 1, 0],
            Direction::East  => [ 0, 1],
            Direction::West  => [ 0,-1]
        }
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Splitter{
    Vertical,
    Horizontal
}impl Splitter{
    fn new(c:char)->Self{
        match c{
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            _ => panic!("Invalid char {c} to convert into Splitter.")  
        }
    }
    fn split(&self, dir: &Direction)->Vec<Direction>{
        match self{
            Splitter::Vertical => {
                match dir{
                    Direction::North | Direction::South=> vec![*dir],
                    Direction::East | Direction::West=> vec![Direction::North, Direction::South],
                }
            },
            Splitter::Horizontal => {
                match dir{
                    Direction::North | Direction::South=> vec![Direction::East, Direction::West],
                    Direction::East | Direction::West=> vec![*dir],
                }
            },
        }
    }
}
impl Display for Splitter{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Vertical => write!(f,"|"),
            Self::Horizontal => write!(f,"-"),
            
        }
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Mirror{
    ForewardLean,
    BackwardLean
}impl Mirror{
    fn new(c:char)->Self{
        match c{
            '/' => Self::ForewardLean,
            '\\' => Self::BackwardLean,
            _ => panic!("Invalid char {c} to convert into Mirror.")  
        }
    }
    fn reflection(&self, dir:&Direction)->Direction{
        match self{
            Mirror::ForewardLean => {
                match dir{
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East  => Direction::North,
                    Direction::West  => Direction::South,
                }
            },
            Mirror::BackwardLean => {
                match dir{
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East  => Direction::South,
                    Direction::West  => Direction::North,
                }
            },
        }
    }
}
impl Display for Mirror{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::ForewardLean => write!(f,"/"),
            Self::BackwardLean => write!(f,r"\"),
            
        }
    }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Tile{
    Empty,
    Mirror(Mirror),
    Splitter(Splitter)
}impl Tile{
    fn new(c:char)->Self{
        match c{
            '/' | '\\'  => Tile::Mirror(Mirror::new(c)),
            '-' | '|'   => Tile::Splitter(Splitter::new(c)),
            '.'         => Tile::Empty,
            _ => panic!("Char '{c}' is not a valid Tile")
        }
    }fn is_empty(&self)->bool{
        match self{
            Tile::Empty => true,
            _ => false
        }
    }
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Tile::Empty => write!(f,"."),
            Tile::Mirror(mirror) => write!(f,"{mirror}"),
            Tile::Splitter(splitter) =>  write!(f,"{splitter}"),
        }
    }
}

pub struct Matrix<T>
where T: Sized+Display{
    matrix: Vec<T>,
    pub xmax:usize,
    pub ymax:usize
}
impl Matrix<Tile>{
    pub fn new(tiles:Vec<Tile>,xmax:usize,ymax:usize)->Self{
        let matrix = tiles;
        Self{matrix,xmax,ymax}
    }
    pub fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    pub fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    pub fn get(&self,x:usize,y:usize)->Tile{
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx]
    }
    pub fn set(&mut self,x:usize,y:usize, val:Tile){
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx] = val;
    }
    pub fn print_pattern(&self){
        for x in 0..self.xmax{
            for y in 0..self.ymax{
                print!("{} ",self.get(x, y));
            }
            println!();
        }
        println!();
    }
}

fn process_data_string(data_string:String)->Matrix<Tile>{
    let lines = data_string.lines().collect::<Vec<&str>>();
    let xmax = lines.len();
    let matrix = lines
        .into_iter()
        .flat_map(|line| {
            line.chars().map(|c| Tile::new(c))
        }).collect::<Vec<Tile>>();
    let ymax = matrix.len() / xmax;
    Matrix{ matrix, xmax, ymax }
}

fn step_coord(coord:[usize;2], adjustment: [i64;2], xmax:usize, ymax:usize) -> Option<[usize;2]>{
    if (coord[0] == xmax-1 && adjustment[0] == 1) || (coord[1] == ymax-1 && adjustment[1] == 1) {
        return None;
    }else if (coord[0] == 0 && adjustment[0] == -1) || (coord[1] == 0 && adjustment[1] == -1) {
        return None;
    }
    let x = (coord[0] as i64) + adjustment[0];
    let y = (coord[1] as i64) + adjustment[1];
    Some([x as usize, y as usize])

}
fn recursive_beam_path(idx: usize, dir:Direction, tiles: &Matrix<Tile>, mut seen: &mut HashMap<usize,Vec<Direction>>){
    if let Some(entry) = seen.get_mut(&idx){
        if entry.contains(&dir){ // We've already seen this tile and gone this way.
            return;
        }
        entry.push(dir);
    }else{
        seen.insert(idx, vec![dir]);
    }
    let [x,y] = tiles.idx_to_coord(idx);
    let [dx,dy] = dir.step();
    let [new_x,new_y] = match step_coord([x,y], [dx,dy], tiles.xmax, tiles.ymax){
        Some(v) => v,
        None => return // No need to proceed if tile is out of bounds.
    };
    let next_idx = tiles.coord_to_idx([new_x,new_y]);
    let next_tile = tiles.get(new_x, new_y);
    match next_tile{
        Tile::Empty => recursive_beam_path(next_idx, dir, tiles, seen),
        Tile::Mirror(mirror) => {
            let next_dir = mirror.reflection(&dir);
            recursive_beam_path(next_idx, next_dir, tiles, seen);
        },
        Tile::Splitter(splitter) => {
            let next_dirs = splitter.split(&dir);
            for next_dir in next_dirs.into_iter(){
                recursive_beam_path(next_idx, next_dir, tiles, seen);
            }
        },
    }
}

fn non_recursive_beam_path(tiles:&Matrix<Tile>,seen:&mut HashMap<usize,Vec<Direction>>){
    let initial_idx = 0;
    let initial_dir = Direction::East;
    let mut queue = VecDeque::<(usize,Direction)>::new();
    queue.push_back((initial_idx,initial_dir));
    while let Some((idx,dir)) = queue.pop_front(){
        let [x,y]= tiles.idx_to_coord(idx);
        let tile = tiles.get(x, y);
        if let Some(seen_dirs) = seen.get_mut(&idx){
            if seen_dirs.contains(&dir){// && tile.is_empty(){ // This was not the bug.
                continue; // We've allready seen this combination of position and direction.
            }else{
                seen_dirs.push(dir);
            }
        }else{
            seen.insert(idx, vec![dir]);
        }
        #[cfg(test)]
        println!("[{x},{y}]: {tile}");
        let dirs = match tile{
                Tile::Empty => vec![dir],
                Tile::Mirror(mirror) => {
                    let next_dir = mirror.reflection(&dir);
                    vec![next_dir]
                },
                Tile::Splitter(splitter) => {
                    let next_dirs = splitter.split(&dir);
                    next_dirs
                }
        };
        for next_dir in dirs.into_iter(){
            let [dx,dy] = next_dir.step();
            if let Some([new_x, new_y]) = step_coord([x,y], [dx,dy], tiles.xmax, tiles.ymax){
                let next_idx = tiles.coord_to_idx([new_x,new_y]);
                queue.push_back((next_idx,next_dir));            
            }
        }
    }
}

fn beam_path(tiles: &Matrix<Tile>)->usize{
    let mut seen = HashMap::<usize,Vec<Direction>>::new();
    // recursive_beam_path(0, Direction::East, &tiles, &mut seen);
    non_recursive_beam_path(tiles,&mut seen);
    #[cfg(test)]{
        println!();
        for x in 0..tiles.xmax{
            for y in 0..tiles.ymax{
                let idx = tiles.coord_to_idx([x,y]);
                if seen.contains_key(&idx){
                    print!("#");
                } else{
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    // panic!();
    let total_energized = seen.into_iter().count();
    total_energized

}

pub fn main_1(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let tiles = process_data_string(data_string);
    let total_energized = beam_path(&tiles);

    Some(total_energized)

}

#[cfg(test)]
    mod tests{
    use std::time::Instant;

    use super::*;

    #[test]
	fn part1_dummy_custom1(){
        let file_name = r"src\dummy_custom1.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 114;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }
    }

}
