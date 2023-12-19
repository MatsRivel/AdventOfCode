use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fmt::Display;
#[derive(PartialEq,Eq,Clone,Copy,Debug)]
enum Spring{
    Working,
    Broken,
    Unknown
}
impl Display for Spring{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Spring::Working => write!(f,"."),
            Spring::Broken => write!(f,"#"),
            Spring::Unknown => write!(f,"?"),
        }
    }
}

pub fn read_lines<P>(file_name:P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
fn process_line(line:String) -> (Vec<(Spring)>,Vec<usize>){
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

fn spring_config_is_valid(springs: &[Spring], order:&[usize])->bool{
    let mut sections = vec![];
    let mut counter = 0;
    for spring in springs{
        if counter != 0 && *spring != Spring::Broken{
            sections.push(counter);
            counter = 0;
        }else if *spring == Spring::Broken{
            counter +=1;
        }
    }
    if counter != 0{
        sections.push(counter);
    }

    if sections.len() != order.len(){
        return false;
    }
    let output = sections.iter().zip(order.iter()).fold(true, |acc,(section_length,order_length)| acc && section_length == order_length );
    output
}
fn delve_for_orders(idx:usize, springs: Vec<Spring>, order:&[usize])->u32{
    if idx == springs.len(){
        if spring_config_is_valid(&springs,order){
            #[cfg(test)]{
                for s in springs.iter(){
                    print!("{s}");
                }
                println!();
            }
            return 1;
        }else{
            return 0;
        }
    }
    if springs[idx] == Spring::Unknown {
        let mut fixed_version = springs.clone().to_vec();
        fixed_version[idx] = Spring::Working;
        let mut broken_version = springs.clone().to_vec();
        broken_version[idx] = Spring::Broken;
        return delve_for_orders(idx+1, fixed_version, order) + delve_for_orders(idx+1,broken_version, order);
    }else{
        return delve_for_orders(idx+1, springs, order);
    }
}

pub fn main_1(file_name:&str)->Option<u32>{
    let mut lines = read_lines(file_name).unwrap();
    let mut total = 0;
    #[cfg(test)]
    let mut idx = 0;
    #[cfg(test)]
    let expected_row_counts = vec![1,4,1,1,4,10];
    while let Some(Ok(line)) = lines.next(){
        let (springs,order) = process_line(line);
        let row_count = delve_for_orders(0, springs, &order);
        #[cfg(test)]{
            if file_name == r"src\dummy.txt"{
                assert_eq!(row_count,expected_row_counts[idx])
            }
            idx +=1;

        }
        total += row_count;

    }
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn solution_validity(){
        use super::Spring::*;
        let springs = vec![Broken,Working,Working,Broken,Broken,Working,Broken];
        let order = vec![1,2,1];
        assert!(spring_config_is_valid(&springs,&order))

    }

}
