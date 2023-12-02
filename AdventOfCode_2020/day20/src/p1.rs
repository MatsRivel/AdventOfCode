use std::fs::read_to_string;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Tile{
    On,
    Off
}
#[derive(Debug,Clone,Copy,PartialEq)]
enum Orientation{
    Up,
    Down,
    Right,
    Left
}
enum FlipState{
    HorizVert,
    Horiz,
    Vert,
    None,
}
impl FlipState{
    fn invert(self) -> Self{
        match self{
            FlipState::HorizVert => FlipState::None,
            FlipState::Horiz => FlipState::Vert,
            FlipState::Vert => FlipState::Horiz,
            FlipState::None => FlipState::HorizVert,
        }
    }
    fn flip_vert(self) -> Self{
        match self{
            FlipState::HorizVert => FlipState::Horiz,
            FlipState::Horiz => FlipState::HorizVert,
            FlipState::Vert => FlipState::None,
            FlipState::None => FlipState::Vert,
        }
    }
    fn flip_horiz(self) -> Self{
        match self{
            FlipState::HorizVert => FlipState::Vert,
            FlipState::Horiz => FlipState::None,
            FlipState::Vert => FlipState::HorizVert,
            FlipState::None => FlipState::Horiz,
        }
    }
}
impl Orientation {
    fn rotate(self)->Self{
        // Rotates 90 degrees clockwise
        match self{
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Right => Orientation::Down,
            Orientation::Left => Orientation::Up,
        }
    }
}
struct TileBorders{
    content: Vec<Tile>,
    tile_height: usize,
    tile_width: usize,
    orientation: Orientation,
    flip_state: FlipState
}
impl TileBorders{
    fn get_top_raw(&self) -> &[Tile]{
        let left = 0;
        let right = self.tile_width;
        &self.content[left..right]
    }
    fn get_right_raw(&self) -> &[Tile]{
        let left = self.tile_width;
        let right = self.tile_width + self.tile_height;
        &self.content[left..right]
    }
    fn get_bottom_raw(&self) -> &[Tile]{
        let left = self.tile_width + self.tile_height;
        let right = 2*self.tile_width + self.tile_height;
        &self.content[left..right]
    }
    fn get_left_raw(&self) -> &[Tile]{
        let left = self.tile_width + self.tile_height;
        let right = 2*self.tile_width + self.tile_height;
        &self.content[left..right]
    }

    fn get_top_flip_check(&self) -> &[Tile]{
        
        todo!()
    }
    
    pub fn get_top(&self) -> &[Tile]{
        match self.orientation{
            Orientation::Up     => self.get_top_raw(),
            Orientation::Right  => self.get_right_raw(),
            Orientation::Down   => self.get_bottom_raw(),
            Orientation::Left   => self.get_left_raw(),
            _ => panic!("Unreachable!")
        }
    }
    pub fn get_right(&self) -> &[Tile]{
        match self.orientation{
            Orientation::Left   => self.get_top_raw(),
            Orientation::Up     => self.get_right_raw(),
            Orientation::Right  => self.get_bottom_raw(),
            Orientation::Down   => self.get_left_raw(),
            _ => panic!("Unreachable!")
        }
    }
    pub fn get_bottom(&self) -> &[Tile]{
        match self.orientation{
            Orientation::Down => self.get_top_raw(),
            Orientation::Left => self.get_right_raw(),
            Orientation::Up => self.get_bottom_raw(),
            Orientation::Right => self.get_left_raw(),
            _ => panic!("Unreachable!")
        }
    }
    pub fn get_left(&self) -> &[Tile]{
        match self.orientation{
            Orientation::Right => self.get_top_raw(),
            Orientation::Down => self.get_right_raw(),
            Orientation::Left => self.get_bottom_raw(),
            Orientation::Up => self.get_left_raw(),
            _ => panic!("Unreachable!")
        }
    }
    fn rotate(&mut self){
        // rotates self by 90 degrees clockwise
        self.orientation = self.orientation.rotate();
        

    }
    fn find_matching_sides(&self, other: &TileBorders )-> Option<(Orientation,Orientation)>{
        // Returns Some((a,b)) where 'a' is the side of self that matches the 'b' side of other.
        // Returns None if no match is found.
        let a = self.get_top();
        let b = self.get_bottom();
        todo!()
    }
}


pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    None

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
