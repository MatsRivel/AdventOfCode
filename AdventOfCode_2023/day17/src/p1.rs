use std::{fs::read_to_string, fmt::Display, collections::{HashMap, VecDeque, HashSet}, f32::consts::E};
use priority_queue::PriorityQueue;

#[derive(Eq,PartialEq,Debug,Hash)]
enum Orientation{
    North,
    South,
    East,
    West,
    None
}
impl Orientation{
    fn new(from:[usize;2], to:[usize;2]) -> Self{
        let diff = [to[0] as i64- from[0] as i64, to[1] as i64 - from[1]  as i64 ];
        match diff{
            [-1, 0] => Orientation::North,
            [ 1, 0] => Orientation::South,
            [ 0,-1] => Orientation::West,
            [ 0, 1] => Orientation::East,
            _ => Orientation::None
        }
    }
    fn reverse(&self) -> Self{
        match self{
            Orientation::North => Orientation::South,
            Orientation::South => Orientation::North,
            Orientation::East => Orientation::West,
            Orientation::West => Orientation::East,
            Orientation::None => Orientation::None,
        }
    }
}
impl Display for Orientation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Orientation::North => write!(f,"N"),
            Orientation::South => write!(f,"S"),
            Orientation::East  => write!(f,"E"),
            Orientation::West  => write!(f,"W"),
            Orientation::None  => write!(f,"X"),
        }
    }
}

pub struct Matrix<T>
where T: Sized+Display{
    pub matrix: Vec<T>,
    pub xmax:usize,
    pub ymax:usize
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
    pub fn neighbours(&self,idx:usize) -> Vec<usize>{
        let [x,y]= self.idx_to_coord(idx);
        let mut output = vec![];
        for (i,j) in vec![(0,-1),(0,1),(-1,0),(1,0)].into_iter(){
                // We want to keep things withing bounds.
                let [nx, ny] = [x as i64 + i, y as i64 + j];
                if nx < 0 || ny < 0 || nx >= (self.xmax as i64)|| ny >= (self.ymax  as i64){
                    continue;
                }
                let neighbour = self.coord_to_idx([nx as usize,ny as usize]);
                output.push(neighbour);
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

pub fn process_data_string(data_string:String)->Matrix<u32>{
    let lines = data_string.lines().collect::<Vec<&str>>();
    let xmax = lines.len();
    let matrix = lines
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .filter_map(|c| {
                    c.to_digit(10)
                })
        }).map(|n| n as u32)
        .collect::<Vec<u32>>();
    let ymax = matrix.len()/xmax;
    Matrix::<u32>::new(matrix,xmax,ymax)
}

fn traverse_matrix(matrix:&Matrix<u32>)->u32{
    let path = astar(matrix).expect("There exists a solution. Panic is acceptable, as if no solution -> Poor implementation");
    #[cfg(test)]{
        {
        for x in 0..matrix.xmax{
            for y in 0..matrix.ymax{
                if let Some(val) = matrix.checked_get(x,y){
                    print!("{} ",val);
                }
            }
            println!();
        }
        println!();
        }

        let temp_mat: Matrix<u32> = Matrix::new(vec![],13,13);// Only for converting idx to coord
        let path_map = path
            .iter()
            .zip( path.iter().skip(1) )
            .map(|(from,to)|{
                let to_coord = temp_mat.idx_to_coord(*from);
                let from_coord = temp_mat.idx_to_coord(*to);
                let ori = Orientation::new(from_coord,to_coord);
                (*from,ori)
            }).collect::<HashMap<usize,Orientation>>();
        {
        for x in 0..matrix.xmax{
            for y in 0..matrix.ymax{
                if let Some(val) = matrix.checked_get(x,y){
                    let idx = matrix.coord_to_idx([x,y]);
                    if let Some(ori) = path_map.get(&idx){
                        print!("{ori} ",);
                    }else{
                        print!("{} ",val);
                    }
                }
            }
            println!();
        }
        println!();
        }
    }
    let cost = path
        .into_iter()
        .map(|idx|{
            let [x,y] = matrix.idx_to_coord(idx);
            matrix.get(x, y)
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
fn estimated_cost(matrix:&Matrix<u32>, a:usize, b:usize)->usize{
    matrix.manhatten_distance(a, b)
}
fn actual_cost(matrix:&Matrix<u32>, a:usize, b:usize)->Option<usize>{
    let [x,y] = matrix.idx_to_coord(b);
    if let Some(v) = matrix.checked_get(x, y){
        Some(*v as usize)
    }else{
        None
    }
}
fn reconstruct_path(matrix: &Matrix<u32>,mut came_from: HashMap<usize,usize>,mut current:usize)->Vec<usize>{
    let mut output = vec![current];
    println!("{:?}", matrix.idx_to_coord(current));
    while let Some(&from) = came_from.get(&current){
        println!("-> {:?}", matrix.idx_to_coord(from));
        came_from.remove(&current);
        output.push(from);
        if from == current{
            break;
        }
        current = from;
    }
    output
}
fn astar(matrix:&Matrix<u32>)->Option<Vec::<usize>>{
    let start = 0;
    let goal = matrix.xmax*matrix.ymax-1;
    let mut seen = HashSet::new();
    let mut to_visit = PriorityQueue::new();
    let mut came_from = HashMap::<usize,usize>::new();
    #[cfg(test)]
    println!("xmax: {:?}, ymax: {:?}", matrix.xmax, matrix.ymax);
    // g_score(n) = Cost from start to n
    let mut g_score = DefaultMap::new(usize::MAX);
    g_score.insert(start,0);

    // f_score(n) = estimate of cost from start to goal through n.
    let mut f_score = DefaultMap::new(usize::MAX);
    f_score.insert(start, estimated_cost(matrix, start, goal));
    to_visit.push( (start,Orientation::None, 0),-1* (f_score.get(&start) as i64));
    seen.insert(start);
    #[cfg(test)]
    let mut counter = 0;
    while let Some(((current, prev_ori, straight_ahead_count), _fscore)) = to_visit.pop(){
        #[cfg(none)]{
            if counter > 150{
                // panic!("Running for too long!");
                return Some(reconstruct_path(matrix, came_from.clone(), current)); // Temp check to see waht is outputted when failing.
            }else{
                counter += 1;
            }
        }
        // seen.remove(&current);
        println!("{current:?}");
        if current == goal{
            #[cfg(test)]
            println!("Current == goal -> Reconstructing and returning");
            return Some(reconstruct_path(matrix, came_from.clone(), current));
        }
        let neighbours = matrix
            .neighbours(current)
            .into_iter()
            .filter_map(|neighbour|{
                // Check the orientation after the move:
                let current_coord = matrix.idx_to_coord(current);
                let neighbour_coord = matrix.idx_to_coord(neighbour);
                let n_ori = Orientation::new(current_coord,neighbour_coord);
                if n_ori.reverse() == prev_ori{ // Avoid taking steps directly backwards. TODO: Does it make sense to do after all?
                    None
                }else if n_ori == prev_ori{ // We're going in the same direction we went last time as well.
                    Some((neighbour, n_ori, straight_ahead_count+1))
                }else{ // We're turning either left or right.
                    Some((neighbour, n_ori, 0))
                }
            }).collect::<Vec<(usize,Orientation, u8)>>();
        #[cfg(test)]
        println!("Therea are {} neighbours of point {current:?}: {neighbours:?}", neighbours.len());
        seen.remove(&current);
        for (neighbour, n_ori, neighbour_straight_count) in neighbours.into_iter(){
            #[cfg(test)]
            println!("Neighbour: {neighbour}");
            // Only allow 3 or fewer steps in a row in the same direction.
            let adjustment;
            if neighbour_straight_count > 1{
                // #[cfg(test)]
                // println!("Too many straight moves. Skipping this neighbour.");
                // adjustment = usize::MAX;
                continue;
            }else{
                adjustment = 0;
            }
            let current_to_neighbour_cost = actual_cost(matrix, current,neighbour).unwrap() + adjustment;
            let tentative_g_score = g_score.get(&current) + current_to_neighbour_cost;
            if tentative_g_score < g_score.get(&neighbour){
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + estimated_cost(matrix, neighbour, goal));
                if !seen.contains(&neighbour){
                    seen.insert(neighbour);
                    to_visit.push((neighbour, n_ori, neighbour_straight_count), -1 * (f_score.get(&neighbour) as i64));
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
    fn neighbour_test_v1(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        // 13x13 matrix
        let matrix = process_data_string(data_string);
        let neighbours = matrix.neighbours(12);
        let expected_neighbours = vec![11, 24 ];
        for neighbour in neighbours.iter(){
            assert!(expected_neighbours.contains(neighbour),"Got a neighbour we did not expect! {neighbour:?}");
        }
        for expected in expected_neighbours.iter(){
            assert!(neighbours.contains(expected),"Missing an expected neighbour! {expected:?}");
        }
    }

    #[test]
    fn neighbour_test_v2(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        // 13x13 matrix
        let matrix = process_data_string(data_string);
        println!("xmax: {:?}, ymax: {:?}", matrix.xmax, matrix.ymax);
        let neighbours = matrix.neighbours(0);
        let expected_neighbours = vec![1, 13 ];
        for neighbour in neighbours.iter(){
            assert!(expected_neighbours.contains(neighbour),"Got a neighbour we did not expect! {neighbour:?}\n>>{neighbours:?}");
        }
        for expected in expected_neighbours.iter(){
            assert!(neighbours.contains(expected),"Missing an expected neighbour! {expected:?}\n>>{neighbours:?}");
        }
    }
    #[test]
    fn coord_to_idx_test(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        // 13x13 matrix
        let matrix = process_data_string(data_string);
        println!("xmax: {:?}, ymax: {:?}", matrix.xmax, matrix.ymax);
        assert_eq!(matrix.coord_to_idx([0,0]), 0);
        assert_eq!(matrix.coord_to_idx([0,1]), 1);
        assert_eq!(matrix.coord_to_idx([0,2]), 2);
        assert_eq!(matrix.coord_to_idx([1,0]),13);
        assert_eq!(matrix.coord_to_idx([1,1]),14);

    }

}
