use std::{fs::read_to_string, collections::HashMap, fmt::{Display, write}};
use anyhow::{anyhow,Error, Context};

pub fn get_rule(row:&str)->Result<(u32,Vec<Vec<String>>),anyhow::Error>{
    let split =  row.split(": ").map(|s| s ).collect::<Vec<&str>>();
    if split.len() < 2{
        return Err(anyhow!("Skipping empty string."));
    }
    let [name,content]: [&str;2] = [split[0],split[1]];
    let either_rules = content.split(" | ").map(|local_all_rules| local_all_rules).collect::<Vec<&str>>();
    let all_rules = either_rules
        .iter()
        .map(|rule_group|{
            rule_group.split(" ").map(|inner_rule| inner_rule.to_string()).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();
    let name_key = name.parse::<u32>().with_context(|| anyhow!("Failed to convert {name} to u32"))?;
    Ok((name_key, all_rules))

}
pub fn get_rules(data_string:&str)-> (HashMap<u32,Vec<Vec<String>>>, Vec<String>){
    // "rules" hold <rule_name,Vec<Vec<Rule_set_1>,Rule_set_2>>
    let rules = data_string
        .lines()
        .filter_map(|line| {
            if !line.starts_with("a") && !line.starts_with("b"){
                match get_rule(line){
                    Ok(v) => Some(v),
                    Err(_) => None
                }
            }else{
                None
            }
        }).collect::<HashMap<u32,Vec<Vec<String>>>>();

    let inputs = data_string
        .lines()
        .filter_map(|line| {
            if line.starts_with("a") || line.starts_with("b"){
                Some(line.to_string())
            }else{
                None
            }
        }).collect::<Vec<String>>();
    (rules,inputs)
}

#[derive(PartialEq,Copy,Clone,Debug)]
enum CoreRule{
    a,
    b
}
impl Display for CoreRule{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            CoreRule::a => write!(f,"a"),
            CoreRule::b => write!(f,"b")
        }
        
    }
}
impl Display for Rule{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self}")
    }
}
impl Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.set_a.iter(){
            write!(f,"{r}")?;
        }
        for r in self.set_b.iter(){
            write!(f,"{r}")?;
        }
        write!(f,"")
    }
}
trait IntoCoreRule<Char>{
    fn to_core_rule(&self) -> Option<CoreRule>;
}
impl IntoCoreRule<char> for char{
    fn to_core_rule(&self) -> Option<CoreRule> {
        match self{
            'a' => Some(CoreRule::a),
            'b' => Some(CoreRule::b),
            _ => None

        }
    }
}
trait IntoRule<Char>{
    fn to_rule(&self) -> Option<Rule>;
}
impl IntoRule<char> for char{
    fn to_rule(&self) -> Option<Rule> {
        match self{
            'a' => {
                Some(Rule::CoreRule { rule: 'a'.to_core_rule().unwrap() })},
            'b' => Some(Rule::CoreRule { rule: 'b'.to_core_rule().unwrap() }),
            _ => None

        }
    }
}

#[derive(Clone,Debug)]
struct Logic{
    set_a: Vec<u32>,
    set_b: Vec<u32>
}
#[derive(Clone,Debug)]
enum Rule{
    CoreRule{ rule: CoreRule },
    RuleLogic{ logic: Logic }
}

fn recursively_traverse_rules(rules: &HashMap<u32, Rule>, current_key:u32, active_input:&Vec<CoreRule>, active_idx:usize) -> bool{
    let current_rule = rules.get(&current_key).unwrap();
    println!(" {current_rule:?}")
    match current_rule{
        Rule::CoreRule { rule } => return &active_input[active_idx] == rule,
        Rule::RuleLogic { logic } => {
            let a_matches = logic
                .set_a
                .iter()
                .enumerate()
                .map(|(i,new_key)|{
                    recursively_traverse_rules(rules, *new_key, active_input, active_idx+i)
                }).fold(true, |acc, b| acc && b);
            if !a_matches{
                let b_matches = logic
                    .set_b
                    .iter()
                    .enumerate()
                    .map(|(i,new_key)|{
                        recursively_traverse_rules(rules, *new_key, active_input, active_idx+i)
                    }).fold(true, |acc, b| acc && b);
                return b_matches;
            }else{
                return true;
            }
        },
    }
}

fn clean_rules(raw_rules: &HashMap<u32, Vec<Vec<String>>>) -> HashMap<u32, Rule>{
    raw_rules.iter().map(|(key,val)|{
        if      val[0][0] == "\"a\""{ (*key,'a'.to_rule().unwrap()) }
        else if val[0][0] == "\"b\""{ (*key,'b'.to_rule().unwrap()) }
        else{
            let set_a = val[0].iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let set_b = val[0].iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            (*key, Rule::RuleLogic { logic: Logic { set_a, set_b } })
        }
    }).collect::<HashMap<u32,Rule>>()
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (raw_rules, inputs) = get_rules(&data_string);
    let rules = clean_rules(&raw_rules);
    todo!()
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn rules_test(){
        let file_name = r"src\dummy.txt";
        let data_string = read_to_string(file_name).unwrap();
        let (raw_rules, raw_inputs) = get_rules(&data_string);
        let rules = clean_rules(&raw_rules);
        let inputs = raw_inputs
            .iter()
            .map(|s| 
                s.chars().map(|c| 
                    c.to_core_rule().unwrap() 
                ).collect::<Vec<CoreRule>>()
            ).collect::<Vec<Vec<CoreRule>>>();

        let mut counter = 0;
        for input in inputs.iter() {
            if recursively_traverse_rules(&rules, 0, input, 0){
                counter += 1;
            }
        }
        println!("Counter: {counter}");
        assert!(false)    
    }

}
