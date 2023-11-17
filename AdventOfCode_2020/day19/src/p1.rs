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
            CoreRule::A => write!(f,"A"),
            CoreRule::B => write!(f,"B")
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
            if rules.len() == 1 && (rules[0] == "\"a\"" || rules[0] == "\"b\""){
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
pub fn get_rules(data_string:&str)-> (HashMap<u32,Vec<Rule>>, Vec<Vec<CoreRule>>){
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
                Some(content)
            }else{
                None
            }
        }).collect::<Vec<Vec<CoreRule>>>();

    (rules,inputs)
}

fn compare_states(current_state: &Vec<CoreRule>, target_state: &Vec<CoreRule>) -> bool{
    current_state.iter().zip(target_state.iter()).fold(true, |acc,(current,real)| {
        #[cfg(test)]
        print!("{current} vs {real} ->  {} && ", current == real);
        acc && (current == real)
    })
}
fn traverse_rules(rules:&HashMap<u32,Vec<Rule>>, input:&Vec<CoreRule>, current_key:&u32, current_state: &Vec<CoreRule>)->Option<Vec<CoreRule>>{
    // println!("Current_key: {current_key}");
    let current_rules = rules.get(current_key).expect("We only traverse existing keys!");
    // println!("Current_rules: {current_rules:?}");
    'outer: for rule in current_rules.iter() {
        match rule{
            Rule::CoreRule(core) => {
                // If we find a leaf, send it back to compose the whole.
                let mut new_state = current_state.clone() ;
                new_state.push(*core);
                return Some(new_state);
            },
            Rule::RuleLogic(logic) => {
                // Not a leaf, so we must continue to find leaves.
                // ALL "logic" content must be valid, so if any is not valid, we abort this rule and check the next.
                let mut must_fulfill: Vec<CoreRule> = vec![];
                for new_key in logic.set.iter(){
                    let mut new_solution = match traverse_rules(rules, input, new_key, current_state){
                        Some(v) => v,
                        None => continue 'outer // All logic-rules must be fulfilled, but only one of the outer rules must be!
                    };
                    must_fulfill = {
                        new_solution.append(&mut must_fulfill);
                        new_solution
                    }
                }
                // If all "logic" matches, we've found a path that is valid so far, so we can return it. No need to find other valid end-paths.
                // Note: We reverse the "must_fulfill" vec because our suggested solution appears in reverse, but the input appears in the correct order. 
                let matches_so_far = compare_states(&must_fulfill, input);
                #[cfg(test)]{
                    println!();
                    for _ in 0..input.len()-must_fulfill.len(){
                        print!(" ")
                    }
                    for cr in must_fulfill.iter(){
                        print!("{cr}");
                    }     
                    println!();
                    for inp in input.iter(){
                        print!("{inp}");
                    } 
                    println!("\nmatches_so_far: {matches_so_far}");
                    println!();
                }
                if matches_so_far{
                    return Some(must_fulfill);
                }
                // If its not valid, then we continue checking the next rule.

            },
        }
    }

    None
}

fn traverse_rules_init(rules:&HashMap<u32,Vec<Rule>>, input:&Vec<CoreRule>) -> bool{
    traverse_rules(rules,input,&0,&vec![]).is_some() // If we find ANY solution, then we're good to go.
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (rules,inputs) = get_rules(&data_string);
    for input in inputs.iter(){

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
            assert_eq!(traverse_rules_init(&rules,input),solutions[idx], "idx == {idx} | {input:?}");
        }
        assert!(false)
    }
    #[test]
    fn dummy_trivial3_test(){
        let data_string = read_to_string(r"src\dummy_trivial3.txt").unwrap();
        let (rules,inputs) = get_rules(&data_string);
        let solutions = vec![false,true];
        for (idx, input) in inputs.iter().enumerate(){
            assert_eq!(traverse_rules_init(&rules,input),solutions[idx], "idx == {idx} | {input:?}");
        }
    }
    #[test]
    fn dummy_trivial4_test(){
        let data_string = read_to_string(r"src\dummy_trivial4.txt").unwrap();
        let (rules,inputs) = get_rules(&data_string);
        println!("Rules: {rules:?}");
        let solutions = vec![false,true];
        for (idx, input) in inputs.iter().enumerate(){
            assert_eq!(traverse_rules_init(&rules,input),solutions[idx], "idx == {idx} | {input:?}");
        }
    }
    #[test]
    fn dummy_trivial5_test(){
        let data_string = read_to_string(r"src\dummy_trivial5.txt").unwrap();
        let (rules,inputs) = get_rules(&data_string);
        println!("Rules: {rules:?}");

        let solutions = vec![false,true];
        for (idx, input) in inputs.iter().enumerate(){
            assert_eq!(traverse_rules_init(&rules,input),solutions[idx], "idx == {idx} | {input:?}");
        }
    }

}
