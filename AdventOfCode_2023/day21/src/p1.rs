use std::{fs::read_to_string, thread::current};
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile{
    Rock,
    Person,
    Garden,
}
impl TryFrom<char> for Tile{
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value{
            '.' => Ok(Tile::Garden),
            '#' => Ok(Tile::Rock),
            'S' => Ok(Tile::Person),
            _ => Err("Invalid Tile Char".to_string())
        }
    }
}

fn limit_value_towards_lower(value:usize, limit:usize)->usize{
    if value > limit{
        value - limit
    }else{
        0
    }
}

fn limit_value_towards_upper(value:usize, limit:usize)->usize{
    if value < limit{
        limit - value
    }else{
        limit
    }
}

fn square_of_pos_around_point(x:usize,y:usize, square_size:usize) -> Vec<[usize;2]>{
    let min_x = limit_value_towards_lower(x,square_size/2);
    let min_y = limit_value_towards_lower(y,square_size/2);
    let max_x = limit_value_towards_upper(x,square_size/2);
    let max_y = limit_value_towards_upper(y,square_size/2);

    let mut output = Vec::<[usize;2]>::new();
    for new_y in min_y..max_y{
        output.push([min_x,new_y]);
        output.push([max_x,new_y]);
    }
    for new_x in min_x..max_x{
        output.push([new_x,min_y]);
        output.push([new_x,max_y]);
    }
    output
}

fn adj_pos(x:usize,y:usize,upper_bound:usize)->Vec<[usize;2]>{
    let mut output = Vec::with_capacity(4);
    output.push([x+1,y]);
    output.push([x,y+1]);
    if x > 0{
        output.push([x-1,y])
    }
    if y > 0{
        output.push([x,y-1])
    }
    output
}
#[derive(Clone)]
struct Atlas{
    width:usize,
    data: Vec<Tile>,
    step_count: Vec<Option<i32>>
}
impl From<&str> for Atlas{
    fn from(s: &str) -> Self  {
        let width = match s.find("\r\n"){
            Some(v) => v,
            None => s.len()
        };
        let data = s
            .chars()
            .filter_map(|c| {
                Tile::try_from(c).ok()
            }).collect::<Vec<Tile>>();
        let step_count = vec![None;data.len()];
        Atlas{width,data,step_count}
    }
}

impl Atlas{
    fn trimmed(self, max_steps:usize) -> Self{
        let target_idx = self.data.iter().position(|&tile| tile == Tile::Person).expect("We are OK with crashing here if we do not have a person in our world map...");
        let [x,y] = self.idx_to_coord(&target_idx);
        let min_x = limit_value_towards_lower(x,max_steps/2);
        let min_y = limit_value_towards_lower(y,max_steps/2);
        let max_x = limit_value_towards_upper(x,self.width/2);
        let max_y = limit_value_towards_upper(y,self.width/2);

        let new_width = max_x-min_x;
        let mut new_data: Vec<Tile> = Vec::with_capacity( new_width * (max_y-min_y) );
        let new_total_size = new_data.len();

        for i in min_x..max_x{
            for j in min_y..max_y{
                new_data[(i-min_x) + (j-min_y)*new_width] = *self.get_pos(&i, &j).unwrap();
            }
        }
        Atlas{width: new_width, data: new_data, step_count: vec![None;new_total_size] }

    }
    fn idx_to_coord(&self, idx:&usize) -> [usize;2]{
        let x = idx % self.width;
        let y = idx / self.width;
        [x,y]
    }
    fn coord_to_idx(&self,x:&usize,y:&usize)->usize{
        y*self.width + x
    }
    fn get_pos(&self,x:&usize,y:&usize)->Option<&Tile>{
        let idx = self.coord_to_idx(x,y);
        self.data.get(idx)
    }
    fn get_steps(&self,x:&usize,y:&usize)->Option<i32>{
        let idx = self.coord_to_idx(x,y);
        match self.step_count.get(idx){
            Some(v) => *v,
            None => None
        }
    }
    fn update_steps(&mut self, x:&usize, y:&usize, new_step:&i32){
        let current_steps = self.get_steps(x, y);
        if current_steps.is_some() && &current_steps.unwrap() > new_step{
                let idx = self.coord_to_idx(x, y);
                let old_step = self.step_count.get_mut(idx).unwrap();
                *old_step = Some(*new_step);
        }
    }
}

fn width_first_traversal(mut atlas:Atlas, max_steps:usize)->Atlas{
    let start_idx = atlas.data.iter().position(|tile| tile == &Tile::Person).unwrap();
    let start_pos = atlas.idx_to_coord( &start_idx );
    let mut current_step = 0;

    while current_step < max_steps{
        current_step += 1;
        let all_next_pos = square_of_pos_around_point(start_pos[0], start_pos[1], current_step);
        for pos in all_next_pos.into_iter(){
            let all_adj_pos = adj_pos(pos[0],pos[1],atlas.width);
            let mut step_values = vec![];
            for temp_pos in all_adj_pos.into_iter(){
                let step_val = atlas.get_steps(&temp_pos[0],&temp_pos[1]);
                if let Some(v) = step_val{
                    step_values.push(v)
                }
            }
            let min_step = *step_values.iter().min().unwrap();
            let idx = atlas.coord_to_idx(&pos[0], &pos[1]);
            let current_step = atlas.step_count.get_mut(idx).unwrap();
            *current_step = Some(min_step);
        }
    }

    todo!()
}
pub fn main_1(file_name:&str,n_steps:usize)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let atlas = Atlas::from( data_string.as_str() ).trimmed(n_steps);

    todo!()

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
