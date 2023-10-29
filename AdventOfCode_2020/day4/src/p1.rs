
pub fn valid_pass(string:&str)->bool{
    let mut components = string.split(" ").map(|s|{
        let component = s.split(":").collect::<Vec<&str>>();
        component[0]
    }).collect::<Vec<&str>>();
    components.sort();
    let mut fields = ["byr", "iyr", "eyr", "hgt", "hcl","ecl","pid"];
    fields.sort();
    for field in fields.iter(){
        if !components.contains(field){
            return false;
        }
    }
    true
}
pub fn passport_batch(data_string:&str) -> Vec<String>{
    let mut batch = Vec::<String>::new();
    let mut temp_string = "".to_string();
    for line in data_string.lines() {
        if line == ""{
            batch.push(temp_string);
            temp_string = "".to_string();
            continue;
        }
        temp_string.push_str(format!(" {line}").as_str());
    }
    batch.push(temp_string);
    batch
}
use std::fs::read_to_string;
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let batch = passport_batch(&data_string);
    let n_valid_passes = batch.iter().fold(0, |acc,pass|{
        if valid_pass(pass){
            acc+1
        }else{
            acc
        }
    });
    Some(n_valid_passes)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
