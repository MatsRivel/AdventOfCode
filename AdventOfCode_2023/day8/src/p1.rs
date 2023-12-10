use std::{fs::read_to_string, collections::HashMap};
#[derive(Copy,Clone,Debug)]
enum Dir{
    Left,
    Right
}
impl Dir{
    fn new(c:char)->Self{
        match c{
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!()
        }
    }
}
struct Node{
    name: String,
    left: String,
    right: String
}
impl Node{
    fn new(name_s:&str,left_s:&str,right_s:&str)->Self{
        let left = left_s.to_string();
        let right =right_s.to_string();
        let name = name_s.to_string();
        Node{name,left,right}
    }
}
struct WalkOrder{
    path: std::iter::Cycle<std::vec::IntoIter<Dir>>
}
impl WalkOrder{
    fn new(s:&str)->Self{
        let mut temp = Vec::with_capacity(s.len());
        for c in s.chars(){
            temp.push(Dir::new(c));
        }
        let path = temp.into_iter().cycle();
        WalkOrder{ path }
    }
}
impl Iterator for WalkOrder{
    type Item=Dir;
    fn next(&mut self) -> Option<Self::Item> {
        self.path.next()
    }
}

fn process_data_string(data_string:String)->(WalkOrder,HashMap<String,Node>){
    let mut lines = data_string.lines();
    let path_str = lines.next().unwrap();
    let path = WalkOrder::new(path_str);
    lines.next(); // Skipping empty line.

    let nodes = lines.map(|line| {
        let [name,temp]: [&str;2] = line.split(" = (").collect::<Vec<&str>>().try_into().unwrap();
        let [left_child, right_child]: [&str;2] = temp
            .split(", ")
            .map(|s| {
                match s.strip_suffix(")"){
                    Some(new_s) => new_s,
                    None => s
                }
            })
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        (name.to_string(),Node::new(name,left_child,right_child))
        }).collect::<HashMap<String,Node>>(); 

    (path,nodes)
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();
    let (mut path,nodes) = process_data_string(data_string);
    let mut current = nodes.get(&start).unwrap();
    let mut counter = 0;
    while current.name != end{
        counter += 1;
        let dir = path.next().unwrap();
        match dir{
            Dir::Left => current = nodes.get(&current.left).unwrap(),
            Dir::Right => current = nodes.get(&current.right).unwrap(),
        }
    }
    Some(counter)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
