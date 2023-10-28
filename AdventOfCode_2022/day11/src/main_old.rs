mod p1{
    use core::panic;
    use std::{fs::read_to_string, vec, collections::{HashMap,VecDeque}};
    #[derive(Debug)]
    struct PartialMathOperation{
        a:Option<u128>,
        b:Option<u128>,
        func: fn(u128,u128) -> u128
    }
    impl PartialMathOperation{
        fn perform_operation(&self,num_in:u128) -> u128{
            match (self.a.clone(),self.b.clone()){
                (Some(v),None) => (self.func)(v,num_in),
                (None,Some(v)) => (self.func)(num_in,v),
                (Some(v),Some(w)) => (self.func)(v,w),
                (None,None) => (self.func)(num_in,num_in)
            }
        }
    }
    #[derive(Debug)]
    struct PartialBoolOperation{
        a:u128,
        func: fn(u128,u128) -> bool
    }
    impl PartialBoolOperation{
        fn perform_operation(&self,num_in:u128) -> bool{
            (self.func)(num_in,self.a)
        }
    }
    #[derive(Debug)]
    struct MonkeySelection{
        if_true:u128,
        if_false:u128
    }
    impl MonkeySelection {
        fn perform_action(&self,test_resut:bool) -> u128{
            match test_resut {
                true => self.if_true,
                false => self.if_false
            }
        }
    }
    #[derive(Debug)]
    struct Ape{
        name:u128,
        current_items:VecDeque<u128>,
        operation: PartialMathOperation,
        test: PartialBoolOperation,
        next_monkey: MonkeySelection, 
    }

    fn add(a:u128,b:u128) -> u128{
        a + b
    }
    fn sub(a:u128,b:u128) -> u128{
        a - b
    }
    fn mult(a:u128,b:u128) -> u128{
        a * b
    }
    fn div(a:u128,b:u128) -> u128{
        a / b
    }
    fn is_divisible(a:u128,b:u128) -> bool{
        a%b == 0u128
    }
    impl Ape{
        fn new(lines:&Vec<&str>) -> Self{
            let name = lines[0].strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap().parse::<u128>().unwrap();
            let current_items = lines[1].strip_prefix("  Starting items: ").unwrap().split(" ")
                .map(|s| {
                    let num_str = match  s.contains(","){
                        true => s.trim().strip_suffix(",").unwrap(),
                        false => s.trim(),
                    };
                    let num = num_str.parse::<u128>().unwrap();
                    num
                }).collect::<VecDeque<u128>>();
            let core_operation:[&str;3] = lines[2].strip_prefix("  Operation: new = ").unwrap().split(" ").collect::<Vec<&str>>().try_into().unwrap();
            let inner_operation = match core_operation[1]{
                "*" => mult,
                "+" => add,
                "-" => sub,
                "/" => div,
                _ => panic!("Unknown operation in ape-action-creation! {:?}",core_operation[1])
            };
            let operation= match [core_operation[0], core_operation[2]]{
                ["old","old"] => {
                    PartialMathOperation{
                        a: None,
                        b: None,
                        func: inner_operation,
                    }
                },
                ["old",v] => {
                    let num = v.parse::<u128>().expect(format!("Failed to parse {v} into u128").as_str());
                    PartialMathOperation{
                        a: None,
                        b: Some(num),
                        func: inner_operation,
                    }
                },
                [v,"old"] => {
                    let num = v.parse::<u128>().expect(format!("Failed to parse {v} into u128").as_str());
                    PartialMathOperation{
                        a: Some(num),
                        b: None,
                        func: inner_operation,
                    }
                },
                [v,w] => {
                    let num1 = v.parse::<u128>().expect(format!("Failed to parse {v} into u128").as_str());
                    let num2 = w.parse::<u128>().expect(format!("Failed to parse {w} into u128").as_str());
                    PartialMathOperation{
                        a: Some(num1),
                        b: Some(num2),
                        func: inner_operation,
                    }
                },
                _ => panic!("Invalid core operation components! {core_operation:?}"),
            };
            let div_num_test = lines[3].strip_prefix("  Test: ").unwrap().split(" ").filter_map(|s|{
                if s == " " || s == "divisible" || s == "by"{
                    None
                }else{
                    Some(s.parse::<u128>().unwrap())
                }
            }).fold(0u128, |acc,val| acc+val); // "Fold" one value, as the rest is filtered out.
            let test = PartialBoolOperation{
                a:div_num_test,
                func: is_divisible
            };
            let if_true = {
                let mut temp_str = lines[4].trim();
                temp_str = temp_str.strip_prefix("If true: throw to monkey ").expect("Failed to strip prefix");
                let temp_int = temp_str.parse::<u128>().expect(format!("Failed to parse {temp_str} into u128").as_str());
                temp_int
            };
            let if_false = {
                let mut temp_str = lines[5].trim();
                temp_str = temp_str.strip_prefix("If false: throw to monkey ").expect("Failed to strip prefix");
                let temp_int = temp_str.parse::<u128>().expect(format!("Failed to parse {temp_str} into u128").as_str());
                temp_int
            };
            let next_monkey = MonkeySelection{
                if_true,
                if_false
            };
            Ape { name, current_items, operation, test, next_monkey }

        } 
        fn update_worry(&self, current_worry:u128) -> u128{
            self.operation.perform_operation(current_worry)
        }
        fn get_next_monkey(&self, current_worry:u128) -> u128{
            let test_result = self.test.perform_operation(current_worry);
            self.next_monkey.perform_action(test_result)
        }
    }
    fn process_data_string(data_string:&str)->HashMap<u128,Ape>{
        let preprocessed_apes = {data_string
            .lines()
            .enumerate()
            .fold(vec![],|mut acc, (raw_idx,line)|{
                let idx = raw_idx +1;
                if idx%7 == 0{
                    acc
                }else{
                    if idx%7 == 1{
                        acc.push( vec![line] );
                    }else{
                        let acc_size = acc.len()-1;
                        acc[acc_size].push(line);
                    }
                    acc
                }
            })
        };
        let apes = preprocessed_apes
            .iter()
            .map(|ape_vec|{
                let ape = Ape::new(ape_vec);
                let ape_name = ape.name;
                (ape_name, ape)

            }).collect::<HashMap<u128,Ape>>();
        return apes;
    }
    
    pub fn main_1(file_name:&str,monkey_business_divider:u128, n_rounds:u128)->Option<u128>{
        let data_string = read_to_string(file_name).unwrap();
        println!("String read!");
        let mut monkeys = process_data_string(&data_string);
        println!("Monkeys fetched!");
        let mut counters:HashMap<u128,u128> = monkeys
            .iter()
            .map(|(name,_)| {
                (*name,0)
            }).collect::<HashMap<u128,u128>>();
        println!("Counters initialized!");
        for n in 0..n_rounds{
            for monkey_idx in 0u128..(monkeys.len() as u128){
                let monkey = monkeys.get_mut(&monkey_idx).expect(format!("Monkey {} not found!",monkey_idx).as_str());
                let mut redistribution_order = VecDeque::<(u128,u128)>::new();
                let count = counters.get_mut(&monkey_idx).expect(format!("Monkey {} not found in counters!",monkey_idx).as_str());
                let mut inner_count = 0;
                while let Some(worry_score) = monkey.current_items.pop_front(){
                    *count += 1;
                    let new_worry_score: u128 = monkey.update_worry(worry_score) / monkey_business_divider as u128; // Div by 3 as stated in problem part1.
                    let new_monkey:u128 = monkey.get_next_monkey(new_worry_score);
                    redistribution_order.push_back((new_monkey,new_worry_score));
                    // Testing that things are as expected...
                    if file_name == r"src\dummy_input.txt" && monkey_business_divider == 3 && n_rounds == 20{
                        if monkey_idx == 0 && n == 0{
                            match inner_count{
                                0 => assert_eq!(worry_score,79u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 79u128),
                                1 => assert_eq!(worry_score,98u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 98u128),
                                _ => panic!("More items than expected at Monkey: {monkey_idx}, n: {n} and c: {inner_count}")
                            }
                        }else if monkey_idx == 0 && n == 1{
                            match inner_count{
                                0 => assert_eq!(worry_score,20u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 20u128),
                                1 => assert_eq!(worry_score,23u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 23u128),
                                2 => assert_eq!(worry_score,27u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 27u128),
                                3 => assert_eq!(worry_score,26u128,"Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 26u128),
                                _ => panic!("More items than expected at Monkey: {monkey_idx}, n: {n} and c: {inner_count}")
                            }
                        }else if monkey_idx == 1 && n == 1{
                            match inner_count{  
                                0 => assert_eq!(worry_score,2080u128,   "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 2080u128),
                                1 => assert_eq!(worry_score,25u128,     "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 25u128),
                                2 => assert_eq!(worry_score,167u128,    "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 167u128),
                                3 => assert_eq!(worry_score,207u128,    "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 207u128),
                                4 => assert_eq!(worry_score,401u128,    "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 401u128),
                                5 => assert_eq!(worry_score,1046u128,   "Monkey {monkey_idx}: Worry score is {worry_score}, but expected it to be {} at n: {n} and c: {inner_count}", 1046u128),
                                _ => panic!("More items than expected at Monkey: {monkey_idx}, n: {n} and c: {inner_count}")
                            }
                        }
                    }
                    inner_count += 1;
                }
                // Redistribute items to apes
                for (m_idx,worry) in redistribution_order.iter(){
                    let monkey = monkeys.get_mut(&m_idx).expect(format!("Monkey {} not found!",m_idx).as_str());
                    monkey.current_items.push_back(worry.clone()); 
                } 
            }
        }
        let best_scores: [u128;2] = {
            counters
                .iter()
                .map(|(_,score)| {
                    *score
                }).fold([0u128,0u128], |mut acc, val|{
                    if val >= acc[0]{
                        acc[1] = acc[0].clone();
                        acc[0] = val;
                    }else if val > acc[1]{
                        acc[1] = val;
                    }
                    acc
                })
        };
        let final_score = best_scores[0].clone() * best_scores[1].clone();
        Some(final_score)
    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn monkey_operation_test(){
        // Note: Part1 can be solved using only u128 and never overflow.
        // Part2 requires u128. Because of this I added u128 to part 1 as well.
        // This has slowed down the performance incredibly...
        let file_name = r"src\dummy_input.txt"; 
        let data_string = read_to_string(file_name).unwrap();
        let mut monkeys = process_data_string(&data_string);
        let expected_results = [19,7,1,4];
        for monkey_idx in 0..(monkeys.len() as u128){
            let monkey = monkeys.get_mut(&monkey_idx).unwrap();
            let operation_result = monkey.operation.perform_operation(u128::from(1u128));
            let idx = monkey_idx as usize;
            let expected_result = expected_results[idx] as u128;
            assert_eq!(operation_result, expected_result, "Operation for Monkey {monkey_idx} failed!");
        }   
	}
    #[test]
	fn monkey_test_test(){
        let file_name = r"src\dummy_input.txt"; 
        let data_string = read_to_string(file_name).unwrap();
        let mut monkeys = process_data_string(&data_string);
        let worries = [22,18,12,16];
        let expected_results = [false;4];
        for monkey_idx in 0..(monkeys.len() as u128){
            let monkey = monkeys.get_mut(&monkey_idx).unwrap();
            let idx = monkey_idx as usize;
            let worry = worries[idx];
            let operation_result = monkey.test.perform_operation(u128::from(worry as u128));
            let expected_result = expected_results[idx];
            assert_eq!(operation_result, expected_result, "Test for Monkey {monkey_idx} failed!");
        }  
        let worries = [23*5,19*1,13*500,17*8];
        let expected_results = [true;4];
        for monkey_idx in 0..(monkeys.len() as u128){
            let monkey = monkeys.get_mut(&monkey_idx).unwrap();
            let idx = monkey_idx as usize;
            let worry = worries[idx];
            let operation_result = monkey.test.perform_operation(u128::from(worry as u128));
            let expected_result = expected_results[idx];
            assert_eq!(operation_result, expected_result, "Test for Monkey {monkey_idx} failed!");
        }  
	
	}
	
}

}
mod p2{
    use core::panic;
    use std::{collections::{HashMap,VecDeque}, fs::read_to_string};

    struct Item{
        ape_spesific_value:u32, // HashMap< ApeIdx, ApeDivisor>
    }
    impl Item{
        fn update_value(&mut self, ape_idx:&u32, operation: &Operation, divisor:&u32){
            let current_value = self.ape_spesific_value;
            let opr_res = match perform_operation(&current_value,operation){
                OperationResult::ABool(_) => panic!("Should put bools into value updates!"),
                OperationResult::AnInt(v) => v
            };
            if divisor == &0{
                panic!("Can't div by 0!")
            }
            self.ape_spesific_value = opr_res % *divisor;
        }
        fn transfer_value(&mut self, old_div:u32, new_div:u32){
            if new_div == 0{
                panic!("Can't div by 0!")
            }
            self.ape_spesific_value = (self.ape_spesific_value*old_div) % new_div
        }

    }
    enum Operation{
        Add(Option<u32>),
        Sub(Option<u32>),
        Div(Option<u32>),
        Mult(Option<u32>),
        Larger(u32),
        Divisible(u32)
    }
    enum OperationResult{
        AnInt(u32),
        ABool(bool)
    }
    fn perform_operation(num:&u32,opr:&Operation) ->OperationResult{
        match opr{
            Operation::Add(v) => {
                let other_num = match v{
                    Some(x) => x,
                    None => num
                };
                OperationResult::AnInt(num+other_num)
            },
            Operation::Sub(v) => {
                let other_num = match v{
                    Some(x) => x,
                    None => num
                };
                OperationResult::AnInt(num-other_num)
            },
            Operation::Div(v) => {
                let other_num = match v{
                    Some(x) => x,
                    None => num
                };
                OperationResult::AnInt(num/other_num)
            },
            Operation::Mult(v) => {
                let other_num = match v{
                    Some(x) => x,
                    None => num
                };
                OperationResult::AnInt(num*other_num)
            },
            Operation::Larger(v) => OperationResult::ABool(num>v),
            Operation::Divisible(v) => OperationResult::ABool(num%v==0),
        }
    }
    struct Ape{
        name:u32,
        current_items:VecDeque<Item>,
        operation: Operation,
        divisor:u32,
        test: Operation,
        test_succeeded:u32,
        test_failed:u32,
    }
    fn make_ape_name(s:&str)->u32{
        s.strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap().parse::<u32>().unwrap()
    }
    fn make_current_items(s:&str) -> VecDeque<Item>{
        s.strip_prefix("  Starting items: ").unwrap().split(" ")
        .map(|s| {
            let num_str = match  s.contains(","){
                true => s.trim().strip_suffix(",").unwrap(),
                false => s.trim(),
            };
            let num = num_str.parse::<u32>().unwrap();
            Item{ ape_spesific_value: num }
        }).collect::<VecDeque<Item>>()
    }
    fn make_operation(s:&str) -> Operation{
        let core_operation:[&str;3] = s.strip_prefix("  Operation: new = ").unwrap().split(" ").collect::<Vec<&str>>().try_into().unwrap();
        let [_,right] = match [core_operation[0],core_operation[2]]{
            ["old","old"] => [None,None],
            ["old", v] => [None, Some(v.parse::<u32>().expect("Parsing failed!"))],
            _ => panic!("Shouldn't occur!")
        };
        let operation = match core_operation[1]{
            "*" => Operation::Mult(right),
            "+" => Operation::Add(right),
            "-" => Operation::Sub(right),
            "/" => Operation::Div(right),
            _ => panic!("Unknown operation in ape-action-creation! {:?}",core_operation)
        };
        operation

    }
    fn make_div_test(s:&str) -> (Operation,u32){
        let div_num = s.strip_prefix("  Test: ").unwrap().split(" ").filter_map(|s|{
            if s == " " || s == "divisible" || s == "by"{
                None
            }else{
                Some(s.parse::<u32>().unwrap())
            }
        }).fold(0u32, |acc,val| acc+val); // "Fold" one value, as the rest is filtered out.
        (Operation::Divisible(div_num),div_num)
    }
    fn make_success_outcome(s:&str) -> u32{
        let mut temp_str = s.trim();
        temp_str = temp_str.strip_prefix("If true: throw to monkey ").expect("Failed to strip prefix");
        let temp_int = temp_str.parse::<u32>().expect(format!("Failed to parse {temp_str} into u32").as_str());
        temp_int
    }
    fn make_failiure_outcome(s:&str) -> u32{
        let mut temp_str = s.trim();
        temp_str = temp_str.strip_prefix("If false: throw to monkey ").expect("Failed to strip prefix");
        let temp_int = temp_str.parse::<u32>().expect(format!("Failed to parse {temp_str} into u32").as_str());
        temp_int
    }
    impl Ape{
        fn new(ape_vec:&Vec<&str>)->Self{
            let name = make_ape_name(ape_vec[0]);
            let current_items = make_current_items(ape_vec[1]);
            let operation = make_operation(ape_vec[2]);
            let (test,divisor) = make_div_test(ape_vec[3]);
            let test_succeeded = make_success_outcome(ape_vec[4]);
            let test_failed = make_failiure_outcome(ape_vec[5]);
            Self { name, current_items, operation, divisor, test, test_succeeded, test_failed }
        }
        fn item_to_monkey(&mut self)->Option<(u32,Item)>{
            if let Some(mut item) = self.current_items.pop_front(){
                let ape_idx = &self.name;
                let divisor = &self.divisor;
                let operation = &self.operation;
                item.update_value(&ape_idx, operation, divisor);
                let item_value = &item.ape_spesific_value;
                let test = &self.test;
                let next_monkey = match perform_operation(item_value, test){
                    OperationResult::ABool(b) => {
                        match b{
                            true => self.test_succeeded,
                            false => self.test_failed,
                        }
                    } ,
                    _ => panic!("Test result must be a boolean!"),
                };
                Some((next_monkey,item))
            }else{
                return None;
            }
        }
    }
    fn process_data_string(data_string:&str)->HashMap<u32,Ape>{
        let preprocessed_apes = {data_string
            .lines()
            .enumerate()
            .fold(vec![],|mut acc, (raw_idx,line)|{
                let idx = raw_idx +1;
                if idx%7 == 0{
                    acc
                }else{
                    if idx%7 == 1{
                        acc.push( vec![line] );
                    }else{
                        let acc_size = acc.len()-1;
                        acc[acc_size].push(line);
                    }
                    acc
                }
            })
        };
        let apes = preprocessed_apes
            .iter()
            .map(|ape_vec|{
                let ape = Ape::new(ape_vec);
                let ape_name = ape.name;
                (ape_name, ape)
            }).collect::<HashMap<u32,Ape>>();
        return apes;
    }
    pub fn main_2(file_name:&str, n_rounds:u128)->Option<u32>{
        let data_string = read_to_string(file_name).unwrap();
        println!("String read!");
        let mut monkeys = process_data_string(&data_string);
        println!("Monkeys fetched!");
        let mut counters:HashMap<u32,u32> = monkeys
            .iter()
            .map(|(name,_)| {
                (*name,0)
            }).collect::<HashMap<u32,u32>>();
        println!("Counters initialized!");
        for n in 0..n_rounds {
            for monkey_idx in 0u32..(monkeys.len() as u32){
                // println!("Processing Monkey {monkey_idx}...");
                let monkey = monkeys.get_mut(&monkey_idx).expect(format!("Monkey {} not found!",monkey_idx).as_str());
                let mut redistribution_order = VecDeque::<(u32,Item,u32)>::new();
                let count = counters.get_mut(&monkey_idx).expect(format!("Monkey {} not found in counters!",monkey_idx).as_str());
                let mut inner_count = 0;
                while let Some((next_monkey,item)) = monkey.item_to_monkey(){
                    *count += 1;
                    let old_div = monkey.divisor;
                    redistribution_order.push_back((next_monkey,item, old_div));
                    
                    inner_count += 1;
                }
                while let Some((new_monkey_idx,mut item, old_div)) = redistribution_order.pop_front(){
                    let monkey = monkeys.get_mut(&new_monkey_idx).expect(format!("Monkey {} not found!",new_monkey_idx).as_str());
                    let new_div = monkey.divisor;
                    let old_item_value = item.ape_spesific_value;
                    item.transfer_value(old_div, new_div);
                    if file_name == r"src\dummy_input.txt" && n == 0{
                        println!("Transfering from (Monkey {}, item {}) to (Monkey {}, item {})", monkey_idx, old_item_value, new_monkey_idx, item.ape_spesific_value)
                    }
                    monkey.current_items.push_back(item);
                }
            }
            if file_name == r"src\dummy_input.txt"{
                match n{
                    1    => {
                        for (&mnky, &cnt) in counters.iter(){
                            match mnky{
                                0 => assert_eq!(cnt,2),
                                1 => assert_eq!(cnt,4),
                                2 => assert_eq!(cnt,3),
                                3 => assert_eq!(cnt,6),
                                _ => panic!("Invalid monkey during live-testing!"),
                            }
                        }
                    },
                    2 => {
                        for (&mnky, &cnt) in counters.iter(){
                            match mnky{
                                0 => assert_eq!(cnt,99),
                                1 => assert_eq!(cnt,97),
                                2 => assert_eq!(cnt,8),
                                3 => assert_eq!(cnt,103),
                                _ => panic!("Invalid monkey during live-testing!"),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        let best_scores: [u32;2] = {
            counters
                .iter()
                .map(|(_,score)| {
                    *score
                }).fold([0u32,0u32], |mut acc, val|{
                    if val >= acc[0]{
                        acc[1] = acc[0].clone();
                        acc[0] = val;
                    }else if val > acc[1]{
                        acc[1] = val;
                    }
                    acc
                })
        };
        let final_score = best_scores[0].clone() * best_scores[1].clone();
        Some(final_score)
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
fn main() {
    let file_name = r"src\dummy_input.txt";
    // let file_name= r"src\puzzle_input.txt";
    println!("\nPart 1: ");
    let start = Instant::now();
    let count = main_1(file_name, 3, 20);
    let end = start.elapsed();
    println!("{count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.expect("No value returned to main"),10605u128,"\n--- Answer should be 10605! ---\n");
    }
    println!("\nPart 2: ");
    let start = Instant::now();
    let count = main_2(file_name,1000);
    let end = start.elapsed();
    println!("{count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.expect("No value returned to main"),2713310158u32,"\n--- Answer should be 2713310158! ---\n");
    }
}

