use std::fs::read_to_string;
use crate::p1::{MyRot, DirMove, manhatten_distance};
trait Rotate{
    fn rot_r90(&mut self){}
    fn rot_l90(&mut self){}
    fn rotate(&mut self, rot:&DirMove){}
    
}
impl Rotate for [i32;2]{
    fn rot_r90(&mut self){
        *self = [self[1], -self[0]]
    }
    fn rot_l90(&mut self){
        *self = [-self[1], self[0]]
    }
    fn rotate(&mut self, rot:&DirMove){
        if let DirMove::R(degrees) = rot{
            for _ in 0..(degrees/90){
                self.rot_r90();
            }
        }else if let DirMove::L(degrees) = rot{
            for _ in 0..(degrees/90){
                self.rot_l90();
            }
        }
    }
}
pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let moves = data_string.lines()
        .map(|line|{
            DirMove::new(line)
        });
    let mut rot = MyRot::E;
    let mut my_pos = [0,0];
    let mut waypoint = [-1,10];
    #[cfg(test)]{
        println!("{waypoint:?} \t| _");
        println!("{my_pos:?} \t| {rot:?}");
    }
    for (idx,m) in moves.enumerate(){
        match m{
            DirMove::N(_) | DirMove::S(_) | DirMove::E(_) | DirMove::W(_) => {
                let [x,y] = m.get_dist(&rot);
                waypoint[0] += x;
                waypoint[1] += y;
            },
            DirMove::L(_) | DirMove::R(_)  => {
                waypoint.rotate(&m);
                rot.update_rot(&m);
            },
            DirMove::F(v) => {
                let mut travel_distance = waypoint.clone();
                travel_distance[0] *= v;
                travel_distance[1] *= v;
                
                let [x,y] = travel_distance;
                my_pos[0] += x;
                my_pos[1] += y;
            },
        }
        // Some inline-tests just to make sure our test-case if fulfilled, without having to extract the logic from the function itself.
        #[cfg(test)]{
            println!("\n{m:?}");
            println!("{waypoint:?}");
            println!("{my_pos:?} \t| {rot:?}");
        }
        #[cfg(test)]{
            if file_name == r"src\dummy.txt"{
                match idx{
                    0 => {
                        assert_eq!(my_pos, [-10,100]);
                        assert_eq!(waypoint, [-1,10]);
                    },
                    1 => {
                        assert_eq!(my_pos, [-10,100]);
                        assert_eq!(waypoint, [-4,10]);
                    },
                    2 => {
                        assert_eq!(my_pos, [-38,170]);
                        assert_eq!(waypoint, [-4,10]);
                    }
                    3 => {
                        assert_eq!(my_pos, [-38,170]);
                        assert_eq!(waypoint, [10,4]);
                    }
                    4 => {
                        assert_eq!(my_pos, [72,214]);
                        assert_eq!(waypoint, [10,4]);
                    }
                _ => ()

                }
            }
        }
    }
    return Some(manhatten_distance(my_pos, [0,0]));

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn test_rot(){
        let mut start = [2,1];
        let targets = [[1,-2],[-2,-1],[-1,2],[2,1]];

        start.rotate(&DirMove::R(180));
        assert_eq!(start, targets[1]);
        start.rotate(&DirMove::L(90));
        assert_eq!(start, targets[0]);
    }

}
