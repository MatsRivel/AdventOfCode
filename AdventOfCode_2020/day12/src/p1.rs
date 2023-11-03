use std::{fs::read_to_string, fmt::Display};
#[derive(Debug)]
pub enum DirMove{
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}
impl DirMove{
    pub fn new(s:&str)->Self{
        if let Some(num) = s.strip_prefix("N"){
            DirMove::N(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("S"){
            DirMove::S(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("E"){
            DirMove::E(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("W"){
            DirMove::W(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("L"){
            DirMove::L(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("R"){
            DirMove::R(num.parse::<i32>().unwrap())
        }else if let Some(num) = s.strip_prefix("F"){
            DirMove::F(num.parse::<i32>().unwrap())
        }else{
            panic!("No valid input found!")
        }
    }
    pub fn get_dist(&self,my_rot:&MyRot)->[i32;2]{
        match self{ // Note to self: The fuckup is here somewhere...
            DirMove::N(v) => [ -1*v, 0],
            DirMove::S(v) => [1*v, 0],
            DirMove::E(v) => [ 0, 1*v],
            DirMove::W(v) => [ 0,-1*v],
            DirMove::R(_) | DirMove::L(_) => [0,0],
            other => {
                match (my_rot,other){
                    (MyRot::N, DirMove::F(v)) => [ -1*v, 0],
                    (MyRot::S, DirMove::F(v)) => [1*v, 0],
                    (MyRot::E, DirMove::F(v)) => [ 0, 1*v],
                    (MyRot::W, DirMove::F(v)) => [ 0,-1*v ],
                    _ => panic!("Immpossible combination?")
                }
            }

        }
    }
}
#[derive(Copy, Clone,Debug, PartialEq)]
pub enum MyRot{
    N,
    S,
    E,
    W
}
impl Display for MyRot{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyRot::N => write!(f,"'N'"),
            MyRot::S => write!(f,"'S'"),
            MyRot::E => write!(f,"'E'"),
            MyRot::W => write!(f,"'W'"),
        }
    }
}
impl MyRot{
    pub fn update_rot(&mut self, dirmove:&DirMove){
        let current = self.clone(); // Cloning here is trivial as the enum is tiny.
        if let DirMove::R(rot) = dirmove{
            match current{
                MyRot::N => match rot{
                    90 => *self = MyRot::E,
                    180 => *self = MyRot::S,
                    270 => *self = MyRot::W,
                    _ => ()
                },
                
                MyRot::S => match rot{
                    90 => *self = MyRot::W,
                    180 => *self = MyRot::N,
                    270 => *self = MyRot::E,
                    _ => ()
                },
                MyRot::E => match rot{
                    90 => *self = MyRot::S,
                    180 => *self = MyRot::W,
                    270 => *self = MyRot::N,
                    _ => ()
                },
                MyRot::W => match rot{
                    90 => *self = MyRot::N,
                    180 => *self = MyRot::E,
                    270 => *self = MyRot::S,
                    _ => ()
                },
            }
        }else if let DirMove::L(rot) = dirmove{
            match current{
                MyRot::E => match rot{
                    90 => *self = MyRot::N,
                    180 => *self = MyRot::W,
                    270 => *self = MyRot::S,
                    _ => ()
                },
                MyRot::W => match rot{
                    90 => *self = MyRot::S,
                    180 => *self = MyRot::E,
                    270 => *self = MyRot::N,
                    _ => ()
                },
                MyRot::S => match rot{
                    90 => *self = MyRot::E,
                    180 => *self = MyRot::N,
                    270 => *self = MyRot::W,
                    _ => ()
                },
                MyRot::N => match rot{
                    90 => *self = MyRot::W,
                    180 => *self = MyRot::S,
                    270 => *self = MyRot::E,
                    _ => ()
                },
            }

        }
    }
}
pub fn manhatten_distance(a:[i32;2], b:[i32;2]) -> i32{
    (a[0]-b[0]).abs() + (a[1]+b[1]).abs()
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let moves = data_string.lines()
        .map(|line|{
            DirMove::new(line)
        });
    let mut my_rot = MyRot::E;
    let mut my_pos = [0,0];
    #[cfg(test)]{
        println!("{my_pos:?} | {my_rot:?}");
        println!("- - - - - - - -");
    }
    for m in moves{
        my_rot.update_rot(&m);
        let [x,y] = m.get_dist(&my_rot);
        my_pos[0] += x;
        my_pos[1] += y;
        #[cfg(test)]
        println!("{my_pos:?} \t| {my_rot:?} <-- {m:?}");


    }
    return Some(manhatten_distance(my_pos, [0,0]));

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn rot_test(){
        let mut some_rot = MyRot::N;
        some_rot.update_rot(&mut DirMove::R(90));
        assert_eq!(some_rot, MyRot::E, "\n>> N + R(90) == E, != {some_rot}  <<\n");
        some_rot.update_rot(&mut DirMove::L(90));
        assert_eq!(some_rot, MyRot::N, "\n>> E + L(90) == N, != {some_rot} <<\n");
        some_rot.update_rot(&mut DirMove::L(90));
        assert_eq!(some_rot, MyRot::W, "\n>> N + L(90) == W, != {some_rot} <<\n");
        some_rot.update_rot(&mut DirMove::L(180));
        assert_eq!(some_rot, MyRot::E, "\n>> W + L(180) == E, != {some_rot} <<\n");
        some_rot.update_rot(&mut DirMove::R(270));
        assert_eq!(some_rot, MyRot::N, "\n>> E + R(270) == N, != {some_rot} <<\n");
    }

}
