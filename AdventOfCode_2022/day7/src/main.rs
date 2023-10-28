mod p1{
    use core::panic;
    use std::fmt::Display;
    use std::fs::read_to_string;
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time;

    use crate::PRINT;
    #[derive(Clone,Debug)]
    struct File{
        size: i32,
    }
    impl File{
        fn new(size_str:&str) -> Self{
            let size = size_str.trim().parse::<i32>().expect("--- Failed to parse \"{size_str}\" to i32 ---");
            File{size}
        }
    }
    #[derive(Clone,Debug)]
    struct Folder{
        children: Vec<Node>,

    }
    impl Folder{
        fn new() -> Self{
            Folder{children: Vec::<Node>::new()}
        }
    }
    #[derive(Clone,Debug)]
    pub enum Obj {
        File(File),
        Folder(Folder),
    }
    #[derive(Eq, Hash, Clone,Debug, PartialEq)]
    pub enum Node{
        Dir(String),
        Leaf(String),
    }
    impl Display for Node{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self{
                Node::Dir(name) => write!(f, "{name}" ),
                Node::Leaf(name) => write!(f, "{name}")
            }

        }
    }
    fn node_vec_to_string(v:&Vec<Node>,current_name:&str) -> String {
        let mut new_string = v.iter().fold("".to_string(), |acc, node| {
            format!("{acc}/{node}")
        });
        new_string = format!("{new_string}/{current_name}");
        return new_string;
    }
    pub fn process_data_string(data_string:&str) -> HashMap<Node,Obj>{
        let mut dir_history: Vec<Node> = vec![];
        let mut output:HashMap<Node,Obj> = HashMap::new();
        output.insert(Node::Dir("root".to_string()), Obj::Folder(Folder::new()));
        let mut depth = 0;
        data_string.lines().for_each(|line|{
            let elements = line.split(" ").collect::<Vec<&str>>();
            match elements.len(){
                // Navigation Commands:
                3 => match (elements[0],elements[1],elements[2]){
                    ("$", "cd", "/") => {
                        dir_history = vec![Node::Dir("root".to_string())];
                        depth = 1;
                    },
                    ("$", "cd", "..") => {
                        dir_history.pop();
                        depth -= 1;
                    },
                    ("$", "cd", name) => {
                        depth += 1;
                        let new_name = node_vec_to_string(&dir_history, name);
                        dir_history.push(Node::Dir(new_name.to_string()));
                        if output.get(&Node::Dir(new_name.to_string())).is_none(){
                            output.insert(Node::Dir(new_name.to_string()),Obj::Folder(Folder::new()));
                        }
    
                    },
                    _ => {
                        panic!("Unexpected pattern: {elements:?}")}
                },
                // File Commands:
                2 => match (elements[0], elements[1]){
                    ("$","ls") => (),
                    ("dir", name) => {
                        let new_name = node_vec_to_string(&dir_history, name);
                        let new_node = Node::Dir(new_name);
                        if output.get(&new_node).is_none(){
                            output.insert(new_node.clone(),Obj::Folder(Folder::new()));
                        }
                        let prev_dir_node = dir_history.last().expect("We know it exists");
 
                        // Dir history only holds folders, so we know output[prev_dir_name] is a Folder, not a File
                        // Here we add the current dir as a child of the previous dir if it is not already registered as a child.
                        if let Some(Obj::Folder(prev_dir)) = output.get_mut(prev_dir_node){
                            if !prev_dir.children.contains(&new_node){
                                prev_dir.children.push(new_node);
                            }
                        }
                    },
                    (size, name) => {
                        let new_name = node_vec_to_string(&dir_history, name);
                        let new_node = Node::Leaf(new_name);
                        output.insert(new_node.clone(), Obj::File(File::new(size)));
                        let prev_dir_name = dir_history.last().expect("We know it exists");

                        // Dir history only holds folders, so we know output[prev_dir_name] is a Folder, not a File
                        if let Some(Obj::Folder(prev_dir)) = output.get_mut(prev_dir_name){
                            if !prev_dir.children.contains(&new_node){
                                prev_dir.children.push(new_node);
                            }
                        }
    
                    },
                    
                },
                _ => panic!("Unexpected length of pattern: {elements:?}")
            }
        });

        return output;
    }

    pub fn depth_first_recursive<'a>(data:&'a HashMap<Node,Obj>, current:Node, depth:i32, current_path:Vec<Node>)->(i32,HashMap<Node, i32>){
        // "File"s are always returned, as they are all leaves.
        if current_path.contains(&&current){
            println!(".");
            panic!("{current:?} exists in {current_path:?}!");
        }
        if let Some(Obj::File(leaf)) = data.get(&current){
            if PRINT{println!("{current_path:?}/{current}");}
            return (leaf.size, HashMap::<Node, i32>::new());
        } // Folders with a known size are always returned; No need to reiterate on known data! 
        if PRINT{println!("{current_path:?}/{current}/");}

        let dir = match data.get(&current){
            Some(Obj::Folder(dir)) => dir.clone(),
            x => panic!("Should not be able to reach here if not a folder.. {current}: {x:?}")
        };
        let mut dir_size = 0;
        let mut history: HashMap<Node, i32> = HashMap::new();
        let dir_children = dir.children.clone();
        for child in dir_children.iter(){ // Have to clone due to immutability issues... Look for alternative solution?
            let mut new_path = current_path.clone();
            new_path.push(current.clone());
            let (more_dir_size,more_history) = depth_first_recursive(data, child.clone(),depth+1,new_path);
            more_history.iter().for_each(|(key,val)|{
                if history.contains_key(&key){
                    let size = history.get_mut(&key).unwrap();
                    *size += val; 
                }else{
                    history.insert(key.clone(),*val);
                }
            });
            dir_size += more_dir_size;
        }
        if history.contains_key(&current){
            let size = history.get_mut(&current).unwrap();
            *size += dir_size; 
        }else{
            history.insert(current.clone(),dir_size);
        }
        return (dir_size,history);

    }
    pub fn initialize_depth_first<'a>(data:&'a HashMap<Node,Obj>, root:Node) -> (i32, HashMap<Node, i32>){
        depth_first_recursive(data, root.clone(), 0, vec![])
    }
    const MAX_SIZE:i32 = 100_000i32;
    pub fn main_1(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        // Make a traversable HashMap of Folders and Files.
        let data: HashMap<Node,Obj> = process_data_string(&data_string);
        if PRINT{println!("Data has been processed!");}
        let root = Node::Dir("root".to_string());
        let (_,folders) = initialize_depth_first(&data, root);
        if PRINT{println!("Depths has been traversed!");}
        let sum_of_dirs = folders.iter().filter_map(|(_node,size)|{
            if size <= &MAX_SIZE { 
                Some(size)
            }else{
                None
            }
        }).fold(0, |acc, size| acc+ size);
        if PRINT{println!("Sum has been collected!");}
        return Some(sum_of_dirs);
    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}
mod p2{
    use std::fs::read_to_string;
    use std::collections::HashMap;
    use crate::p1::{process_data_string,Node,Obj,initialize_depth_first};
    use crate::PRINT;
    const NEEDED_SPACE:i32 = 30_000_000;
    const TOTOAL_SPACE:i32 = 70_000_000;
    pub fn main_2(file_name:&str)->Option<i32>{
        let data_string = read_to_string(file_name).unwrap();
        // Make a traversable HashMap of Folders and Files.
        let data: HashMap<Node,Obj> = process_data_string(&data_string);
        if PRINT{println!("Data has been processed!");}
        let root = Node::Dir("root".to_string());
        let (total_size,folders) = initialize_depth_first(&data, root);
        if PRINT{println!("Depths has been traversed!");}
        let current_space = TOTOAL_SPACE - total_size;
        let savings_needed = NEEDED_SPACE-current_space;
        // Filter any file too small to matter,
        // Then return only the smallest of the files big enough to make a difference.
        let smallest_deletable_size = folders.iter().filter_map(|(_,val)|{
            if val >= &savings_needed{
                Some(val)
            }else{
                None
            }
        }).fold(total_size+1, |acc,val|{
            if val <= &acc{
                *val
            }else{
                acc
            }
        });
        Some(smallest_deletable_size)
    }
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}

use p1::main_1;
use p2::main_2;
use std::time::Instant;
const PRINT:bool = false;
fn main() {
    let file_name = r"src\dummy_input.txt";
    let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.unwrap(),95437,"\n\n- - - Answer should be 95437! - - -\n\n")
    }else if file_name == r"src\puzzle_input.txt"{
        assert!(count.unwrap()>864974,"\n\n- - - Answer is too low!  ( {} <=  864974 ) - - -\n\n",count.unwrap());
        assert!(count.unwrap()<1157426,"\n\n- - - Answer is too high! ( {} >= 1157426 ) - - -\n\n",count.unwrap());
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.unwrap(),24933642,"\n\n- - - Answer should be 24933642! - - -\n\n")
    }else if file_name == r"src\puzzle_input.txt"{
    // assert!(count.unwrap()>864974,"\n\n- - - Answer is too low!  ( {} <=  864974 ) - - -\n\n",count.unwrap());
    assert!(count.unwrap()<24933642,"\n\n- - - Answer is too high! ( {} >= 24933642 ) - - -\n\n",count.unwrap());
}
}

