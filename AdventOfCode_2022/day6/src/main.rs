
mod p1{
    use std::fs::read_to_string;
    pub fn overlapping_chars(chars:&Vec<char>)->bool{
        chars.iter()
            .enumerate()
            .map(|(idx,c)| {
                chars.iter()
                    .skip(idx+1)
                    .collect::<Vec<&char>>()
                    .contains(&c)
            }).collect::<Vec<bool>>()
            .contains(&true)
    }

    pub fn main1(file_name:&str){
        const N: usize = 4usize;
        let text = read_to_string(file_name).expect("Failed to read file!\n").chars().collect::<Vec<char>>();
        let mut chars:Vec<char> = Vec::new();
        for i in 0..N{
            chars.push(text[i]);
        }
        if !overlapping_chars(&chars){
            println!("{}",N);
            return;
        }
        for i in N..text.len(){
            chars.remove(0);
            chars.push(text[i]);
            if !overlapping_chars(&chars){
                println!("{}",i+1);
                println!("{:?}",chars);
                return;
            }
        }
    }   

}
mod p2{
    use std::fs::read_to_string;
    use crate::p1::overlapping_chars;
    pub fn main2(file_name:&str){
        const N: usize = 14usize;
        let text = read_to_string(file_name).expect("Failed to read file!\n").chars().collect::<Vec<char>>();
        let mut chars:Vec<char> = Vec::new();
        for i in 0..N{
            chars.push(text[i]);
        }
        if !overlapping_chars(&chars){
            println!("{}",N);
            return;
        }
        for i in N..text.len(){
            chars.remove(0);
            chars.push(text[i]);
            if !overlapping_chars(&chars){
                println!("{}",i+1);
                println!("{:?}",chars);
                return;
            }
        }
    }  
}
use crate::p1::main1;
use crate::p2::main2;
fn main() {
    let file_name = r"src/dummy1.txt";
    let file_name = r"src/dummy2.txt";
    let file_name = r"src/puzzle.txt";
    main1(file_name);
    main2(file_name);
}
