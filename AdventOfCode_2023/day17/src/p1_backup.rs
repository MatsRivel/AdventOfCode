use std::{fs::read_to_string, fmt::Display, collections::{HashMap, VecDeque, HashSet}, f32::consts::E};
use priority_queue::PriorityQueue;
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Direction{
    Left,
    Right,
    Forward
}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Orientation{
    North,
    South,
    West,
    East
}impl Orientation{
    fn change_orientation(&self, dir:&Direction)->Self{
        match (self,dir){
            (Self::North, Direction::Left)    => Self::West,
            (Self::North, Direction::Right)   => Self::East,
            (Self::North, Direction::Forward) => Self::North,
            (Self::South, Direction::Left)    => Self::East,
            (Self::South, Direction::Right)   => Self::West,
            (Self::South, Direction::Forward) => Self::South,
            (Self::West,  Direction::Left)    => Self::South,
            (Self::West,  Direction::Right)   => Self::North,
            (Self::West,  Direction::Forward) => Self::West,
            (Self::East,  Direction::Left)    => Self::North,
            (Self::East,  Direction::Right)   => Self::South,
            (Self::East,  Direction::Forward) => Self::East
        }
    }
    fn to_coord_adjustment(&self)->[i8;2]{
        match self{
            Orientation::North => [-1, 0],
            Orientation::South => [ 1, 0],
            Orientation::West  => [ 0,-1],
            Orientation::East  => [ 0, 1]
        }
    }
    fn get_orientation(from_coord: [usize;2], to_coord: [usize;2])->Self{
        let diff = [from_coord[0] as i32 - to_coord[0] as i32, from_coord[1] as i32 - to_coord[1] as i32];
        match diff{
            [ 0,-1] => Orientation::North,
            [ 0, 1] => Orientation::South,
            [-1, 0] => Orientation::West,
            [ 1, 0] => Orientation::East,
            [ 0, 0] => Orientation::West, // TODO: This is a temp hack to allow the first step to be valid.
            _ => panic!("{from_coord:?} -> {to_coord:?}\nFunction should only be applied to adjacent coords!")
        }
    }
}
pub struct Matrix<T>
where T: Sized+Display{
    pub matrix: Vec<T>,
    pub xmax:usize,
    pub ymax:usize
}
impl Matrix<u8>{
    pub fn new(tiles:Vec<u8>,xmax:usize,ymax:usize)->Self{
        let matrix = tiles;
        Self{matrix,xmax,ymax}
    }
    pub fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    pub fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    pub fn get(&self,x:usize,y:usize)->u8{
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx]
    }
    pub fn checked_get(&self,x:usize,y:usize)->Option<&u8>{
        let idx = self.coord_to_idx([x,y]);
        self.matrix.get(idx)
    }
    pub fn set(&mut self,x:usize,y:usize, val:u8){
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx] = val;
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
    pub fn print_pattern(&self){
        for x in 0..self.xmax{
            for y in 0..self.ymax{
                if let Some(something) = self.checked_get(x,y){
                    print!("{} ",something);
                }
            }
            println!();
        }
        println!();
    }
    pub fn neighbours(&self,x:usize,y:usize) -> Vec<[usize;2]>{
        let mut output = vec![];
        for i in -1..=1{
            for j in -1..=1{
                // We don't want diagonals or the identity.
                if i == j || (i != 0 && j != 0){
                    continue;
                }
                let [nx, ny] = [x as i64 + i, y as i64 + j];
                if nx < 0 || ny < 0{
                    continue;
                }
                if nx >= self.xmax as i64 || ny >= self.ymax as i64{
                    continue;
                }
                output.push([nx as usize, ny as usize]);
            }
        }
        output
    }
    pub fn manhatten_distance(&self,idx_a:usize, idx_b:usize)->usize{
        let coord_a = self.idx_to_coord(idx_a);
        let coord_b = self.idx_to_coord(idx_b);
        let mut dist = 0;
        for i in 0..=1usize{
            if coord_a[i] > coord_b[i]{
                dist += coord_a[i] - coord_b[i];
            }else{
                dist += coord_b[i] - coord_a[i];
            }
        }
        dist
    }
}
impl Matrix<u32>{
    pub fn new(tiles:Vec<u32>,xmax:usize,ymax:usize)->Self{
        let matrix = tiles;
        Self{matrix,xmax,ymax}
    }
    pub fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    pub fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    pub fn get(&self,x:usize,y:usize)->u32{
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx]
    }
    pub fn checked_get(&self,x:usize,y:usize)->Option<&u32>{
        let idx = self.coord_to_idx([x,y]);
        self.matrix.get(idx)
    }
    pub fn set(&mut self,x:usize,y:usize, val:u32){
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx] = val;
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
    pub fn print_pattern(&self){
        for x in 0..self.xmax{
            for y in 0..self.ymax{
                if let Some(val) = self.checked_get(x,y){
                    if *val <10{
                        print!("0{} ",val);
                    }else{
                        print!("{} ",val);
                    }
                }
            }
            println!();
        }
        println!();
    }
    pub fn neighbours(&self,x:usize,y:usize) -> Vec<[usize;2]>{
        let mut output = vec![];
        for i in -1..=1{
            for j in -1..=1{
                // We don't want diagonals or the identity.
                if i == j || (i != 0 && j != 0){
                    continue;
                }
                let [nx, ny] = [x as i64 + i, y as i64 + j];
                if nx < 0 || ny < 0 || nx >= self.xmax as i64|| ny >= self.ymax  as i64{
                    continue;
                }
                output.push([nx as usize, ny as usize]);

            }
        }
        output
    }
    pub fn manhatten_distance(&self,idx_a:usize, idx_b:usize)->usize{
        let coord_a = self.idx_to_coord(idx_a);
        let coord_b = self.idx_to_coord(idx_b);
        let mut dist = 0;
        for i in 0..=1usize{
            if coord_a[i] > coord_b[i]{
                dist += coord_a[i] - coord_b[i];
            }else{
                dist += coord_b[i] - coord_a[i];
            }
        }
        dist
    }
}
pub fn process_data_string(data_string:String)->Matrix<u8>{
    let lines = data_string.lines().collect::<Vec<&str>>();
    let xmax = lines.len();
    let matrix = lines
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .filter_map(|c| {
                    c.to_digit(10)
                })
        }).map(|n| n as u8)
        .collect::<Vec<u8>>();
    let ymax = matrix.len()/xmax;
    Matrix::<u8>::new(matrix,xmax,ymax)
}

fn traverse_matrix(matrix:&Matrix<u8>)->u32{
    let path = astar(matrix).expect("There exists a solution. Panic is acceptable, as if no solution -> Poor implementation");
    #[cfg(test)]{
        for x in 0..matrix.xmax{
            for y in 0..matrix.ymax{
                if let Some(val) = matrix.checked_get(x,y){
                    let idx = matrix.coord_to_idx([x,y]);
                    if *val <10{
                        if path.contains(&idx){
                            print!("## ",);
                        }else{
                            print!("0{} ",val);
                        }
                    }else{
                        if path.contains(&idx){
                            print!("# ",);
                        }else{
                            print!("{} ",val);
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }
    let cost = path
        .into_iter()
        .map(|idx|{
            let [x,y] = matrix.idx_to_coord(idx);
            matrix.get(x, y) as u32
        })
        .sum();
    cost
}
struct DefaultMap{
    dict: HashMap<usize,usize>,
    default: usize
}
impl DefaultMap{
    fn new(default:usize)->Self{
        Self{dict:HashMap::<usize,usize>::new(), default}
    }
    fn contains_key(&self,key:&usize)->bool{
        self.dict.contains_key(key)
    }
    fn get(&self,key:&usize)->usize{
        if let Some(v) = self.dict.get(key){
            *v
        }else{
            self.default
        }
    }
    fn insert(&mut self, key:usize, val:usize){
        self.dict.insert(key,val);
    }
}
fn estimated_cost(matrix:&Matrix<u8>, a:usize, b:usize)->usize{
    matrix.manhatten_distance(a, b)
}
fn actual_cost(matrix:&Matrix<u8>, a:usize, b:usize)->Option<usize>{
    let [x,y] = matrix.idx_to_coord(b);
    if let Some(v) = matrix.checked_get(x, y){
        Some(*v as usize)
    }else{
        None
    }
}

fn reconstruct_path(came_from: &HashMap<usize,usize>,mut current:usize)->Vec<usize>{
    let mut output = vec![current];
    while let Some(&from) = came_from.get(&current){
        output.push(from);
        if from == current{
            break;
        }
        current = from;
    }
    output
}
fn astar(matrix:&Matrix<u8>)->Option<Vec::<usize>>{
    let start = 0;
    let goal = matrix.xmax*matrix.ymax-1;
    let mut seen = HashSet::new();
    let mut to_visit = PriorityQueue::new();
    let mut came_from = HashMap::<usize,usize>::new();
    // g_score(n) = Cost from start to n
    let mut g_score = DefaultMap::new(usize::MAX);
    g_score.insert(start,0);
    // f_score(n) = estimate of cost from start to goal through n.
    let mut f_score = DefaultMap::new(usize::MAX);
    f_score.insert(start, estimated_cost(matrix, start, goal));
    to_visit.push((start, start,Orientation::West),-1* (f_score.get(&start) as i64));
    seen.insert(start);
    let mut straight_ahead_counter = 1;
    #[cfg(test)]
    let mut counter = 0;
    while let Some(((prev, current, orientation), _)) = to_visit.pop(){
        #[cfg(test)]{
            println!();
            print!("{orientation:?} ");
            let coord = matrix.idx_to_coord(current);
            print!("{coord:?} ({})",g_score.get(&current));
            counter +=1;
            if counter > 13*13{
                panic!("Ran too long!");
            }
        }
        if current == goal{
            return Some(reconstruct_path(&came_from, current));
        }
        let [x,y] = matrix.idx_to_coord(current);
        let [prev_x, prev_y] = matrix.idx_to_coord(prev);
        let new_orientation = Orientation::get_orientation([prev_x,prev_y], [x,y]);
        if orientation == new_orientation{
            println!("{orientation:?} == {new_orientation:?}");
            print!("-->");
            straight_ahead_counter += 1;
        }else{
            println!("{orientation:?} =/= {new_orientation:?}");
            print!("^v");
            straight_ahead_counter = 1;
        }
        let neighbours = matrix.neighbours(x, y).into_iter().map(|coord| matrix.coord_to_idx(coord));
        for neighbour in neighbours{
            // Only allow 3 or fewer steps in a row in the same direction.
            let neighbour_coord = matrix.idx_to_coord(neighbour);
            let neighbour_orientation = Orientation::get_orientation([x,y], neighbour_coord);
            if neighbour_orientation == orientation && straight_ahead_counter == 2{
                continue;
            }
            let current_to_neighbour_cost = match actual_cost(matrix, current,neighbour){
                Some(v) => v,
                None => continue, // Neighbour idx out of bounds.
            };
            let tentative_g_score = g_score.get(&current) + current_to_neighbour_cost;
            if tentative_g_score < g_score.get(&neighbour){
                #[cfg(none)]{
                    let neighbour_coord = matrix.idx_to_coord(neighbour);
                    println!("Current: [{x},{y}], Neighbour: {neighbour_coord:?}")//, tentative_score: {tentative_g_score} ,g_score[neigh]: {:?}",g_score.get(&neighbour));
                }
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + estimated_cost(matrix, neighbour, goal));
                if !seen.contains(&neighbour){
                    seen.insert(neighbour);
                    to_visit.push((current, neighbour, neighbour_orientation), -1 * (f_score.get(&neighbour) as i64));
                }
            }
        }
    }
    return None;
}
pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let matrix = process_data_string(data_string);
    #[cfg(test)]{
        matrix.print_pattern();
    }
    let score = traverse_matrix(&matrix);
    return Some(score);

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn neighbour_test(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        // 13x13 matrix
        let matrix = process_data_string(data_string);
        let neighbours = matrix.neighbours(0, 12);
        let mut expected_neighbours = vec![[0,11], [1,12] ];
        for neighbour in neighbours.iter(){
            assert!(expected_neighbours.contains(neighbour),"Got a neighbour we did not expect! {neighbour:?}");
        }
        for expected in expected_neighbours.iter(){
            assert!(neighbours.contains(expected),"Missing an expected neighbour! {expected:?}");
        }
    }

}
