use std::{fs::read_to_string, collections::VecDeque};
use crate::p1::{Matrix, process_data_string};
pub fn main_2(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let (tile_vec,xmax,ymax) = process_data_string(data_string);
    let mut tiles = Matrix::new(tile_vec,xmax,ymax);
    let mut history = vec![];
    for i in 0..1000{
        tiles.tilt_cycle();
        if (i+1)%10 == 0{
            let current_load = tiles.current_load();
            if history.len() > 100{
                if history.iter().all(|v| *v == current_load){
                    break;
                }
                history.remove(0);
            }
            history.push(current_load);
        }
    }
    let current_load = tiles.current_load();
    Some(current_load)

}

#[cfg(test)]
    mod tests{
    use super::*;

    // #[test]
    fn tilt_north(){
        let data_string = read_to_string(r"src/dummy.txt").unwrap();
        let (tile_vec,xmax,ymax) = process_data_string(data_string);
        let mut tiles = Matrix::new(tile_vec,xmax,ymax);
        println!("_____ Tilting North: _____");
        tiles.print_pattern();
        tiles.tilt_north();
        tiles.print_pattern();
        println!("__________________________");
        assert!(false)
    }
    // #[test]
    fn tilt_south(){
        let data_string = read_to_string(r"src/dummy.txt").unwrap();
        let (tile_vec,xmax,ymax) = process_data_string(data_string);
        let mut tiles = Matrix::new(tile_vec,xmax,ymax);
        println!("_____ Tilting South: _____");
        tiles.print_pattern();
        tiles.tilt_south();
        tiles.print_pattern();
        println!("__________________________");
        assert!(false)
    }
    // #[test]
    fn tilt_west(){
        let data_string = read_to_string(r"src/dummy.txt").unwrap();
        let (tile_vec,xmax,ymax) = process_data_string(data_string);
        let mut tiles = Matrix::new(tile_vec,xmax,ymax);
        println!("_____ Tilting West: _____");
        tiles.print_pattern();
        tiles.tilt_west();
        tiles.print_pattern();
        println!("__________________________");
        assert!(false)
    }
    // #[test]
    fn tilt_east(){
        let data_string = read_to_string(r"src/dummy.txt").unwrap();
        let (tile_vec,xmax,ymax) = process_data_string(data_string);
        let mut tiles = Matrix::new(tile_vec,xmax,ymax);
        println!("_____ Tilting East: _____");
        tiles.print_pattern();
        tiles.tilt_east();
        tiles.print_pattern();
        println!("__________________________");
        assert!(false)
    }

}
