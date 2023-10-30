use std::{fs::read_to_string, string, collections::HashMap};
use crate::p1::{self, passport_batch};

#[derive(Debug)]
struct Passport{
    byr: u32,
    ecl: String,
    eyr: u32,
    hcl: String,
    hgt: String,
    iyr: u32,
    pid: String,
}
fn is_present(sorted_fields: &Vec<&str>)->bool{
    let mut correct_fields = ["byr", "iyr", "eyr", "hgt", "hcl","ecl","pid"];
    correct_fields.sort();
    for field in correct_fields.iter(){
        if !sorted_fields.contains(field){
            return false;
        }
    }
    true
}

fn byr(s:&str)->bool{
    let val = match s.parse::<u32>(){
        Ok(v) => Some(v),
        Err(_) => {
            println!("Invalid byr!: {}",s);
            None
        }
    };
    if let Some(byr) = val{
        if byr >= 1920 || byr <= 2002{
            return true;
        }
    }
    false
}
fn ecl(s:&str)->bool{
    let ecl: String = s.to_string();
    if ["amb","blu","brn","gry","grn","hzl","oth"].contains( &ecl.as_str()) {
        return true;
    }
    false
}
fn eyr(s:&str)->bool{
    let eyr = match s.parse::<u32>(){
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };
    if eyr >= 2020 || eyr <= 2030{
        return true;
    }
    false
}
fn hcl(s:&str)->bool{
    let hcl = s.to_string();
    if let Some(colour) = hcl.strip_prefix("#"){
        if colour.len() != 6{
            return false;
        }
        // Must consist of only these characters:
        for c in colour.chars(){
            if !['a','b','c','d','e','f'].contains(&c) && ! ['0','1','2','3','4','5','6','7','8','9'].contains(&c){
                return false;
            }
        }
    }else{ // Haircolour has to start with a "#"
        return false;
    }
}
fn hgt(s:&str)->bool{
    let hgt = s.to_string();
    if let Some(height_string) = hgt.strip_suffix("cm"){
        let height_in_cm = height_string.parse::<u32>().unwrap();
        if height_in_cm < 150 || height_in_cm > 193{
            return false;
        }
    }else if let Some(height_string) = hgt.strip_suffix("in"){
        let height_in_inch = height_string.parse::<u32>().unwrap();
        if height_in_inch < 59 || height_in_inch > 76{
            return false;
        }
    }else{ // Return false if height is neither cm or inches.
        return false;
    }
}
fn iyr(s:&str)->bool{
    let iyr = match s.parse::<u32>(){
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };
    if iyr < 2010 || self.iyr > 2020{
        return false;
    }
    true
}
fn pid(s:&str)->bool{
    let pid = s;
    if pid.len() != 9{
        return false;
    }
    true
}
fn get_components(string:String) -> Option<Vec<[String;2]>>{
    let mut components = string.split(" ").enumerate().filter_map(|(i,s)|{
        if s == ""{
            None
        }else{
            let component:[&str;2] = s.split(":").collect::<Vec<&str>>().try_into().unwrap();
            Some(component)
        }
    }).collect::<Vec<[&str;2]>>();
    components.sort();
    // Verify that all fields are present.
    let fields = components.iter()
        .filter_map(|pair| {
            if pair[0] != "cid"{
                Some(pair[0])
            }else{
                None
            }
        }).collect::<Vec<&str>>();
    if !is_present(&fields){
        return None;
    };
    let new_components = components.iter().map(|[left,right]|{
        [left.to_string(), right.to_string()]
    }).collect::<Vec<[String;2]>>();
    Some(new_components)
}

pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut batch = passport_batch(&data_string);
    let passports = batch.iter().filter_map(|b|{
        get_components(*b)
    }).collect::<Vec<Vec<[String;2]>>>();
    let n_valid_passports = passports.iter().fold(0, |acc, passport| {
        if passport.is_valid(){
            acc+1
        }else{
            acc
        }
    });
    Some(n_valid_passports)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
