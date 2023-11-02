use std::fs::read_to_string;
use crate::p1::{Tile,Map};

impl Map{

    fn neighbour_vision_search(&self, seat_idx:usize, direction:&[i32;2])->Option<usize>{
        let [i,j] = direction;
        let [x,y] = Map::get_tile_coords(seat_idx, self.n_cols);
        if (x == 0 && i == &-1) || (x == self.n_rows-1 && i == &1){
            return None;
        }else if (y == 0 && j == &-1) || (y == self.n_cols-1 && j == &1){
            return None;
        }
        let [new_x, new_y] = [(x as i64 + *i as i64) as usize, (y as i64 + *j as i64) as usize];
        let new_idx = Map::get_tile_idx(new_x, new_y, self.n_cols);
        let possible_seat = self.get_tile(new_idx);
        if let Some(Tile::Seat(_)) =possible_seat {
            return Some(new_idx);
        }
        return self.neighbour_vision_search(new_idx, direction)
    }
    fn get_tile_neighbours_v2(&self,tile_idx:usize)->Vec<usize>{
        let [x,y] = Map::get_tile_coords(tile_idx, self.n_cols);
        let mut neighbours = Vec::new(); // Any tile has 0 to 8 neighbours.
        for i in -1..=1i32{ // Using i64 to make sure we don't truncate x and y when adding it in.
            // Check that we're not leaving the usable area
            if (x == 0 && i==-1) || (x == self.n_rows-1 && i==1){
                continue;
            }
            for j in -1..=1i32{
                // Check that we're not leaving the usable area
                if (y == 0 && j == -1) || (y == self.n_cols-1 && j == 1){ 
                    continue;
                }
                if i==0 && j==0{ // Don't count the tile itself!
                    continue;
                }
                let direction = [i,j];
                // Only store neighbouring tiles that have seats. Floor is irrelevant.
                if let Some(idx) =self.neighbour_vision_search(tile_idx, &direction){
                    neighbours.push(idx)

                }
            }
        }
        neighbours
    }
    fn occupied_neighbours_list_v2(&self)->Vec<usize>{
        // Gets the number of occupied tiles for each neighbour
        let occupied_count = self.data
            .iter()
            .enumerate()
            .map(|(i,t)|{
                if let Tile::Floor = t{
                    0
                }else{
                    let neighbors = self.get_tile_neighbours_v2(i);
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
    pub fn update_tiles_v2(&mut self){
        let occupied_count = self.occupied_neighbours_list_v2();
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
                            (true, 5..) => {
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
}
pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut map = Map::new(&data_string);
    #[cfg(test)]
    {   
        Map::print_pair_map(&map.data,&map.occupied_neighbours_list_v2(),map.n_cols,map.n_rows);
    }
    map.update_tiles_v2();
    #[cfg(test)]
    {
        Map::print_pair_map(&map.data,&map.occupied_neighbours_list_v2(),map.n_cols,map.n_rows);
    }
    let mut count = 1;
    while map.recent_change{
        map.update_tiles_v2();
        count += 1;
        #[cfg(test)]
        {
            Map::print_pair_map(&map.data,&map.occupied_neighbours_list_v2(),map.n_cols,map.n_rows);
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
        for target_data_file in [r"src\dummy_p2_step1.txt",r"src\dummy_p2_step2.txt",r"src\dummy_p2_step3.txt"]{
            map.update_tiles_v2();
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
    fn neighbour_check_step1_test(){
        let data_string = read_to_string(r"src\dummy_p2_step1.txt").unwrap();
        let map = Map::new(&data_string);
        map.print_map();
        let [x,y] = [0,2];
        let tile_idx = Map::get_tile_idx(x, y, map.n_cols);
        let neighbours = map.get_tile_neighbours_v2(tile_idx);
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
        let occupied_neighbours = map.occupied_neighbours_list_v2();
        assert_eq!(occupied_neighbours[tile_idx], 5);
    }
    #[test]
    fn neighbour_check_step2_test(){
        let data_string = read_to_string(r"src\dummy_p2_step2.txt").unwrap();
        let map = Map::new(&data_string);
        map.print_map();
        let [x,y] = [0,2];
        let tile_idx = Map::get_tile_idx(x, y, map.n_cols);
        let neighbours = map.get_tile_neighbours_v2(tile_idx);
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
        let occupied_neighbours = map.occupied_neighbours_list_v2();
        assert_eq!(occupied_neighbours[tile_idx], 1);
    }
}
