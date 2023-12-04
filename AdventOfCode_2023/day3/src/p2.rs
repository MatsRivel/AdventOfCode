use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug,Clone,Copy,PartialEq)]
enum ValRef{
    Ref([usize;2]),
    Val(i32)
}
#[derive(Debug,Clone,Copy,PartialEq)]
enum Symbol{
    Pluss,
    Minus,
    Mult,
    Div,
    Other
}
#[derive(Debug,Clone,Copy,PartialEq)]
enum Component{
    Value(ValRef),
    Symbol(Symbol),
    Space
}
impl Component{
    fn new(s:&str)->Self{
        if let Ok(val) = s.parse::<i32>(){
            Component::Value(ValRef::Val(val))
        }else{
            match s{
                "+" => Self::Symbol(Symbol::Pluss),
                "-" => Self::Symbol(Symbol::Minus),
                "*" => Self::Symbol(Symbol::Mult),
                "/" => Self::Symbol(Symbol::Div),
                _ => Self::Symbol(Symbol::Other),
            }
        }
    }
    fn is_value(&self)->bool{
        if let Component::Value(_) = &self{
            return true;
        }
        false
        
    }
    fn is_symbol(&self)->bool{
        !self.is_value()
    }
    fn len(&self)->usize{
        if let Component::Value(ValRef::Val(v)) = self{
            format!("{v}").len()
        }else if self.is_symbol(){
            1
        }else{
            todo!("Add \".len()\" format to valref::ref");
        }

    }
}
fn extract_symbols_from_section(section:&str)->Vec<Component>{
    // println!("{section}");
    if section.len() == 0 || section =="."{
        return vec![Component::Space];
    }
    let mut output = vec![];
    let mut left = 0;
    let chars = section.chars().collect::<Vec<char>>();
    while left < section.len(){
        if !chars[left].is_numeric(){
            output.push(Component::new(&section[left..left+1]));
            left += 1;
            continue; // Restart the loop in case we encounter another symbol instead of a number.
        }
        let mut right = left;
        while right < section.len() && chars[right].is_numeric(){
            right +=1;
        }
        output.push(Component::new(&section[left..right]));
        left = right;
    }
    output
}

fn split_line(line:&str)->Vec<&str>{
    let mut output = vec![];
    let mut left = 0;
    let chars = line.chars().collect::<Vec<char>>();
    while left < line.len(){
        if !chars[left].is_numeric(){
            output.push(&line[left..left+1]);
            left += 1;
            continue; // Restart the loop in case we encounter another symbol instead of a number.
        }
        let mut right = left;
        while right < line.len() && chars[right].is_numeric(){
            right +=1;
        }
        output.push(&line[left..right]);
        left = right;
    }
    output
}
fn process_data_string(data_string:&str)->(HashMap<[usize;2],ValRef>,HashMap<[usize;2],Symbol>){
    let mut values = HashMap::<[usize;2],ValRef>::new();
    let mut symbols = HashMap::<[usize;2],Symbol>::new();
    for (xidx, line) in data_string.lines().enumerate(){
        let mut yidx = 0;
        for section in split_line(line).into_iter(){
            // ""-sections will automatically be skipped as a consequence of "extract_symbols_from_section" returning "[]".
            if section.len() == 0{
                yidx += 1;
                continue;
            }
            let components = extract_symbols_from_section(section);
            for component in components.iter(){
                let coord = [xidx,yidx];
                match component{
                    Component::Space => (),
                    Component::Symbol(symbol) => {symbols.insert(coord, *symbol);},
                    Component::Value(ValRef::Val(v)) => {
                        // Insert the value
                        values.insert(coord, ValRef::Val(*v));
                        // If value has more than 1 digit, we insert a "reference" to that value instead.
                        for length in 1..component.len(){
                            let new_coord = [coord[0], coord[1] + length]; 
                            values.insert(new_coord, ValRef::Ref(coord));
                        }
                        yidx += component.len()-1;
                    },
                    Component::Value(ValRef::Ref(_)) => panic!("Not possible to happen at this point in the process."),
                }
                yidx += 1;
            }
        }
    }

    (values,symbols)
}
fn print_grid(values: &HashMap<[usize;2],ValRef>, symbols: &HashMap<[usize;2],Symbol>){
    let max_x_val = values.iter().fold(0, |acc,([x,_],_)| if *x > acc{*x}else{acc});
    let max_x_sym= symbols.iter().fold(0, |acc,([x,_],_)| if *x > acc{*x}else{acc});
    let max_y_val = values.iter().fold(0, |acc,([_,y],_)| if *y > acc{*y}else{acc});
    let max_y_sym= symbols.iter().fold(0, |acc,([_,y],_)| if *y > acc{*y}else{acc});
    let max_x = std::cmp::max(max_x_val,max_x_sym);
    let max_y = std::cmp::max(max_y_val,max_y_sym);

    for x in 0..=15{
        for y in 0..=15{
            let coord = [x,y];
            if !values.contains_key(&coord) && !symbols.contains_key(&coord){
                print!(".");
            }else if let Some(ValRef::Val(value)) = values.get(&coord){
                print!("{value}");
            }else if let Some(ValRef::Ref(_)) = values.get(&coord){
                ()
            }else if let Some(symbol) = symbols.get(&coord){
                match symbol{
                    Symbol::Pluss   => print!("+"),
                    Symbol::Minus   => print!("-"),
                    Symbol::Mult    => print!("*"),
                    Symbol::Div     => print!("/"),
                    Symbol::Other   => print!("#"),
                }
            }
        }
        println!();
    }
    println!();
}

fn get_2d_neighbours(coord: &[usize;2]) -> Vec<[usize;2]>{
    let mut output = vec![];
    for i in -1..=1{
        for j in -1..=1{
            let a = coord[0] as i64;
            let b = coord[1] as i64;
            if (i != 0 || j != 0) && ( a+i >= 0 && b+j >= 0 )  {
                let x = (a + i ) as usize;
                let y = (b + j ) as usize;
                output.push([x,y]);
            }
        }
    }
    output
}
pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let (mut values, symbols) = process_data_string(&data_string);
    let mut keys = symbols.keys().map(|key| key.clone()).collect::<Vec<[usize;2]>>();
    keys.sort();
    let mut total = 0;
    for symbol_key in keys.into_iter(){
        let mut found = vec![];
        for neighbour in get_2d_neighbours(&symbol_key){
            // Check if any symbol is adjacent to the neighbour.
            let mut new_key = neighbour;
            // Find the correct value if the key does not point to it directly.
            while let Some(ValRef::Ref(nested_key)) = values.get(&new_key){
                new_key = *nested_key;
            }
            if let Some(ValRef::Val(v)) = values.get_mut(&new_key){
                if *v != 0{
                    found.push(*v);
                    *v = 0; // Once we've used the value once, we "kill" it so we don't end up adding it again.
                }
            }
        }
        if found.len() == 2{
            total += found[0] *found[1];
        }
    }
    Some(total)    
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn extraction_test(){
        let output = extract_symbols_from_section("*123/");
        let expected = vec![Component::Symbol(Symbol::Mult), Component::Value(ValRef::Val(123)), Component::Symbol(Symbol::Div)];
        output.iter().zip(expected.iter()).for_each(|(found,correct)| assert_eq!(found,correct));
    }

}
