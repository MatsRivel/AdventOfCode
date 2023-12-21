use std::{fs::read_to_string, fmt::Display};
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
    pub fn tilt_north(&mut self){ // Not efficient, but can be improved later if needed.
        for y in 0..self.ymax{
            for mut x in 1..self.xmax{
                while self.get(x,y).is_ball() && self.get(x-1, y).is_open(){
                    self.set(x, y, Tile::Open);
                    self.set(x-1,y,Tile::Ball);
                    if x == 1{
                        break;
                    }else{
                        x-=1;
                    }
                }
            }
        }
    }
    pub fn tilt_south(&mut self){ // Not efficient, but can be improved later if needed.
        for y in 0..self.ymax{
            for mut x in (0..self.xmax-1).rev(){
                while self.get(x,y).is_ball() && self.get(x+1, y).is_open(){
                    self.set(x, y, Tile::Open);
                    self.set(x+1,y,Tile::Ball);
                    if x == self.xmax-2{
                        break;
                    }else{
                        x+=1;
                    }
                }
            }
        }
    }
    pub fn tilt_west(&mut self){ // Not efficient, but can be improved later if needed.
        for x in 0..self.xmax{
            for mut y in 1..self.ymax{
                while self.get(x,y).is_ball() && self.get(x, y-1).is_open(){
                    self.set(x, y, Tile::Open);
                    self.set(x,y-1,Tile::Ball);
                    if y == 1{
                        break;
                    }else{
                        y-=1;
                    }
                }
            }
        }
    }
    pub fn tilt_east(&mut self){ // Not efficient, but can be improved later if needed.
        for x in 0..self.xmax{
            for mut y in (0..self.ymax-1).rev(){
                while self.get(x,y).is_ball() && self.get(x, y+1).is_open(){
                    self.set(x, y, Tile::Open);
                    self.set(x,y+1,Tile::Ball);
                    if y == self.ymax-2{
                        break;
                    }else{
                        y+=1;
                    }
                }
            }
        }
    }
    pub fn tilt_cycle(&mut self){
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
    pub fn current_load(&self)->usize{
        let mut rocks_by_col = (0..self.ymax).map(|y|{
            let mut row = (0..self.xmax)
                .map(|x| {
                    if self.get(x, y).is_ball(){
                        1
                    }else{
                        0
                    }
                })
                .collect::<Vec<usize>>();
            // for i in (1..self.xmax).rev(){
            //     row[i-1] += row[i];
            // }
            for i in (0..(self.xmax)){
                row[i] *= self.xmax - i;
            }
            row
            }).collect::<Vec<Vec<usize>>>();
        #[cfg(none)]{
            for j in 0..rocks_by_col[0].len(){
                for i in 0..rocks_by_col.len(){
                    let val = rocks_by_col[i][j];
                    if val > 9{
                        print!("{val} ");
                    }else{
                        print!(" {val} ");
                    }
                }
                println!();
            }
            println!();
        }
        rocks_by_col.iter().flat_map(|row| row).sum()

    }
}
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Tile{
    Ball,
    Wall,
    Open
}
impl Tile{
    pub fn new(c:char)->Option<Self>{
        match c{
            'O' => Some(Self::Ball),
            '#' => Some(Self::Wall),
            '.' => Some(Self::Open),
            _ => None
        }
    }
    pub fn is_ball(&self)->bool{
        match self{
            Tile::Ball => true,
            _ => false,
        }
    }
    pub fn is_wall(&self)->bool{
        match self{
            Tile::Wall => true,
            _ => false,
        }
    }
    pub fn is_open(&self)->bool{
        match self{
            Tile::Open => true,
            _ => false,
        }
    }
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Tile::Ball => write!(f,"O"),
            Tile::Wall => write!(f,"#"),
            Tile::Open => write!(f,"."),
        }
    }
}
pub fn process_data_string(data_string:String)->(Vec<Tile>,usize,usize){
    let chars = data_string.lines().map(|line| line.chars().collect::<Vec<char>>() ).collect::<Vec<Vec<char>>>();
    let xmax = chars.len();
    let ymax = chars[0].len();
    let tiles = chars.into_iter().flat_map(|line| line.into_iter().filter_map(|c| Tile::new(c))).collect::<Vec<Tile>>();
    (tiles,xmax,ymax)
}
pub fn main_1(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let (tile_vec,xmax,ymax) = process_data_string(data_string);
    let mut tiles = Matrix::new(tile_vec,xmax,ymax);
    #[cfg(none)]{
        println!("Initial state:");
        tiles.print_pattern();
    }
    tiles.tilt_north();
    #[cfg(none)]{
        println!("Tilted state:");
        tiles.print_pattern();
    }
    let current_load = tiles.current_load();
    Some(current_load)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
