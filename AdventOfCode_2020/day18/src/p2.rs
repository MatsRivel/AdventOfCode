use std::{fs::read_to_string, collections::VecDeque};
use crate::p1::{Nest,Node, ToNode, Stack, get_nodes};

impl Nest{
    // This is the only change needed, as the logic is the same for everything else :)
    fn merge_content_p2(self)->Node{
        if let Some(v) = self.val{
            return v;
        } else if self.content.len() == 1{
            return self.content[0].clone().merge_content_p2();
        }
        let binding = self.content.clone();
        let mut content = binding.iter();

        let mut current_value = {
            content
                .next()
                .expect("Should never have less than 3 elements in a Nest")
                .clone()
                .merge_content_p2()
                .get_val()
        };

        let mut operation = {
            content
                .next()
                .expect("Should never have less than 3 elements in a Nest")
                .clone()
                .merge_content_p2()
        };

        let mut queue = VecDeque::new();
        while let Some(new_raw) = content.next(){
            let new_value =new_raw.clone().merge_content_p2().get_val();
            match operation{
                Node::Mult => {
                    queue.push_back(current_value);
                    current_value = new_value;
                },
                Node::Add => {
                    #[cfg(test)]
                    println!("{current_value} {operation} {new_value} = {}",current_value + new_value);
                    current_value += new_value;
                },
                Node::LParen | Node::RParen | Node::Val(_) => panic!("Should never happen!"),
            }
            if let Some(new_opr) = content.next(){
                operation = new_opr.clone().merge_content_p2();
            }else{
                break;
            }            
        }
        // Now perform multiplication:
        let product = queue
            .iter()
            .fold(current_value, |acc,v| {
                #[cfg(test)]
                println!("{acc} * {v} = {}",acc*v);
                acc*v
            });
        product.to_node().expect("u128.to_node() is infallable")
    }
}

pub fn main_2(file_name:&str)->Option<u128>{
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
        sum += current_nest.merge_content_p2().get_val();
    }
    Some(sum)

}

#[cfg(test)]
    mod tests{
}
