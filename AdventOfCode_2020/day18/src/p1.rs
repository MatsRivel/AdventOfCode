use std::{fs::read_to_string, vec, collections::VecDeque, fmt::{Debug, Display, write}};
#[derive(Clone, Copy, Debug)]
enum Opr{
    Mult,
    Add,
    LParen,
    RParen,
    Val(u64)
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
#[derive(Clone)]
struct Cal{
    val: Opr,
    contents: Vec<Cal>
}
impl Cal{
    fn merge_contents(&mut self)->Opr{
        if self.contents.len() == 0{
            return self.val;
        }
        let mut cal_iter = self.contents.iter_mut();
        let mut left_opr = cal_iter.next().unwrap().merge_contents();
        let mut middle_opr = cal_iter.next().unwrap().merge_contents();
        let mut current_value = {
            loop{
                let val = match left_opr{
                    Opr::Val(v) => v,
                    _ => {
                        left_opr = middle_opr;
                        middle_opr = cal_iter.next().unwrap().merge_contents();
                        continue;
                    },
                };
                break val;
            }
        };
        while let Some(right_cal) = cal_iter.next() {
            if let Opr::Val(right_value) = right_cal.val{
                match middle_opr{
                    Opr::Mult => current_value *= right_value,
                    Opr::Add => current_value += right_value,
                    _ => ()
                }
            }
            middle_opr = right_cal.val;
        }
        Opr::Val(current_value)

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
    let mut stack = VecDeque::<Cal>::new();
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
                            true => Some(Opr::Val(v.to_digit(10).unwrap() as u64)),
                            false => None
                        }
                    }
                })
            })
    };
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
    current_cal
}
pub fn main_1(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let mut sub_groups = extract_sub_groups(&data_string);
    let final_val = sub_groups.merge_contents();
    if let Opr::Val(output) = final_val{
        return Some(output);
    }
    panic!("Failed to find solution!")

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
