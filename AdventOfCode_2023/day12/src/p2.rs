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

fn process_line(line:String) -> (Vec<(u32,Spring)>,Vec<usize>,[usize;2]){
    let [springs_str,order_str]:[&str;2] = line.split(" ").collect::<Vec<&str>>().try_into().unwrap();
    let single_springs_raw = springs_str
        .chars()
        .filter_map(|c| {
            match c{
                '?' => Some(Spring::Unknown),
                '.' => Some(Spring::Working),
                '#' => Some(Spring::Broken),
                _ => None
            }
        }).collect::<Vec<Spring>>();
    let single_order = order_str.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let base_spring_size = single_springs_raw.len();
    let base_order_size = single_order.len();


    let mut single_springs = vec![];
    let mut counter = 1;
    let mut what_is_counted = single_springs_raw[0];
    for element in single_springs_raw.iter().skip(1){
        if *element == what_is_counted{
            counter +=1;
        }else{
            single_springs.push((counter,what_is_counted));
            counter = 1;
            what_is_counted = *element;
        }
    }
    single_springs.push((counter,what_is_counted));

    let mut order = Vec::with_capacity(single_order.len()*5);
    let mut springs = Vec::with_capacity(single_springs.len()*5 + 4);
    for i in 0..5{
        order.append(&mut single_order.clone());
        springs.append(&mut single_springs.clone());
        if i != 4{
            springs.push((1,Spring::Unknown))
        }
    }
    // Merge instances of same type spring twice.
    let spring_length = springs.len();
    for i in 1..spring_length {
        let mut overlap_count = 0;
        if springs[i-1].1 == springs[i].1{
            overlap_count = springs[i].0;
        }
        if overlap_count != 0{
            let (count, _) = springs.get_mut(i-1).unwrap();
            *count += overlap_count;
            let (count, _) = springs.get_mut(i).unwrap();
            *count = 0;
        }
    }
    // Remove any zero-count occurences.
    springs = springs.iter().filter(|(count,_)| *count != 0).map(|inner| *inner).collect();
    #[cfg(test)]{
        for (count,s) in springs.iter(){
            print!("{count}:{s}, ");
        }
        println!();
    }
    (springs,order, [base_spring_size,base_order_size])
}  

fn spring_config_is_valid(springs: &[Spring], order:&[usize])->bool{
    todo!()
}
fn delve_for_orders(mut idx:usize, springs: Vec<(u32,Spring)>, order:&[usize],spring_size:usize,order_size:usize)->u32{
    todo!()
}

pub fn main_2(file_name:&str)->Option<u32>{
    let mut lines = read_lines(file_name).unwrap();
    let mut total = 0;
    #[cfg(test)]
    let mut idx = 0;
    #[cfg(test)]
    let expected_row_counts = vec![1,16384,1,16,2500,506250];
    while let Some(Ok(line)) = lines.next(){
        let (springs,order, [base_spring_size,base_order_size]) = process_line(line);
        let row_count = delve_for_orders(0, springs, &order,base_spring_size,base_order_size);
        #[cfg(test)]{
            if file_name == r"src\dummy.txt"{
                assert_eq!(row_count,expected_row_counts[idx],"Failed to meet expected row count");
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
    #[test]
    fn solving_test(){
        let line = "????.#...#... 4,1,1".to_string();
        let (springs,order,[spring_size,order_size]) = process_line(line);
        let row_count = delve_for_orders(0, springs, &order,spring_size,order_size);
        assert_eq!(row_count,16)
    }
    #[test]
    fn partial_solution_validity(){
        use super::Spring::*;
        let springs = vec![Broken,Working,Working,Broken,Broken,Working,Broken];
        let order = vec![1,2,1,1,1,1];
        assert!(spring_config_is_valid(&springs,&order))
    }

}
