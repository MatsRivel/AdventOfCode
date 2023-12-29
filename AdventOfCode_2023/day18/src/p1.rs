use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::{fs::read_to_string, collections::HashMap};
use std::cmp::{min,max};

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;

type FakeCoord = [i32;2];
type Coord = [usize;2];

struct Instruction{
    dir:Dir,
    dist:i32,
    colour:String
}
impl Instruction{
    fn new(line:&str)->Self{
        let [dir_str, num_str, colour_str]: [&str;3] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
        let dir = Dir::new(dir_str);
        let dist = num_str.parse::<i32>().unwrap();
        let colour = colour_str.strip_prefix("(").unwrap().strip_suffix(")").unwrap().to_string();
        Instruction { dir, dist, colour }
    }
}
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum TileOption{
    None,
    Outer,
    Inner
}
impl TileOption{
    fn as_char(&self)->char{
        match self{
            TileOption::None => ' ',
            TileOption::Outer => '#',
            TileOption::Inner => 'O',
        }
    }
    fn is_outer(&self)->bool{
        if &TileOption::Outer == self{
            true
        }else{
            false
        }
    }
    fn is_none(&self)->bool{
        if &TileOption::None == self{
            true
        }else{
            false
        }
    }
}
impl Display for TileOption{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.as_char())
    }
}
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Dir{
    Up,
    Down,
    Left,
    Right
}
impl Dir{
    fn new(s:&str)->Self{
        match s{
            "U" => Dir::Up,
            "D" => Dir::Down,
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => panic!("Invalid string \"{s}\" for enum Dir.")
        }
    }
    fn step_dir(&self)->FakeCoord{
        match self{
            Dir::Up     => [-1, 0],
            Dir::Down   => [ 1, 0],
            Dir::Left   => [ 0,-1],
            Dir::Right  => [ 0, 1],
        }
    }
}


#[derive(Clone)]
pub struct Matrix<T>
where T: Sized+Display{
    pub matrix: Vec<T>,
    pub xmax:usize,
    pub ymax:usize
}
impl Matrix<TileOption>{
    pub fn new(tiles:Vec<TileOption>,xmax:usize,ymax:usize)->Self{
        let matrix = tiles;
        Self{matrix,xmax,ymax}
    }
    pub fn new_empty(empty_val:TileOption,xmax:usize,ymax:usize)->Self{
        let tiles = vec![empty_val; xmax*ymax];
        let matrix: Vec<TileOption> = tiles;
        Self{matrix,xmax,ymax}
    }
    pub fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    pub fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    pub fn get(&self,x:usize,y:usize)->TileOption{
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx]
    }
    pub fn checked_get(&self,x:usize,y:usize)->Option<&TileOption>{
        let idx = self.coord_to_idx([x,y]);
        self.matrix.get(idx)
    }
    pub fn set(&mut self,x:usize,y:usize, val:TileOption){
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx] = val;
    }
    pub fn checked_set(&mut self,x:usize,y:usize, val:TileOption)->Option<()>{
        let idx = self.coord_to_idx([x,y]);
        self.matrix.get(idx)?;
        self.matrix[idx] = val;
        Some(())
    }
    pub fn checked_adjustment(&self, coord:[usize;2], adjustment:[i8;2])->Option<[usize;2]>{
        let xmax = self.xmax;
        let ymax = self.ymax;
        if (coord[0] == xmax-1 && adjustment[0] == 1) || (coord[1] == ymax-1 && adjustment[1] == 1) {
            return None;
        }else if (coord[0] == 0 && adjustment[0] == -1) || (coord[1] == 0 && adjustment[1] == -1) {
            return None;
        }
        let x = (coord[0] as i64) + adjustment[0] as i64;
        let y = (coord[1] as i64) + adjustment[1] as i64;
        Some([x as usize, y as usize])
    }
    pub fn neighbours(&self,x:usize, y:usize) -> Vec<Coord>{
        let mut output = vec![];
        for (i,j) in vec![(0,-1),(0,1),(-1,0),(1,0)].into_iter(){
                // We want to keep things withing bounds.
                let [nx, ny] = [x as i64 + i, y as i64 + j];
                if nx < 0 || ny < 0 || nx >= (self.xmax as i64)|| ny >= (self.ymax  as i64){
                    continue;
                }
                output.push([nx as usize,ny as usize]);
        }
        output
    }
    pub fn verified_edge(&self, x:usize,y:usize)->bool{
        // Find a tile [x,y] such that [x,y-1] != Outer and [x,y] == Outer and [x,y+1] == None.
        if self.checked_get(x, y) != Some(&TileOption::Outer){
            false
        }else if self.checked_get(x, y+1) != Some(&TileOption::None){
            false
        }else if y > 0 && Some(&TileOption::Outer) == self.checked_get(x,y-1){
            false
        }else{
            true
        }
    }
    pub fn is_outer(&self,x:usize,y:usize)->bool{
        if let Some(&TileOption::Outer) = self.checked_get(x, y){
            true
        }else{
            false
        }
    }
    pub fn is_none(&self,x:usize,y:usize)->bool{
        if let Some(&TileOption::None) = self.checked_get(x, y){
            true
        }else{
            false
        }
    }
}
fn process_data_string(data_string:String)->Vec<Instruction>{
    data_string
        .lines()
        .map(|line| {
            Instruction::new(line)
        }).collect::<Vec<Instruction>>()
}

fn make_pit_corners(instructions: &Vec<Instruction>)->HashMap<Coord,Coord>{
    let mut current = [0,0];
    let mut corners = Vec::with_capacity(instructions.len()+1);
    corners.push(current);
    for instruction in instructions.iter(){
        let step = instruction.dir.step_dir();
        let dist = instruction.dist;
        current[0] += step[0]*dist;
        current[1] += step[1]*dist;
        corners.push(current);
    }
    let min_corners = corners
        .iter()
        .fold([0;2], |acc, coord|{
            [ min(acc[0],coord[0]), min(acc[1], coord[1]) ]
        });
    
    let mut adj_corners = corners
        .into_iter()
        .map(|corner| {
            [corner[0] - min_corners[0], corner[1] - min_corners[1]]
        })
        .collect::<Vec<FakeCoord>>();
    
    // adj_corners.push( [-min_corners[0], -min_corners[1]]); // Adding a connection between 0 and the last known point.
    let output = adj_corners
        .iter()
        .zip(adj_corners.iter().skip(1))
        .map(|(from, to)| {
            ([from[0] as usize, from[1] as usize], [to[0] as usize, to[1] as usize])
        })
        .collect::<HashMap<Coord,Coord>>();
    output

}

fn get_pit_area(map:&HashMap<Coord,Coord>)->usize{
    let pos_max = map.iter().fold([0;2], |acc, (x,_)| [ max(acc[0],x[0]), max(acc[1], x[1]) ] );
    let [xmax,ymax] = [pos_max[0]+1, pos_max[1]+1];
    let mut grid = Matrix::new_empty(TileOption::None, xmax, ymax);
    #[cfg(not(test))]
    let file = File::create("output.txt").unwrap();
    #[cfg(not(test))]
    let mut file = LineWriter::new(file);
    for (from,to) in map.iter(){
        for x in min(from[0], to[0])..=(max(from[0],to[0])) {
            for y in min(from[1], to[1])..=(max(from[1],to[1])) {
                grid.checked_set(x, y, TileOption::Outer);
            }
        }
    }
    // Forwards:
    for x in 0..(grid.xmax){
        let mut left = 0;
        let mut right = left+1;
        while left < right && right < grid.ymax{
            #[cfg(not(test))]{
                println!("{x}, {left}, {right} ,{}\n{:?}, {:?}, {:?}",grid.ymax, grid.checked_get(x, left-1), grid.checked_get(x, left), grid.checked_get(x, left+1));
                println!("{:?}, {:?}, {:?}",grid.checked_get(x, right-1), grid.checked_get(x, right), grid.checked_get(x, right+1));
                println!();
            }
            while left < grid.ymax && !grid.verified_edge(x, left){ // Find first hole.
                left += 1;
                // println!("{x}, {left}, a");
            }
            while left < grid.ymax && !grid.verified_edge(x, left){ // Find end of the first hole.
                left += 1;
                // println!("b");
            }
            right = left+1;
            while right < grid.ymax-1 && !grid.is_outer(x, right){ // Now find the start of the next hole.
                right += 1;
                // println!("c");
            }
            while right < grid.ymax-1 && grid.is_outer(x, right){ // Now find the start of the next hole.
                right += 1;
                // println!("c");
            }

            while left < right{ // Fill inn the area between left and right.
                grid.checked_set(x,left,TileOption::Inner);
                left += 1;
                // println!("d");
            }
            while right < grid.ymax-1 && grid.is_outer(x, right){ // Find the end of the current hole.
                right += 1;
                // println!("e");
            }
            while right < grid.ymax-1 && !grid.verified_edge(x, right){ // Skip the next area, as it is not between two holes.
                right +=1;
                // println!("f");
            }
            left = right;
            right += 1;
        }
        
        #[cfg(test)]{
            for y in 0..grid.ymax{
                print!("{}",grid.get(x, y));
            }
        }
        #[cfg(test)]
        println!();
        #[cfg(not(test))]{
            let mut s = (0..grid.ymax).filter_map(|y| grid.checked_get(x,y) ).map(|tile| tile.as_char()).collect::<Vec<char>>();
            s.push('\n');
            let bytes = s.into_iter().map(|c| c as u8).collect::<Vec<u8>>();
            file.write_all(&bytes).unwrap();
            file.flush().unwrap();
        }
    }

    let counter = grid.matrix.into_iter().filter(|point| *point != TileOption::None).count();
    counter
}

fn get_pit_areav2(map:&HashMap<Coord,Coord>)->usize{
    let pos_max = map.iter().fold([0;2], |acc, (x,_)| [ max(acc[0],x[0]), max(acc[1], x[1]) ] );
    let [xmax,ymax] = [pos_max[0]+1, pos_max[1]+1];
    let mut grid = Matrix::new_empty(TileOption::None, xmax, ymax);
    for (from,to) in map.iter(){
        for x in min(from[0], to[0])..=(max(from[0],to[0])) {
            for y in min(from[1], to[1])..=(max(from[1],to[1])) {
                grid.checked_set(x, y, TileOption::Inner);
            }
        }
    }
    let mut to_visit = VecDeque::new();
    // Put every edge position into the queue.
    for x in 0..xmax{
        to_visit.push_back([x,0]);
        to_visit.push_back([x,grid.ymax-1]);
    }
    for y in 0..ymax{
        to_visit.push_back([0,y]);
        to_visit.push_back([grid.xmax-1,y]);
    }
    while let Some([x,y]) = to_visit.pop_front(){
        if !grid.get(x, y).is_none(){
            continue;
        }
        grid.set(x, y, TileOption::Outer);
        let neighbours = grid.neighbours(x,y).into_iter().filter(|[i,j]| grid.get(*i, *j).is_none());
        for n in neighbours{
            to_visit.push_back(n);
        }
    } 
    #[cfg(not(test))]
    let file = File::create("output.txt").unwrap();
    #[cfg(not(test))]
    let mut file = LineWriter::new(file);
    for x in 0..grid.xmax{
        #[cfg(not(test))]{
            let mut s = (0..grid.ymax).filter_map(|y| grid.checked_get(x,y) ).map(|tile| tile.as_char()).collect::<Vec<char>>();
            s.push('\n');
            let bytes = s.into_iter().map(|c| c as u8).collect::<Vec<u8>>();
            file.write_all(&bytes).unwrap();
            file.flush().unwrap();
        }
    }
    let count = grid.matrix.into_iter().filter(|element| !element.is_outer()).count();
    count

}
pub fn main_1(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let instructions = process_data_string(data_string);
    let map = make_pit_corners(&instructions);
    let count = get_pit_areav2(&map);
    Some(count)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
