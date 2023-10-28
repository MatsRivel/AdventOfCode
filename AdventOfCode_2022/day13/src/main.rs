mod p1{
    use core::panic;
    use std::{fs::read_to_string, cmp::max};
    use itertools::Itertools;

    #[derive(Clone,Copy, PartialEq, Debug)]
    pub enum CO{ // Content Options
        C(char),
        I(u8),
    }
    pub struct Peeky{
        id:bool,
        depth:u8,
        internal_iter: Box<dyn Iterator<Item = CO>>,
        last: Option<CO>,
        current: Option<CO>,
        next: Option<CO>,
    }
    impl Peeky{
        fn new(line:&str, id:bool) -> Self{
            let mut the_chars = line.chars();
            let mut iter_to_be:Vec<CO> = Vec::with_capacity(line.len());
            let mut current:Option<char> = None;
            let mut next:Option<char> = match the_chars.next(){
                Some(v) => Some(v),
                None => todo!("Make the internal iter empty"),
            };
            loop{
                current = next;
                next = the_chars.next();
                match (current, next){
                    (None, _) => break,
                    (Some('['), _) | (Some(']'),_) |(Some(','),_) => {
                        let c = current.expect("Infallable unwrap");
                        let coc = CO::C(c);
                        iter_to_be.push(coc);
                    }
                    (Some(v), Some('['))|(Some(v),Some(']'))|(Some(v),Some(',')) => {
                        let c = current.expect("Infallable unwrap");
                        let i = c.to_digit(10).expect("Infallable unwrap") as u8;
                        let coi = CO::I(i);
                        iter_to_be.push(coi)
                        
                    },
                    (Some(v), Some(w)) => {
                        let iv = v.to_digit(10).expect("Infallable unwrap") as u8;
                        let iw = w.to_digit(10).expect("Infallable unwrap") as u8;
                        let i = iv*10 + iw;
                        let coi = CO::I(i);
                        iter_to_be.push(coi)
                    }
                    (x,y) => panic!("Wtf is x: {x:?}, y: {y:?} ")
                }
            }
            let last = None;
            let iter = iter_to_be.into_iter();
            let mut internal_iter = Box::new(iter);
            let current = internal_iter.next();
            let next = internal_iter.next();
            Self { id, depth:0u8, internal_iter, last, current, next }
        }

        fn next(&mut self){
            self.last = self.current;
            self.current = self.next;
            self.next = self.internal_iter.next();
            match self.last{
                Some(CO::C('[')) => self.depth += 1,
                Some(CO::C(']')) => self.depth -= 1,
                _ => ()
            }
            // let fake_name = match self.id{
            //     true => "c1",
            //     false => "                        c2"
            // };
            // let chr = match self.current{
            //     Some(CO::C(v)) => format!("{}",v),
            //     Some(CO::I(v)) => v.to_string(),
            //     _ => "_".to_string(),
            // };
            // println!("{fake_name}: {:?} | {}",chr, self.depth);
        }
    }
    pub fn mega_match(c1:&mut Peeky,c2:&mut Peeky) -> Option<bool>{
        'inner_loop: loop{
            // while c1.current.is_some() && c1.current==c2.current{
            //     c1.next();
            //     c2.next();
            // }
            if Some(CO::C(',')) == c1.current{
                c1.next();
            }
            if Some(CO::C(',')) == c2.current{
                c2.next();
            }
            'equal_char: loop{
                // println!("<eq>");
                if c1.current != c2.current{
                    break 'equal_char;
                }
                match (c1.current, c2.current) {
                    (Some(CO::C(_)),Some(CO::C(_))) => {
                        c1.next();
                        c2.next();
                    }
                    (_,_) => break 'equal_char
                }
            }
            // println!("</eq>");
            
            match (c1.current, c2.current){
                (Some(CO::I(u)),Some(CO::I(v))) => {
                    // println!("Comparing {u} and {v} | Depths: {} and {}", c1.depth, c2.depth);
                    if u < v{
                        // println!("Ordered: c1 ({u}) < c2 ({v})!");
                        return Some(true);
                    }else if u > v {
                        // println!("Not ordered: c1 ({u}) > c2 ({v})!");
                        return Some(false);
                    }
                    // u == v
                    if c1.depth < c2.depth && c2.next != Some(CO::C(']')) {
                        // println!("Same I, but c1 is shallower");
                        return Some(true); 
                    }else if c1.depth > c2.depth && c1.next != Some(CO::C(']')){
                        // println!("Same I, but c1 is deeper");
                        return Some(false);
                    } // Same depth.
                    if c1.last == Some(CO::C('[')) && c1.next == Some(CO::C(']')) && c2.next != Some(CO::C(']')) {
                        // println!("x|c1 ({:?}) is a single digit list, c2 ({:?})is not.", c1.current,c2.current);
                        return Some(true);
                    }
                    if c2.last == Some(CO::C('[')) && c2.next == Some(CO::C(']'))&& c1.next != Some(CO::C(']')){
                        // println!("x|c2 ({:?}) is a single digit list, c1 ({:?})is not.", c2.current,c1.current);
                        return Some(false);
                    }
                    // They are identical. Proceed to next step:
                    c1.next();
                    c2.next();
                },

                (Some(CO::C('[')), Some(CO::C(']'))) => {
                    // println!("A| c1.depth: {:?}, c1.last: {:?}, c1.current: {:?}, c1.next{:?}", c1.depth, c1.last, c1.current, c1.next);
                    // println!("A| c2.depth: {:?}, c2.last: {:?}, c2.current: {:?}, c2.next{:?}\n", c2.depth, c2.last, c2.current, c2.next);
                    if c1.next == Some(CO::C(']')) && c2.last == Some(CO::C('[')){
                        if c1.depth < c2.depth -1{
                            // println!("A|c1 has empty list at lower depth");
                            return Some(true);
                        } else if c1.depth > c2.depth -1{
                            // println!("A|c2 has empty list at lower depth");
                            return Some(false);
                        }// Else: Same depth, both continue.
                    }else if c1.next == Some(CO::C(']')){
                        // println!("A|c1 has empty list, c2 does not.");
                        return Some(true);
                    }else if c2.last == Some(CO::C('[')){
                        // println!("A|c2 has empty list, c1 does not.");
                        return Some(false);
                    }
                    c1.next();
                    c2.next();
                },
                (Some(CO::C(']')), Some(CO::C('['))) => {
                    // println!("B| c1.depth: {:?}, c1.last: {:?}, c1.current: {:?}, c1.next{:?}", c1.depth, c1.last, c1.current, c1.next);
                    // println!("B| c2.depth: {:?}, c2.last: {:?}, c2.current: {:?}, c2.next{:?}\n", c2.depth, c2.last, c2.current, c2.next);
                    if c1.last == Some(CO::C('[')) && c2.next == Some(CO::C(']')){
                        if c1.depth-1 < c2.depth{
                            // println!("B|c1 has empty list at lower depth");
                            return Some(true);
                        } else if c1.depth-1 < c2.depth{
                            // println!("B|c2 has empty list at lower depth");
                            return Some(false);
                        }// Else: Same depth, both continue.
                    } else if c1.last == Some(CO::C('[')){
                        // println!("B|c1 has empty list, c2 does not.");
                        return Some(true);
                    }else if c2.next == Some(CO::C(']')){
                        // println!("B|c2 has empty list, c1 does not.");
                        return Some(false);
                    }
                    c1.next();
                    c2.next();
                },
                (Some(_), Some(CO::C('['))) => {
                    // println!("a");
                    c2.next();
                    continue 'inner_loop;
                },
                (Some(CO::C('[')), Some(_)) => {
                    // println!("b");
                    c1.next();
                    if let Some(CO::C(']')) = c1.current{
                        // println!("c1 has an empty list segment; Shorter (?)");
                        //TODO: If c2 also has empty list segment, the lowest depth wins.
                        return Some(true);
                    } 
                    continue 'inner_loop
                },
                (None, Some(_)) | (Some(CO::C(']')), _) => {
                    // println!("Is in order; c1 shorter than c2 | {:?},{:?} | \n{line1:?}#\n {line2:?}#\n", c1.current, c2.current);
                    return Some(true);
                },
                (Some(_), None | Some(CO::C(']')))  =>{
                    // println!("Not in order; c1 longer than c2 | {:?},{:?} |\n{line1:?}#\n{line2:?}#\n",c1.current,c2.current);
                    return Some(false);
                },
                (_,_) => return Some(true) //Seems that if identical lines: ends up here
            }
        }
    }
    
    pub fn is_line_pair_in_order(line1:&str, line2:&str) -> bool{
        // println!("{:?}",line1);
        // println!("{:?}",line2);
        // println!();
        let mut c1 = Peeky::new(line1, true);
        let mut c2 = Peeky::new(line2, false);
        'outer_loop: loop {
            match (c1.current, c2.current) {
                (None,_) => break 'outer_loop,
                (_,None) => return false,
                (_,_) => ()
            }
            let match_result = mega_match(&mut c1,&mut c2);
            match match_result{
                Some(v) => return v,
                None => ()
            }

        }
    true // No errors found
    }

    pub fn main_1(file_name:&str){
        let line_string = read_to_string(file_name).unwrap();
        let mut lines = line_string.lines();
        let mut correct_summer = 0;
        let mut nth_pair = 0;
        while let (Some(line1), Some(line2)) = (lines.next(),lines.next()){
            // println!("________________________________________________________________");
            nth_pair += 1;
            let expected_answer = match lines.next(){
                Some("true") => Some(true),
                Some("false") => Some(false),
                _ => None
            };
            let ordering = is_line_pair_in_order(line1,line2);
            if let Some(ans) = expected_answer{
                if ans != ordering{
                    println!("Expect: {ans}, Ordering: {ordering}");
                    println!("{line1:?}\n{line2:?}\n");
                }
                assert_eq!(ordering, ans);
            }
            if ordering{
                correct_summer += nth_pair;
            }
            // Process each line and compare lines.
            // Each line should likely be processed recursively, starting with the inner list and building it from there.
            // Compare from left to right.
            // Rules:
            // 1. If both values are integers, the lowest integer should be the one in the first line.
            // 2. If both values are lists, compare the first value of each.
            //     2.1. The first time a value in one is lower than the other, the one should be first.
            //     2.2. If both lists hold the same values, the shorter list should come first.
            // 3. If one value is an integer and the other is a list, compare the one into a list with only one element.
            //      3.1 Compare the first element of each: Lowest should be first.
            // Goal: Store the indices of all pairs that are in order, then return the sum of those indices.
        }
        println!("Part 1: {correct_summer} (The correct answer is 5506)");
    }
}
mod p2{
    use std::fs::read_to_string;
    use crate::p1::{self, is_line_pair_in_order};
    pub fn main_2(file_name:&str){
        let line_string = read_to_string(file_name).unwrap();
        let mut lines = line_string
            .lines()
            .filter(|line|{
                line.starts_with("[")
            })
            .map(|line| {
                line
            })
            .collect::<Vec<&str>>();
        lines.push("[[2]]");
        lines.push("[[6]]");
        let mut in_order = false;

        while !in_order{
            in_order = true;
            for i in 0..(lines.len()-1){
                let line1 = lines[i];
                let line2 = lines[i+1];
                if !is_line_pair_in_order(line1, line2){
                    lines[i] = line2;
                    lines[i+1] = line1;
                    in_order = false;
                }
            }
        }
        // for line in lines.iter(){
        //     println!("{line}");
        // }
        println!("Now we find the brake-lines:");
        let mut start_idx: Option<usize> = None;
        let mut end_idx: Option<usize> = None;
        for (idx,line) in lines.iter().enumerate(){
            if line.starts_with("[[2]]"){
                start_idx = Some(idx+1);
                println!("Start at line {idx}");
                println!("{:?}",line);
            }else if line.starts_with("[[6]]"){
                end_idx = Some(idx+1);
                println!("End at line {idx}");
                println!("{:?}",line);

            }
        }
        println!("Part 2: {}", start_idx.unwrap()*end_idx.unwrap());
        
    }
}
use p1::main_1;
use p2::main_2;
fn main() {
    let file_name_list = vec![r"src\homemade_input.txt", r"src\homemade_input2.txt", r"src\dummy_input.txt",r"src\puzzle_input.txt"];
    for file_name in file_name_list.iter(){
        // main_1(file_name);
    }   
    let file_name = r"src\puzzle_input.txt";
    // main_1(file_name);
    // let file_name = r"src\dummy_input.txt";
    main_2(file_name);
}



