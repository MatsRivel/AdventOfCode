mod part1{
    use std::{fs::read_to_string, error::Error};
    #[derive(Debug,Clone)]
    pub struct Crate{
        pub name: String,
    }
    #[derive(Debug,Clone)]
    pub struct Column{
        pub stack: Vec<Crate>
    }
    impl Column{
        fn new() -> Self{
            Self{stack: Vec::new()}
        }
        fn move_n_to_other(&mut self, other: &mut Self, n:u8){
            for _ in 0..n{
                let a_box = self.stack.pop();
                match a_box{
                    None => panic!("No box to move!"),
                    Some(v) => other.stack.push(v)
                }
                
            }
        }
    }
    #[derive(Debug)]
    pub struct Command{
        pub n:u8,
        pub from:usize,
        pub to:usize
    }
    impl Command{
        fn new(line:&str) -> Self{
            let sections:[&str;6] = line.split(" ")
            .collect::<Vec<&str>>()
            .try_into()
            .expect("Failed to parse command");
            match sections{
                ["move",n_str,"from",from_str,"to",to_str] => {
                    let (n,from,to) = (n_str.parse().unwrap(),from_str.parse().unwrap(), to_str.parse().unwrap());
                    Self{n,from,to}
                },
                _ => Self{n:0,from:0,to:0},
            }

        }
    }
    #[derive(Debug)]
    pub struct Row{
        pub row: Vec<Option<Crate>>
    }

    impl Row{
        fn new(line:&str) -> Self{
            let row:Vec<Option<Crate>>= Vec::new();
            let mut n = 0;
            let mut row:Vec<Option<Crate>> = Vec::with_capacity(3);
            while n+3 < line.len(){
                if let Ok(element) = process_subline(&line, n){
                    row.push(element);
                };
                n+=4;
            }
            if n > 0{
                if let Ok(last_element) = process_subline(&line, n){
                    row.push(last_element);
                };
            }
            Self{row}
        }
    }

    fn process_subline(line:&str,n:usize) -> Result<Option<Crate>,&str> {
        let sub_line: [&str;3] = [&line[n..=n],&line[(n+1)..=(n+1)], &line[(n+2)..=(n+2)]];
        let element = match sub_line{
            ["[",c,"]"] => Ok(Some(Crate{name:c.to_string()})),
            [" ", " ", " "] => Ok(None),
            [" ",num," "] => {
                let num_is_numeric = num.parse::<i32>();
                match num_is_numeric{
                    Ok(_) => Err("Do not use numeric!"),
                    Err(_) => Ok(None)
                }
            },
            _ => Ok(None)
        };
        element
    }

    pub fn read_input(file_name:&str) -> Result<(Vec<Row>, Vec<Command>),std::io::Error>{
        let mut rows:Vec<Row> = Vec::new();
        let mut commands:Vec<Command> = Vec::new();
        read_to_string(file_name)?
            .lines()
            .for_each(|line| {
                if line.starts_with("move"){
                    commands.push(Command::new(line));
                }else{
                    let row = Row::new(line);
                    if !row.row.is_empty(){
                        rows.push(row)
                    }
                    
                }
            });
        Ok((rows, commands))
        
    }
    
    pub fn rotate_rows(rows:&mut Vec<Row>) -> Vec<Column>{
        let mut columns:Vec<Column> = Vec::new();
        for _ in 0..rows.len(){
            columns.push(Column::new())
        }
        for i in 0..rows.len(){
            for j in (0..(rows[i].row.len())).rev(){
                if let Some(inner_crate) = &rows[i].row[j]{
                    let new_crate = Crate{name:inner_crate.name.to_string()};
                    while columns.len() <= j{
                        columns.push(Column::new());
                    }
                    columns[j].stack.push(new_crate);

            }

            }
        }
        for i in 0..columns.len(){ // TODO: Find a beter solution that this stuff...
            columns[i].stack = columns[i].stack.iter().rev().map(|x| Crate{name:x.name.to_string()}).collect();
        }
        columns

    }
    pub fn main_1(file_name:&str){
        let (mut rows, commands) = read_input(file_name).expect("Failed to get rows!");
        let mut columns = rotate_rows(&mut rows);
        // columns.iter().for_each(|print_this| println!("{:?}",print_this.stack));
        // println!("");
        
        for command in commands.iter(){
            for i in 0..command.n as usize{
                //println!("Command{i}: {:?}",command);
                let in_transit = columns[command.from-1].stack.pop().expect("Failed to pop crate!");
                // println!("Picked up |{}| from column {}",in_transit.name,command.from);
                // println!("Put down  |{}| at   column {}",in_transit.name,command.to);
                columns[command.to-1].stack.push(in_transit);
                // columns.iter().enumerate().for_each(|(idx,print_this)| println!("{}: {:?}",idx+1,print_this.stack));
                // println!("");
            }
        }
        for column in columns.iter() {
            if let Some(top_crate) = column.stack.last(){
                print!("{}",top_crate.name);
            }else{
                print!(" ");
            }
        }
        println!("\nDone!")

    }
}
mod part2{
    use std::fs::read_to_string;
    use crate::part1::{read_input, rotate_rows, Crate};
    pub fn main_2(file_name:&str){
        let (mut rows, commands) = read_input(file_name).expect("Failed to get rows!");
        let mut columns = rotate_rows(&mut rows);
        // columns.iter().for_each(|print_this| println!("{:?}",print_this.stack));
        // println!("");
        
        for command in commands.iter(){
            let mut crate_buffer:Vec<Crate> = Vec::new();
            for _ in 0..command.n as usize{
                //println!("Command{i}: {:?}",command);
                let in_transit = columns[command.from-1].stack.pop().expect("Failed to pop crate!");
                crate_buffer.push(in_transit);
                // println!("Picked up |{}| from column {}",in_transit.name,command.from);
                // println!("Put down  |{}| at   column {}",in_transit.name,command.to);
                // columns.iter().enumerate().for_each(|(idx,print_this)| println!("{}: {:?}",idx+1,print_this.stack));
                // println!("");
            }
            for _ in 0..command.n as usize{
                let in_transit = crate_buffer.pop().expect("Failed to pop crate!");
                columns[command.to-1].stack.push(in_transit);

            }
        }
        for column in columns.iter() {
            if let Some(top_crate) = column.stack.last(){
                print!("{}",top_crate.name);
            }else{
                print!(" ");
            }
        }
        println!("\nDone!")
    }
}

use part1::main_1;
use part2::main_2;
fn main() {
    let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
    main_1(file_name);
    main_2(file_name);
}
