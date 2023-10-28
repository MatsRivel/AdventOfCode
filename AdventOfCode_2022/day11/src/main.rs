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
    use std::{collections::{VecDeque, HashSet}, fs::read_to_string};
    use num::integer::lcm;
    enum Operation{
        Mod_Add(Option<u128>),
        Mod_Sub(Option<u128>),
        Mod_Div(Option<u128>),
        Mod_Mult(Option<u128>)
    }
    struct Ape{
        divisor: u128, // Test == x%divisor==0
        operation: Operation,
        test_succeeded:usize,
        test_failed:usize,        
    }
    impl Ape{
        fn new(ape_vec:&Vec<&str>) -> (Self, VecDeque<u128>){
            // let name = make_ape_name(ape_vec[0]);
            let current_items = make_current_items(ape_vec[1]);
            let operation = make_operation(ape_vec[2]);
            let divisor = make_divisor(ape_vec[3]);
            let test_succeeded = make_success_outcome(ape_vec[4]);
            let test_failed = make_failiure_outcome(ape_vec[5]);
            (Self { operation, divisor, test_succeeded, test_failed }, current_items)
        }
        fn perform_modulus_operation(&self, num:&u128) -> u128{
            // Takes a number, performs this apes operation on it, then returns the result
            let div = self.divisor;
            match self.operation{
                Operation::Mod_Add(v) => {
                    if let Some(val) = v{
                        ((num%div) + (val%div))%div
                    }else{
                        ((num%div) + (num%div))%div
                    }
                },
                Operation::Mod_Sub(v) => {
                    if let Some(val) = v{
                        ((num%div) - (val%div))%div
                    }else{
                        ((num%div) - (num%div)) %div
                    }
                },
                Operation::Mod_Mult(v) => {
                    if let Some(val) = v{
                        ((num%div) * (val%div))%div
                    }else{
                        ((num%div) * (num%div))%div
                    }
                },
                Operation::Mod_Div(v) => {
                    if let Some(val) = v{
                        ((num%div) / (val%div))%div
                    }else{
                        ((num%div) / (num%div))%div
                    }
                },
            }
        }
    }

    
    fn make_ape_name(s:&str)->u32{
        s.strip_prefix("Monkey ").unwrap().strip_suffix(":").unwrap().parse::<u32>().unwrap()
    }
    fn make_current_items(s:&str) -> VecDeque<u128>{
        s.strip_prefix("  Starting items: ").unwrap().split(" ")
        .map(|s| {
            let num_str = match  s.contains(","){
                true => s.trim().strip_suffix(",").unwrap(),
                false => s.trim(),
            };
            let num = num_str.parse::<u128>().unwrap();
            num
        }).collect::<VecDeque<u128>>()
    }
    fn make_operation(s:&str) -> Operation{
        let core_operation:[&str;3] = s.strip_prefix("  Operation: new = ").unwrap().split(" ").collect::<Vec<&str>>().try_into().unwrap();
        let [_,right] = match [core_operation[0],core_operation[2]]{
            ["old","old"] => [None,None],
            ["old", v] => [None, Some(v.parse::<u128>().expect("Parsing failed!"))],
            _ => panic!("Shouldn't occur!")
        };
        let operation = match core_operation[1]{
            "*" => Operation::Mod_Mult(right),
            "+" => Operation::Mod_Add(right),
            "-" => Operation::Mod_Sub(right),
            "/" => Operation::Mod_Div(right),
            _ => panic!("Unknown operation in ape-action-creation! {:?}",core_operation)
        };
        operation

    }
    fn make_divisor(s:&str) -> u128{
        let div_num = s.strip_prefix("  Test: ").unwrap().split(" ").filter_map(|s|{
            if s == " " || s == "divisible" || s == "by"{
                None
            }else{
                Some(s.parse::<u128>().unwrap())
            }
        }).fold(0u128, |acc,val| acc+val); // "Fold" one value, as the rest is filtered out.
        div_num
    }
    fn make_success_outcome(s:&str) -> usize{
        let mut temp_str = s.trim();
        temp_str = temp_str.strip_prefix("If true: throw to monkey ").expect("Failed to strip prefix");
        let temp_int = temp_str.parse::<usize>().expect(format!("Failed to parse {temp_str} into u32").as_str());
        temp_int
    }
    fn make_failiure_outcome(s:&str) -> usize{
        let mut temp_str = s.trim();
        temp_str = temp_str.strip_prefix("If false: throw to monkey ").expect("Failed to strip prefix");
        let temp_int = temp_str.parse::<usize>().expect(format!("Failed to parse {temp_str} into u32").as_str());
        temp_int
    }
    fn process_data_string(data_string:&str)->(Vec<Ape>,Vec<u128>, Vec<VecDeque<usize>>){
        let mut preprocessed_apes = {data_string
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
        let mut apes= Vec::<Ape>::with_capacity(preprocessed_apes.len());
        let mut item_queues= Vec::<VecDeque<usize>>::with_capacity(preprocessed_apes.len());
        let mut items= Vec::<u128>::with_capacity(preprocessed_apes.len());
        let mut current_item_number: usize = 0;
        preprocessed_apes
        .iter_mut()
        .for_each(|ape_vec|{
            let (a,mut i) = Ape::new(ape_vec);
            apes.push(a);
            let mut item_idx_queue: VecDeque<usize> = VecDeque::new();
            while let Some(item) = i.pop_front(){
                item_idx_queue.push_back(current_item_number);
                items.push(item); 
                current_item_number += 1;
            }
            item_queues.push(item_idx_queue);

        });
        
        (apes,items, item_queues)

    }
    fn get_next_monkey_idx(monkey:&Ape, item_value:&mut u128, common_denominator:&u128) -> usize{
        // Perform the monkeys operation on the core value
        let new_pre_val = perform_action(&monkey.operation, item_value, common_denominator);
        // Adjust the core value to fit withing our bounds
        *item_value = new_pre_val % common_denominator;
        // Perform the monkey-test to see which monkey recieves the item next
        let monkey_test_result = *item_value % monkey.divisor == 0;
        let next_monkey_idx = match monkey_test_result{
            true => monkey.test_succeeded,
            false => monkey.test_failed
        };
        return next_monkey_idx;
    }
    fn perform_action(monkey_operation:&Operation, item_value:&u128, common_denominator:&u128)->u128{
        match monkey_operation{
            Operation::Mod_Add(v) => {
                match v{
                    Some(x) => item_value + (x%common_denominator),
                    None => item_value + item_value
                }
            },
            Operation::Mod_Sub(v) => {
                match v{
                    Some(x) => {
                        let y = x%common_denominator;
                        if y > *item_value{
                            ((common_denominator - y) + *item_value) % common_denominator 
                        }else{
                            item_value - (x%common_denominator)
                        }
                    },
                    None => 0
                }
            },
            Operation::Mod_Div(v) => {
                match v{
                    Some(x) => item_value / (x%common_denominator),
                    None => 1
                }
            },
            Operation::Mod_Mult(v) => {
                match v{
                    Some(x) => item_value * (x%common_denominator),
                    None => item_value * item_value
                }
            }
        }
    }
    fn get_lowest_common_denominator(numbers:&Vec<u128>) -> u128{
        let initial = lcm(numbers[0],numbers[1]);
        numbers.iter().skip(2).fold(initial, |acc,num|{
            lcm(acc,*num)
        })
    }
    pub fn main_2(file_name:&str, n_rounds:u32) -> Option<u32>{
        let data_string = read_to_string(file_name).unwrap();
        println!("String read!");
        let (mut monkeys, mut items, mut item_queues) = process_data_string(&data_string);
        println!("Monkeys fetched!");
        // Get common denominator
        let common_denominator = {
            let div_vec = monkeys.iter()
                .map(|ape|{
                    ape.divisor
                }).
                collect::<Vec<u128>>();
            get_lowest_common_denominator(&div_vec)
        };
        // Make each item no higher than the highest common denominator.
        items.iter_mut().for_each(|item|{
            *item = *item%common_denominator;
        });
        println!("Common Denominator: {common_denominator}");
        let mut counters = vec![0;monkeys.len()];
        println!("Counters initialized!");
        for n in 0..n_rounds{        
            for monkey_idx in 0..(monkeys.len() as usize){
                // Grab the current monkey:
                let monkey = &monkeys[monkey_idx];
                // For each item it holds:
                while let Some(current_item_idx) = &item_queues[monkey_idx].pop_front(){
                    // Increase its interaction-count
                    counters[monkey_idx] += 1;
                    // get a mutable copy of the item, so that the function can change it
                    let mut item_value = items[*current_item_idx];
                    if n == 4 && file_name == r"src\test_input.txt"{
                        print!("Monkey {monkey_idx}: {} -> ", item_value)
                    }
                    let next_monkey_idx = get_next_monkey_idx(monkey, &mut item_value, &common_denominator);
                    if n == 4 && file_name == r"src\test_input.txt"{
                        print!("Monkey {next_monkey_idx}: {} -> ",item_value);
                        println!();

                    }
                    // Move the items index to that monkeys queue.
                    item_queues[next_monkey_idx as usize].push_back(*current_item_idx);

                }   
                if n == 4 && file_name == r"src\test_input.txt"{
                    println!();
                }
                if file_name == r"src\dummy_input.txt"{
                    match n{
                        0 => match monkey_idx{
                            0 => assert_eq!(counters[monkey_idx],2),
                            1 => assert_eq!(counters[monkey_idx],4),
                            2 => assert_eq!(counters[monkey_idx],3),
                            3 => assert_eq!(counters[monkey_idx],6),
                            _ => ()
                        },
                        20 => match monkey_idx{
                            0 => assert_eq!(counters[monkey_idx],99),
                            1 => assert_eq!(counters[monkey_idx],97),
                            2 => assert_eq!(counters[monkey_idx],8),
                            3 => assert_eq!(counters[monkey_idx],103),
                            _ => ()
                        },
                        1000 => match monkey_idx{
                            0 => assert_eq!(counters[monkey_idx],5204),
                            1 => assert_eq!(counters[monkey_idx],4792),
                            2 => assert_eq!(counters[monkey_idx],199),
                            3 => assert_eq!(counters[monkey_idx],5192),
                            _ => ()
                        },
                        _ => ()
                    }
                }
            }
            // break; // -------------------- TODO: REMOVE -----------------------------
        }
        let best_score = counters
            .iter()
            .fold([0u32;2], |mut acc, &val|{
                if val >= acc[0]{
                    acc[1] = acc[0].clone();
                    acc[0] = val;
                }else if val > acc[1]{
                    acc[1] = val;
                }
                acc
            }).iter().fold(1, |acc, val|{
                println!(" -- {val}");
                acc * val
            });

        Some(best_score)
    }
#[cfg(test)]
mod tests{
    use crate::p1::main_1;
    use super::*;
	#[test]
	fn next_monkey_test(){
        let file_name = r"src\test_input.txt";
        for n_rounds in 1..20{
            let m1_score = main_1(file_name, 1, n_rounds).unwrap();
            let m2_score = main_2(file_name,n_rounds as u32).unwrap() as u128;
            assert_eq!(m1_score,m2_score,"\n--- n_rounds = {n_rounds} ---\n");
        }

	}
    #[test]
    fn opr_add_test(){
        let dividers = vec![9,15,23,2];
        let act_num = 73;
        let start_num = 200;   
        let common_denominator = get_lowest_common_denominator(&dividers);
        let mut adj_val = start_num % dividers[0];
        let mut true_val = start_num;
        let opr:Operation = Operation::Mod_Add(Some(act_num));
        for i in 1..1001{
        adj_val = perform_action(&opr,&adj_val,&common_denominator);
        true_val += act_num;
        assert_eq!(adj_val % dividers[0], true_val % dividers[0],"Adding function is wrong!")
        }

    }
    #[test]
    fn opr_mult_test(){
        let dividers = vec![9,15,23,2];
        let act_num = 3;
        let start_num = 1;   
        let common_denominator = get_lowest_common_denominator(&dividers);
        let mut adj_val = start_num % dividers[0];
        let mut true_val = start_num;
        let opr:Operation = Operation::Mod_Mult(Some(act_num));
        for i in 1..25{
            adj_val = perform_action(&opr,&adj_val,&common_denominator);
            true_val *= act_num;
            assert_eq!(adj_val % dividers[0], true_val % dividers[0],"Adding function is wrong!")
        }
    }

    #[test]
    fn opr_sub_test(){
        let dividers = vec![9,15,23,2];
        let div = dividers[0];
        let act_num = 78;
        let start_num = 100_000;        
        let common_denominator = get_lowest_common_denominator(&dividers);
        let mut adj_val = start_num % dividers[0];
        let mut true_val = start_num;
        let opr:Operation = Operation::Mod_Sub(Some(act_num));
        println!("Common Denominator: {common_denominator}");
        while true_val > 80{
            adj_val = perform_action(&opr,&adj_val,&common_denominator);
            true_val -= act_num;
            println!("{adj_val} % {div} = {}, {true_val} % {div} = {}",adj_val % div, true_val % div);
            assert_eq!(adj_val % div, true_val % div,"Adding function is wrong!")
        }
    }
    fn opr_div_test(){ // Test is disabled. A division does not occur in the dataset, and it was a lot of work trying to make modulus division work out.
        let dividers = vec![9,15,23,2];
        let div = dividers[0];
        let act_num = 3;
        let start_num = 9781342978132;        
        let common_denominator = get_lowest_common_denominator(&dividers);
        let mut adj_val = start_num % dividers[0];
        let mut true_val = start_num;
        let opr:Operation = Operation::Mod_Div(Some(act_num));
        println!("Common Denominator: {common_denominator}");
        while true_val > 500{
            adj_val = perform_action(&opr,&adj_val,&common_denominator);
            true_val /= act_num;
            println!("{adj_val} % {div} = {}, {true_val} % {div} = {}",adj_val % div, true_val % div);
            assert_eq!(adj_val % div, true_val % div,"Adding function is wrong!")
        }
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
    let count = main_2(file_name,10_000);
    let end = start.elapsed();
    println!("{count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.expect("No value returned to main"),2713310158u32,"\n--- Answer should be 2713310158! ---\n");
    }
}

