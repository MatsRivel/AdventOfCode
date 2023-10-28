mod Part1{
    use std::fs;
    pub fn get_max_sum_from_file(file_name:&str) -> Result<i32,std::io::Error>{
        let max_of_vec = *fs::read_to_string(file_name)?
            .lines()
            .fold(Vec::new(), |mut acc, line| {
                //println!("|{}|",line);
                if line == "" || acc.is_empty(){
                    acc.push(Vec::new());
                }else{
                    let num = match line.parse::<i32>(){
                        Ok(v) => v,
                        Err(_) => 0
                    };
                    acc.last_mut().expect("Failed to get last element of acc").push(num);
                }
                acc
            })
            .iter()
            .map(|inner| {
                //println!("{:?}",inner);
                (*inner).iter().sum::<i32>()
            })
            .collect::<Vec<i32>>()
            .iter()
            .max()
            .expect("No elements in vector");

        Ok(max_of_vec)
    }
}

mod Part2{
    use std::fs;
    pub fn get_three_max_sum_from_file(file_name:&str) -> Result<i32,std::io::Error>{
        let mut max_vec = vec![0,0,0]; 
        fs::read_to_string(file_name)?
            .lines()
            .fold(Vec::new(), |mut acc, line| {
                //println!("|{}|",line);
                if line == "" || acc.is_empty(){
                    acc.push(Vec::new());
                }else{
                    let num = match line.parse::<i32>(){
                        Ok(v) => v,
                        Err(_) => 0
                    };
                    acc.last_mut().expect("Failed to get last element of acc").push(num);
                }
                acc
            })
            .iter()
            .map(|inner| {
                //println!("{:?}",inner);
                (*inner).iter().sum::<i32>()
            })
            .collect::<Vec<i32>>()
            .iter()
            .for_each(|x| {
                let [min, mid, max] = [max_vec[0], max_vec[1], max_vec[2]];
                if x > &min{
                    if x <= &mid{
                        max_vec[0] = *x;
                    }else if x > &mid && x <= &max{
                        max_vec[0] = max_vec[1];
                        max_vec[1] = *x;
                    } else{
                        max_vec[0] = max_vec[1];
                        max_vec[1] = max_vec[2];
                        max_vec[2] = *x;
                    }
                }
            });

        Ok(max_vec[0]+max_vec[1]+max_vec[2])
    }
}
use Part1::get_max_sum_from_file;
use Part2::get_three_max_sum_from_file;
fn main() {
    let file_name = "puzzle_input.txt";
    println!("{:?}",get_max_sum_from_file(file_name).expect("Reading failed"));
    println!("{:?}",get_three_max_sum_from_file(file_name).expect("Reading failed"));
}
