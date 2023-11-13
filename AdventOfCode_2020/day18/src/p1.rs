use std::{fs::read_to_string, fmt::Display, collections::VecDeque};

// Visualization Implementations 
impl Display for Nest{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(v) = &self.val{
            write!(f, "{v}").unwrap();
        }else{
            for element in self.content.iter() {
                write!(f, "{element}").unwrap();
            }
        }

        Ok(())
    }
}
impl Display for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self{
            Node::Mult => "*".to_string(),
            Node::Add => "+".to_string(),
            Node::LParen => "(".to_string(),
            Node::RParen => ")".to_string(),
            Node::Val(v) => v.to_string(),
        };
        write!(f,"{text}")
    }
}
// Wrapper to prevent pop_front / push_front accitentally.
pub struct Stack<T>{
    stack: VecDeque<T>
}
impl<T> Stack<T>{
    pub fn new() -> Self{
        Stack { stack: VecDeque::new() }
    }
    pub fn push(&mut self, v:T){
        self.stack.push_back(v);
    }
    pub fn pop(&mut self) -> Option<T>{
        self.stack.pop_back()
    }

} 

// Trait for convenience
pub trait ToNode<T>{
    fn to_node(self) -> Option<Node>;
}
impl ToNode<u128> for u128{
    fn to_node(self) -> Option<Node> {
        Some(Node::Val(self))
    }
}
impl ToNode<char> for char{
    fn to_node(self)->Option<Node>{
        match self{
            '*' => Some(Node::Mult),
            '+' =>Some(Node::Add),
            '('=>Some(Node::LParen), 
            ')' =>Some(Node::RParen),
            s => {
                if s.is_numeric(){
                    Some(Node::Val(s.to_digit(10).expect("Already checked") as u128))
                }else{
                    None
                }
            }
        }
    }
}
 
// Actual data structures:
#[derive(Clone,Copy,Debug)]
pub enum Node{
        Mult,
        Add,
        LParen,
        RParen,
        Val(u128)
}
impl Node{

    pub fn to_nest(self) -> Nest{
        Nest{val:Some(self),content: vec![]}
    }
    pub fn get_val(self) -> u128{
        match self{
            Node::Val(v) => v,
            _ => panic!("'get_val()' only works on nodes holding values!")
        }
    }
}

#[derive(Clone,Debug)]
pub struct Nest{
    pub val: Option<Node>,
    pub content:Vec<Nest>
}
impl Nest{
    pub fn new()->Self{
        Nest{val:None,content:vec![]}
    }
    pub fn push(&mut self, nest_in: Nest){
        self.content.push(nest_in);
    }
    fn merge_content(self)->Node{
        if let Some(v) = self.val{
            return v;
        } else if self.content.len() == 1{
            return self.content[0].clone().merge_content();
        }
        let binding = self.content.clone();
        let mut content = binding.iter();

        let mut current_value = {
            content
                .next()
                .expect("Should never have less than 3 elements in a Nest")
                .clone()
                .merge_content()
                .get_val()
        };

        let mut operation = {
            content
                .next()
                .expect("Should never have less than 3 elements in a Nest")
                .clone()
                .merge_content()
        };

        while let Some(new_raw) = content.next(){
            let new_value =new_raw.clone().merge_content().get_val();
            match operation{
                Node::Mult => {
                    #[cfg(test)]
                    println!("{current_value} {operation} {new_value} = {}",current_value * new_value);
                    current_value *= new_value;
                },
                Node::Add => {
                    #[cfg(test)]
                    println!("{current_value} {operation} {new_value} = {}",current_value + new_value);
                    current_value += new_value;
                },
                Node::LParen | Node::RParen | Node::Val(_) => panic!("Should never happen!"),
            }
            if let Some(new_opr) = content.next(){
                operation = new_opr.clone().merge_content();
            }else{
                break;
            }            
        }
        // Now perform multiplication:
        current_value.to_node().expect("u128.to_node() is infallable")
    }
}

pub fn get_nodes(data_row:&str)->Vec<Node>{
    data_row.chars().filter_map(|c| c.to_node()).collect::<Vec<Node>>()
}

pub fn main_1(file_name:&str)->Option<u128>{
    let data_string = read_to_string(file_name).unwrap();
    let mut sum = 0;
    for line in data_string.lines(){
        let raw_nodes = get_nodes(line);
        let mut nest_stack = Stack::new();
        let mut current_nest = Nest::new();
        // First we divide each section such that it obeys the parenthesies.
        for node in raw_nodes.iter(){
            match node{
                Node::Mult | Node::Add | Node::Val(_) => { // We only add a nest here which has no children.
                    current_nest.push(node.to_nest())
                },
                Node::LParen => {
                    nest_stack.push(current_nest.clone());
                    current_nest = Nest::new();
                },
                Node::RParen => {
                    let mut old_nest = nest_stack.pop().expect("Should never have a ')' without a '(' first!");
                    old_nest.push(current_nest);
                    current_nest = old_nest;
                }
            }
        }
        sum += current_nest.merge_content().get_val();
    }
    Some(sum)

}

#[cfg(test)]
    mod tests{
}
