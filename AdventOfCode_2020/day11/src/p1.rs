use std::{fs::read_to_string, fmt::Display};
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Tile{
    Seat(bool),
    Floor
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Tile::Seat(true) => write!(f,"#"),
            Tile::Seat(false) => write!(f,"L"),
            Tile::Floor => write!(f,"."),
        }
    }
}
pub struct Map{
    pub n_rows:usize,
    pub n_cols:usize,
    pub data:Vec<Tile>,
    pub recent_change:bool
}

fn string_to_tiles(s:&str) -> Vec<Tile>{
    s.lines()
    .map(|line|{
        line.chars().map(|c|{
            match c{
                'L' => Tile::Seat(false),
                '.' => Tile::Floor,
                '#' => Tile::Seat(true),
                _ => panic!("Invalid tile! {c}")
            }
        })
    }).fold(vec![], |mut acc, v_iter|{
        for v in v_iter{
            acc.push(v);
        }
        acc
    })
}
impl Map{
    pub fn new(data_string:&str) -> Self{
        let mut n_cols= 0;
         // Extract the column number from the first line, then break out to not iter unnecessarily.
        for line in data_string.lines(){
            #[cfg(None)]{ println!("{line}") }
            n_cols = line.len();
            break;
        }
        let data = string_to_tiles(data_string);
        let n_rows = data.len()/n_cols;
        Self{n_rows, n_cols, data, recent_change:false}
    }
    fn occupied_neighbours_list(&self)->Vec<usize>{
        // Gets the number of occupied tiles for each neighbour
        let occupied_count = self.data
            .iter()
            .enumerate()
            .map(|(i,t)|{
                if let Tile::Floor = t{
                    0
                }else{
                    let neighbors = Map::get_tile_neighbours(i, self.n_rows, self.n_cols);
                    let n_occupied_neighbours = neighbors
                        .iter()
                        .fold(0,|acc,n|{
                            if let Some(Tile::Seat(true)) = self.get_tile(*n){
                                acc+1
                            }else{
                                acc
                            }
                        });
                    n_occupied_neighbours
                }

            }).collect::<Vec<usize>>();
        occupied_count
    }
    fn update_tiles(&mut self){
        let occupied_count = self.occupied_neighbours_list();
        #[cfg(none)]
        {
            Map::print_custom_map(&occupied_count,self.n_cols,self.n_rows);
            println!();
        }
        // Does the updating:
        self.recent_change = false;
        self.data.iter_mut()
            .enumerate()
            .for_each(|(i,t)|{
                let n_neighbours = occupied_count[i];
                match t{
                    Tile::Seat(is_taken) => {
                        match (is_taken, n_neighbours){
                            (false, 0) => *t = {
                                self.recent_change = true;
                                Tile::Seat(true)
                            },
                            (true, 4..) => {
                                self.recent_change = true;
                                *t = Tile::Seat(false)
                            },
                            _ => ()
                        }
                    },
                    Tile::Floor => (),
                }
            });
    }
    pub fn get_tile(&self, tile_idx:usize) -> Option<Tile>{
        if tile_idx >= 0 && tile_idx < self.data.len(){
            Some(self.data[tile_idx])
        }else{
            None
        }
    }
    fn get_mut_tile(&mut self, tile_idx:usize) -> Option<&mut Tile>{
        if tile_idx >= 0 && tile_idx < self.data.len(){
            Some(&mut self.data[tile_idx])
        }else{
            None
        }
    }
    pub fn get_tile_coords(tile_idx:usize, n_cols:usize) -> [usize;2]{
        [tile_idx/n_cols, tile_idx % n_cols]
    }
    pub fn get_tile_idx(row_idx:usize, col_idx:usize, n_cols:usize) -> usize{
        row_idx*n_cols + col_idx
    }
    fn get_tile_neighbours(tile_idx:usize, n_rows:usize, n_cols:usize) -> Vec<usize>{
        let [x,y] = Map::get_tile_coords(tile_idx, n_cols);
        let mut neighbours = Vec::new(); // Any tile has 0 to 8 neighbours.
        for i in -1..=1i64{ // Using i64 to make sure we don't truncate x and y when adding it in.
            // Check that we're not leaving the usable area
            if (x == 0 && i==-1) || (x == n_rows-1 && i==1){
                continue;
            }
            for j in -1..=1i64{
                // Check that we're not leaving the usable area
                if (y == 0 && j == -1) || (y == n_cols-1 && j == 1){ 
                    continue;
                }
                if i==0 && j==0{ // Don't count the tile itself!
                    continue;
                }
                let [new_x, new_y] = [(x as i64 +i) as usize, (y as i64+j) as usize];
                let idx = Map::get_tile_idx(new_x, new_y, n_cols);
                // Only store neighbouring tiles that have seats. Floor is irrelevant.
                neighbours.push(idx)
            }
        }
        neighbours
    }
    pub fn count_occupied(&self) -> i32{
        self.data.iter().fold(0, |acc,t|{
            match t{
                Tile::Seat(true) => acc+1,
                _ => acc
            }
        })
    }
    pub fn print_map(&self){
        for x in 0..self.n_rows{
            for y in 0..self.n_cols{
                let idx = Map::get_tile_idx(x, y, self.n_cols);
                print!("{}",self.data[idx]);
            }
            println!();
        }
        println!()
    }
    fn print_custom_map<T>(map:&Vec<T>, n_cols:usize, n_rows:usize) where T:Display{
        for x in 0..n_rows{
            for y in 0..n_cols{
                let idx = Map::get_tile_idx(x, y, n_cols);
                print!("{}",map[idx]);
            }
            println!();
        }
        println!()
    }
    pub fn print_pair_map<T,K>(map_a:&Vec<T>, map_b: &Vec<K>, n_cols:usize, n_rows:usize) where T:Display, K:Display{
        for x in 0..n_rows{
            for y in 0..n_cols{
                let idx = Map::get_tile_idx(x, y, n_cols);
                print!("{}",map_a[idx]);
            }
            print!(" | ");
            for y in 0..n_cols{
                let idx = Map::get_tile_idx(x, y, n_cols);
                print!("{}",map_b[idx]);
            }
            println!();
        }
        println!("__________ | __________");
        println!();
    }
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut map = Map::new(&data_string);
    #[cfg(test)]
    {
        map.print_map();
        println!();
    }
    map.update_tiles();
    #[cfg(test)]
    {
        map.print_map();
        println!();
    }
    let mut count = 1;
    while map.recent_change{
        map.update_tiles();
        count += 1;
        #[cfg(test)]
        {
            map.print_map();
            println!();
        }
    }
    println!("Count: {count}");
    return Some(map.count_occupied())

}

#[cfg(test)]
    mod tests{
    use super::*;
    #[test]
    fn step_test(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        let mut map = Map::new(&data_string);
        for target_data_file in [r"src\dummy_step1.txt",r"src\dummy_step2.txt",r"src\dummy_step3.txt"]{
            map.update_tiles();
            let target_data_string = read_to_string(target_data_file).unwrap();
            let target_map = Map::new(&target_data_string);

            Map::print_pair_map(&map.data, & target_map.data, map.n_cols, map.n_rows);

            target_map.data
                .iter()
                .zip(map.data.iter())
                .enumerate()
                .for_each(|(i,(expected_t, actual_t))|{
                    let [x,y] = Map::get_tile_coords(i, target_map.n_cols);
                    assert_eq!(*actual_t, *expected_t,">>{target_data_file}<<\nat [{x},{y}]");
                });
        }
    }
    #[test]
    fn neighbour_check_test(){
        let data_string = read_to_string(r"src\dummy_step1.txt").unwrap();
        let map = Map::new(&data_string);
        map.print_map();
        let [x,y] = [0,2];
        let tile_idx = Map::get_tile_idx(x, y, map.n_cols);
        let neighbours = Map::get_tile_neighbours(tile_idx, map.n_rows, map.n_cols);
        println!("{neighbours:?}");
        for x in 0..map.n_rows{
            for y in 0..map.n_cols{
                let idx = Map::get_tile_idx(x, y, map.n_cols);
                if idx == tile_idx{
                    print!("O");
                }else if neighbours.contains(&idx){
                    print!("X");
                } else{
                    print!("{}",map.data[idx]);
                }
            }
            println!();
        }
        println!();
        let occupied_neighbours = map.occupied_neighbours_list();
        assert_eq!(occupied_neighbours[tile_idx], 4);
    }

}
