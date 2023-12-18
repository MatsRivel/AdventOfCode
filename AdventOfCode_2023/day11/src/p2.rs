use std::{fs::read_to_string, arch::x86_64};

struct Matrix{
    mat: Vec<[usize;2]>,
    xmax:usize,
    ymax:usize
}
impl Matrix{
    fn new()->Self{
        todo!()
    }
    fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    fn get(&self,x:usize,y:usize)->[usize;2]{
        let idx = self.coord_to_idx([x,y]);
        self.mat[idx]
    }
    fn set(&mut self,x:usize,y:usize, val:[usize;2]){
        let idx = self.coord_to_idx([x,y]);
        self.mat[idx] = val;
    }
    fn flip(&mut self){
        for idx in 0..=(self.xmax*self.ymax){
            let [x,y] = self.idx_to_coord(idx);
            let [a,b] = self.get(x, y);
            self.set(x, y, [b,a]);
        }
        // Sort it so that it makes sense to print again.
        self.mat.sort()
    }
}

fn read_galaxy(data_string:String)->(Matrix,usize,usize){
    let mut xmax = 0;
    let mut ymax = 0;
    let galaxy_vec = data_string.lines()
        .enumerate()
        .flat_map(|(x,line)| {
            line.char_indices()
                .filter_map(|(y,c)| {
                    xmax = x;
                    ymax = y;
                    match c{
                        '#' => Some([x, y]),
                        _ => None
                    }
                }).collect::<Vec<[usize;2]>>()
        }).collect::<Vec<[usize;2]>>();
    let galaxy = Matrix{mat: galaxy_vec, xmax, ymax};
    (galaxy,xmax,ymax)
}
fn expand_galaxy(mut galaxy: Matrix,xmax:usize,ymax:usize, expansion_rate: usize)->(Matrix,usize,usize){
    // For each row, check how many elements are in it.
    let rows_count = {
        galaxy
            .mat
            .iter()
            .fold(vec![0;xmax+1],|mut acc,val|{
                acc[val[0]] += 1;
                acc
            })
    };
    // For each col, check how many elements are in it.
    let cols_count = {
        galaxy
            .mat
            .iter()
            .fold(vec![0;ymax+1],|mut acc,val|{
                acc[val[1]] += 1;
                acc
            })
    };
    #[cfg(test)]{
        println!("row_count: {:?}", rows_count);
        println!("cols_count: {:?}", cols_count);
    }
    // Summarize the adjustment for each row/col
    let x_adjustment = {
        rows_count
            .into_iter()
            .enumerate()
            .fold(vec![0;xmax+1],|mut acc, (idx, count)|{
                if idx != 0{
                    acc[idx] = acc[idx-1];
                }
                if count == 0{
                    acc[idx] += std::cmp::max(expansion_rate-1,1);
                }
                acc
            })
    };

    let y_adjustment = {
        cols_count
            .into_iter()
            .enumerate()
            .fold(vec![0;ymax+1],|mut acc, (idx, count)|{
                if idx != 0{
                    acc[idx] = acc[idx-1];
                }
                if count == 0{
                    acc[idx] += std::cmp::max(expansion_rate-1,1);
                }
                acc
            })
    };
    galaxy.mat.iter_mut().for_each(|coord| {
        let new_x = coord[0]+x_adjustment[coord[0]];
        let new_y = coord[1]+y_adjustment[coord[1]];
        *coord = [new_x,new_y];
    }); 
    #[cfg(test)]{
        println!("x_adjustment: {:?}", x_adjustment);
        println!("y_adjustment: {:?}", y_adjustment);
    }
    (galaxy,xmax + x_adjustment.last().unwrap(), ymax + y_adjustment.last().unwrap())
}

fn distance_between_pairs(start: [usize;2], end:[usize;2])->usize{
    let [x1,y1] = [start[0], start[1]];
    let [x2,y2] = [end[0], end[1]];
    let mut total_diff = 0;
    if x1 > x2{
        total_diff += x1-x2;
    }else{
        total_diff += x2-x1;
    }
    if y1 > y2{
        total_diff += y1-y2;
    }else{
        total_diff += y2-y1;
    }
    total_diff as usize
}
pub fn main_2(file_name:&str,expansion_rate: usize)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let (raw_galaxy,xmax,ymax) = read_galaxy(data_string);
    let (mut galaxy,xmax,ymax) = expand_galaxy(raw_galaxy,xmax,ymax,expansion_rate);
    let mut total_dist = 0;
    galaxy.mat.sort();
    for (idx_a,pos_a) in galaxy.mat.iter().enumerate(){
        for (idx_b,pos_b) in galaxy.mat.iter().enumerate(){
            if idx_a >= idx_b{
                continue;
            }
            let dist = distance_between_pairs(*pos_a,*pos_b);
            #[cfg(test)]
            println!("{pos_a:?}  \t-> {pos_b:?} \t= {dist}");
            total_dist += dist;
        }
    }
    Some(total_dist)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
