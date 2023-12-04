use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

pub fn read_lines<P>(file_name:P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn process_line(line:&str)->(Vec<u32>,Vec<u32>){
    let [_,numbers]: [&str;2] = line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
    let [winners, current]:[Vec<u32>;2] = numbers
        .split(" | ")
        .map(|segment| {
            segment.split(" ")
                .filter_map(|s|{ s.parse::<u32>().ok()} )
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .try_into()
        .unwrap();
    (winners, current)

}
pub fn main_1(file_name:&str)->Option<i32>{
    let lines = read_lines(file_name).expect("We don't mind crashing this early.");
    let mut total = 0;
    for line_result in lines{
        if let Ok(line) = line_result{
            let (winners, current) = process_line(line.as_str());
            let mut score = 0;
            for key in current.iter(){
                if winners.contains(key){
                    if score == 0{
                        score = 1;
                    }else{
                        score *=2;
                    }
                }
            }
            total += score;
        }
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
