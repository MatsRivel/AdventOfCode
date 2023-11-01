use std::fs::read_to_string;

enum Tile{
    Seat(bool),
    Floor
}

struct Map{
    n_rows:usize,
    n_cols:usize,
    data:Vec<Tile>
}
fn string_to_tiles(s:&str) -> Vec<Tile>{
    s.lines()
    .map(|line|{
        line.chars().map(|c|{
            match c{
                'L' => Tile::Seat(false),
                '.' => Tile::Floor,
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
    fn new(data_string:&str) -> Self{
        let mut n_cols= 0;
        for line in data_string.lines(){
            n_cols = line.len();
            break;
        }
        let data = string_to_tiles(data_string);
        let n_rows = data.len()/n_cols;
        Self{n_rows, n_cols, data}
    }
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let map = Map::new(&data_string);
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
