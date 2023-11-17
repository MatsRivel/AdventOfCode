use std::{fs::read_to_string, fmt::Display, collections::HashMap, ops::Index};
use anyhow::{Error,Context,anyhow};

// Structures
#[derive(PartialEq,Copy,Clone,Debug)]
pub enum CoreRule{
    A,
    B
}
pub struct CoreRuleVec{
    content: Vec<CoreRule>
}
pub struct AltRules{
    alt_rules: Vec<CoreRuleVec>
}
#[derive(Clone,Debug)]
pub struct Logic{
    set: Vec<u32>,
}
pub struct LogicIterator{
    idx:usize,
    set: Vec<u32>
}
impl Iterator for LogicIterator{
    type Item=u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.set.len(){
            let output = Some(self.set[self.idx]);
            self.idx += 1;
            output
        }else{
            None
        }
    }
}

#[derive(Clone,Debug)]
pub enum Rule{
    CoreRule( CoreRule ),
    RuleLogic( Logic )
}

// Custom Traits:
trait IntoCoreRule<Char>{
    fn to_core_rule(&self) -> Option<CoreRule>;
}
impl IntoCoreRule<char> for char{
    fn to_core_rule(&self) -> Option<CoreRule> {
        match self{
            'a' => Some(CoreRule::A),
            'b' => Some(CoreRule::B),
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
            'a' => Some(Rule::CoreRule ('a'.to_core_rule().unwrap() )),
            'b' => Some(Rule::CoreRule( 'b'.to_core_rule().unwrap())),
            _ => None

        }
    }
}
impl Index<usize> for CoreRuleVec {
    type Output = CoreRule;

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}
impl Index<usize> for Logic {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.set[index]
    }
}
// Displays
impl Display for CoreRule{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            CoreRule::A => write!(f,"a"),
            CoreRule::B => write!(f,"b")
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
        // let string = self.set.iter().map(|r| format!("{r}")).collect::<String>();
        write!(f,"")

    }
}
impl Display for CoreRuleVec{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let string = self.content.iter().map(|core| format!("{core}") ).collect::<String>();
        write!(f,"")
    }
}

// Functions
fn get_rule(row:&str)->Result<(u32,Vec<Rule>),anyhow::Error>{
    let split =  row.split(": ").map(|s| s ).collect::<Vec<&str>>();
    if split.len() < 2{
        return Err(anyhow!("Skipping empty string."));
    }
    let [name,content]: [&str;2] = [split[0],split[1]];
    let either_rules = content.split(" | ").map(|local_all_rules| local_all_rules).collect::<Vec<&str>>();
    let all_rules = either_rules
        .iter()
        .filter_map(|rule_group|{
            let rules = rule_group.split(" ").collect::<Vec<&str>>();
            if rules.len() == 1{
                match rules[0]{
                    "\"a\"" => 'a'.to_rule(),
                    "\"b\"" => 'b'.to_rule(),
                    _ => None
                }
            }else{
                let nums = rules.iter().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>();
                Some( Rule::RuleLogic (Logic{set: nums} ))
            }
        }).collect::<Vec<Rule>>();
    let name_key = name.parse::<u32>().with_context(|| anyhow!("Failed to convert {name} to u32"))?;
    Ok((name_key, all_rules))
}
pub fn get_rules(data_string:&str)-> (HashMap<u32,Vec<Rule>>, Vec<CoreRuleVec>){
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
        }).collect::<HashMap<u32,Vec<Rule>>>();

    let inputs = data_string
        .lines()
        .filter_map(|line| {
            if line.starts_with("a") || line.starts_with("b"){
                let content = line.chars().filter_map(|c| c.to_core_rule()).collect::<Vec<CoreRule>>();
                Some(CoreRuleVec{content})
            }else{
                None
            }
        }).collect::<Vec<CoreRuleVec>>();

    (rules,inputs)
}

fn traverse_rules(rules:&HashMap<u32,Vec<Rule>>, problem:&CoreRuleVec, key:&u32, depth:usize)->Option<Vec<CoreRule>>{
    let rule_set = rules.get(&key).expect("Infallable, as we traverse known keys.");
    for rule in rule_set.iter(){
        match rule{
            Rule::CoreRule(core) =>{
                let does_match = problem[depth] == *core;
                #[cfg(test)]{
                    for core_rule in problem.content.iter(){
                        print!("{core_rule}");
                    }
                    println!("[{depth}] == {core} <-- {}", does_match);
                    for _ in 0..depth{
                        print!(" ");
                    }
                    println!("^");
                }
                match does_match{
                    true => return Some(vec![problem.content[depth]]),
                    false => return None,
                }
            },
            Rule::RuleLogic(new_rule) => {
                for new_key in new_rule.set.iter(){
                    if let Some(mut old_vec) = traverse_rules(rules, problem, new_key, depth+1){
                        let mut new_vec = vec![problem.content[depth]];
                        new_vec.append(&mut old_vec);
                        return Some(new_vec);
                    }
                }
            },
        }
    }
    None
}

fn traverse_rules_initializer(rules:&HashMap<u32,Vec<Rule>>, problem:&CoreRuleVec)->Option<Vec<CoreRule>>{
    let ans = traverse_rules(&rules, problem, &0, 0);
    ans
}

fn rec_merge_rules(rules:&HashMap<u32,Vec<Rule>>, current_key:u32, memo: &mut HashMap<u32, Vec<CoreRuleVec>>)->Vec<CoreRuleVec>{
    let current_rules = rules.get(&current_key).expect("We only traverse existing keys");
    let mut output = vec![];
    for rule in current_rules.iter(){
        match rule{
            Rule::CoreRule(core) => output.push(CoreRuleVec{ content: vec![*core]}) ,
            Rule::RuleLogic(logic) => {
                for new_key in logic.set.iter(){
                    let new_core = rec_merge_rules(rules, *new_key, memo);
                    output.append( new_core );
                }
            },
        }
    }
    todo!()
}
fn merge_rules(rules:&HashMap<u32,Vec<Rule>>, initial_key:u32) -> CoreRuleVec{
    let mut memo = HashMap::<u32,CoreRuleVec>::new();

    todo!()
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (rules,inputs) = get_rules(&data_string);
    for input in inputs.iter(){
        let ans = traverse_rules(&rules, input, &0, 0);
        println!("\n");

    }
    todo!()

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn simplified_dummy_test(){
        let data_string = read_to_string(r"src\dummy_simplified.txt").unwrap();
        let (rules,inputs) = get_rules(&data_string);
        let solutions = vec![false,false,true,true,false];
        for (idx, input) in inputs.iter().enumerate(){
            let ans = traverse_rules_initializer(&rules, &input);
            println!("{ans:?}");
            assert_eq!(solutions[idx],ans.is_some())
            
        }
        assert!(false)
    }

}
