use std::{fs::read_to_string, collections::HashMap};

type IsActive = bool;

#[derive(Clone,Copy,Debug, PartialEq)]
enum Symbol{
    Value(i32),
    Plus(IsActive),
    Minus(IsActive),
    Multiply(IsActive),
    Divide(IsActive),
    Modulo(IsActive),
    Other(IsActive)
}
impl Symbol{
    fn new(s:&str)->Self{
        if s.len() > 1{
            Symbol::Value(s.parse::<i32>().unwrap())
        }else{
            match s{
                "+" => Self::Plus(true),
                "-" => Self::Minus(true),
                "*" => Self::Multiply(true),
                "/" => Self::Divide(true),
                "#" => Self::Modulo(true),
                _   => Self::Other(true)
            }
        }
    }
}
fn extract_adjacent_symbols(s:&str) -> Vec<&str>{
    if s.len() == 0{
        return vec![];
    }else if s.len() == 1{
        return vec![s];
    }
    print!("{s}");
    let mut output = vec![];
    let chars = s.chars().collect::<Vec<char>>();
    // let mut non_num_count = chars.clone().iter().filter(|c| !c.is_numeric() ).fold(0,|acc,_| acc+1 );
    let mut left =0;
    'main_loop: while left < chars.len(){
        if !chars[left].is_numeric(){
            while left < chars.len() && !chars[left].is_numeric(){
                output.push(&s[left..left+1]);
                println!("\t{}",&s[left..left+1]);
                left+=1;
            }
            if left == chars.len(){
                break 'main_loop;
            }
        }
        let mut right = left+1;
        while right < chars.len() && chars[right].is_numeric(){
            right+=1;
        }
        if right == chars.len(){
            println!("\t{}",&s[left..right]);
            output.push(&s[left..right]);
        }else{
            println!("\t{}",&s[left..right]);
            output.push(&s[left..right]);
        }
        left = right;
    }
    output
}
fn process_data(data_string:&str)->(HashMap<[usize;2],Symbol>, Vec<[usize;2]>){
    let mut insertion_order = vec![];
    let mut output = HashMap::<[usize;2],Symbol>::new();
    for (row_num, row) in data_string.lines().enumerate(){
        let mut col_num = 0;
        for col in row.split("."){
            let col_elements = extract_adjacent_symbols(col);
            for &element in col_elements.iter(){
                let el_len = element.len();
                let symbol = Symbol::new(element);
                // println!("{element}, {el_len}, {symbol:?}");
                for idx in col_num..col_num+el_len{
                    // println!("\t[{row_num}, {idx}], {symbol:?}");
                    output.insert([row_num, idx], symbol);
                    insertion_order.push([row_num,idx])
                }
                col_num += el_len;
            }
            // if col_elements.len() > 0{
            //     println!();
            // }
            col_num += 1;
        }
    } 
    (output, insertion_order)
}
fn get_2d_neighbours(coord:&[usize;2])->Vec<[usize;2]>{
    let mut output = Vec::with_capacity(8);
    for i in -1..=1{
        for j in -1..=1{
            if i == 0 && j == 0 {
                continue;
            }
            let new_x = coord[0] as i64 + i;
            let new_y = coord[1] as i64 + j;
            if new_x < 0 || new_y < 0{
                continue;
            }
            output.push([new_x as usize, new_y as usize]);
        }
    }
    output
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (matrix, insertion_order) = process_data(&data_string);
    let mut output = 0;
    let mut last_symbol = Symbol::Other(false);// Arbitary temp symbol;
    let mut last_idx = [usize::MAX;2]; // Temp idx
    let mut neighbour_found = false;
    for key in insertion_order.iter(){
        let val = matrix.get(key).unwrap();
        // If we found an adjacent symbol for this value, dont look any more. One symbol per value. Add each value one or zero times.
        if &last_symbol == val && last_idx == [key[0],key[1]-1]{
            last_idx = *key;
            if neighbour_found{
                continue;
            }
        }
        #[cfg(test)]{rr
            if last_idx[0] < key[0]{
                println!();
            }
            if last_symbol != *val{
                let to_print = match val{
                    Symbol::Value(v) => format!("{v}"),
                    Symbol::Plus(_) => "+".to_string(),
                    Symbol::Minus(_) => "-".to_string(),
                    Symbol::Multiply(_) => "*".to_string(),
                    Symbol::Divide(_) => "/".to_string(),
                    Symbol::Modulo(_) => "#".to_string(),
                    Symbol::Other(_) => "?".to_string(),
                };
                print!("{to_print} ");
            }
        }
        last_symbol = *val;
        last_idx = *key;
        neighbour_found = false;

        // To prevent iterating over the same value n times.
        if let Symbol::Value(current_val) = val{
            // if ANY symbol is adjacent to the current value, add it.
            let neighbours = get_2d_neighbours(key);
            'neighbour_loop: for nei in neighbours.iter(){
                if let Some( symbol) = matrix.get(nei){
                    match symbol{
                        Symbol::Value(_)    => (),
                        _    => {
                            output += current_val;
                            neighbour_found = true;
                            break 'neighbour_loop; // We only need to verify we have ONE neighbour, else we get too many additions.
                        },
                    }
                }
            }
        }
    }
    Some(output)

}

#[cfg(test)]
    mod tests{
    use std::time::Instant;

    use super::*;
	#[test]
	fn zero_puzzle(){
        let file_name = r"src\dummy_zero.txt";
        let start = Instant::now();
        let count = main_1(file_name);
        let end = start.elapsed();
        println!("\nPart 1 Dummy: {count:?}\nRuntime: {end:?}");
        if let Some(actual_value) = count{
            let expected_value = 0;
            assert_eq!(actual_value, expected_value, "Got {actual_value}, expected {expected_value}\n__________________________");
        }

    }
    #[test]
    fn correct_neighbours_test(){
        let base = [1,1];
        let expected_neighbours = vec![[0,0], [0,1], [1,0], [1,2],[2,1],[2,2], [0,2],[2,0]];
        let found_neighbours = get_2d_neighbours(&base);
        for n in found_neighbours.clone().iter(){
            assert!(expected_neighbours.contains(&n));
        }
        for n in expected_neighbours{
            assert!(found_neighbours.contains(&n));
        }
    }

}
