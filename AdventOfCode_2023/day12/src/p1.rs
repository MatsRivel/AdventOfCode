use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
#[derive(PartialEq,Eq)]
enum Spring{
    Working,
    Broken,
    Unknown
}

pub fn read_lines<P>(file_name:P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
fn process_line(line:String) -> (Vec<Spring>,Vec<usize>){
    let [springs_str,order_str]:[&str;2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
    let springs = springs_str
        .chars()
        .filter_map(|c| {
            match c{
                '?' => Some(Spring::Unknown),
                '.' => Some(Spring::Working),
                '#' => Some(Spring::Broken),
                _ => None
            }
        }).collect::<Vec<Spring>>();
    let order = order_str.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    (springs,order)
}  

fn delve_for_orders(mut springs: &[Spring], order:&[usize])->u32{
    todo!()
}

pub fn main_1(file_name:&str)->Option<i32>{
    let mut lines = read_lines(file_name).unwrap();
    let mut total = 0;
    while let Some(Ok(line)) = lines.next(){
        let (springs,order) = process_line(line);

    }
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
