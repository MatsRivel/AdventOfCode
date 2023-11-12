use std::{fs::read_to_string, vec, collections::VecDeque, fmt::{Debug, Display, write}};
#[derive(Clone, Copy, Debug)]
enum Opr{
    Mult,
    Add,
    LParen,
    RParen,
    Val(u128)
}
impl Opr{
    fn to_cal(&mut self) -> Cal{
        Cal{val:*self, contents: vec![]}
    }
}
impl Display for Opr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self{
            Opr::Mult => "*".to_string(),
            Opr::Add => "+".to_string(),
            Opr::LParen => "(".to_string(),
            Opr::RParen => ")".to_string(),
            Opr::Val(v) => v.to_string(),
        };
        write!(f,"{text}")
    }
}
#[derive(Clone,Debug)]
struct Cal{
    val: Opr,
    contents: Vec<Cal>
}
impl Cal{
    fn merge_contents(&mut self)->Opr{
        if self.contents.len() == 0{
            return self.val;
        }
        println!();
        let mut cal_iter = self.contents.iter_mut();
        let mut left_opr;
        if let Opr::Val(v) = self.val{
            left_opr = Opr::Val(v)
        }else{
            // print!("{} ",self.val);
            left_opr = cal_iter.next().unwrap().merge_contents();
        }
        // print!("{left_opr} ");
        let mut middle_opr = cal_iter.next().unwrap().merge_contents();
        // print!("{middle_opr} ");
        let mut current_value = {
            loop{
                let val = match left_opr{
                    Opr::Val(v) => {
                        v},
                    _ => {
                        left_opr = middle_opr;
                        middle_opr = cal_iter.next().unwrap().merge_contents();
                        // println!("{middle_opr} ");
                        continue;
                    },
                };
                break val;
            }
        };
        // TODO: NOTE TO SELF: The issue is that currently + is prioritized over ) if all that is left to do to the left of it is * ? Maybe?
        let mut val_stack: VecDeque<u128> = VecDeque::new();
        while let Some(right_cal) = cal_iter.next() {
            // print!("{right_cal} ");
            if let Opr::Val(right_value) = right_cal.merge_contents(){
                match middle_opr{
                    Opr::Mult => {
                        val_stack.push_back(current_value);
                        current_value = right_value;                        
                        
                    },
                    Opr::Add => {
                        print!("{current_value} + {right_value} = ");
                        current_value += right_value;
                        println!("{current_value}")
                    },
                    _ => ()
                }
                
            }
            middle_opr = right_cal.val;
        }
        println!();
        let sum = val_stack.iter().fold(current_value, |acc,v| {println!("{acc} * {v} = {}",acc*v); acc*v});
        Opr::Val(sum)

    }
}
impl Display for Cal{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.val).unwrap();
        for content in &self.contents{
            write!(f," {content}").unwrap();
        }
        Ok(())
    }
}
fn extract_sub_groups(data_string:&str)-> Cal{
    let mut all_oprs = {
        data_string
            .lines()
            .flat_map(|line| {
                line.chars().filter_map(|c| {
                    match c{
                        '*' => Some(Opr::Mult),
                        '+' => Some(Opr::Add),
                        '('=> Some(Opr::LParen),
                        ')' => Some(Opr::RParen),
                        v => match v.is_numeric(){
                            true => Some(Opr::Val(v.to_digit(10).unwrap() as u128)),
                            false => None
                        }
                    }
                })
            })
    };
    #[cfg(none)]{
        let clone_opr = all_oprs.clone().collect::<Vec<Opr>>();
        for opr in clone_opr{
            print!("{opr} ");
        }
        println!("\n_____________________________")
    }

    let mut stack = VecDeque::<Cal>::new();
    let mut current_cal = Cal{val:all_oprs.next().unwrap(), contents: vec![]};
    for mut opr in all_oprs{
        match opr{
            Opr::Mult | Opr::Add | Opr::Val(_)=> current_cal.contents.push(opr.to_cal()),
            Opr::LParen => {
                stack.push_back(current_cal.clone());
                current_cal = opr.to_cal();
            },
            Opr::RParen => {
                current_cal.contents.push(opr.to_cal());
                if let Some(mut old_cal) = stack.pop_back(){
                    old_cal.contents.push(current_cal.clone());
                    current_cal = old_cal;
                }

            }
        }
    }
    #[cfg(none)]
    println!("{current_cal}");
    current_cal
}
pub fn main_2(file_name:&str)->Option<u128>{
    let data_string = read_to_string(file_name).unwrap();
    let mut vals = vec![];
    for line in data_string.lines(){
        let mut sub_group = extract_sub_groups(line);
        if let Opr::Val(val) = sub_group.merge_contents(){
            vals.push(val);
        }
    }
    let output = vals.iter().fold(0, |acc,v| acc+v);
    Some(output)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
