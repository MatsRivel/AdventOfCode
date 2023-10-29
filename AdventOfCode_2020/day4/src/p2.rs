use std::{fs::read_to_string, string};
use crate::p1::{self, passport_batch};

struct Passport{
    byr: u32,
    ecl: String,
    eyr: u32,
    hcl: String,
    hgt: u32,
    iyr: u32,
    pid: u32,
}
fn is_present(sorted_fields: Vec<&str>)->bool{
    let mut correct_fields = ["byr", "iyr", "eyr", "hgt", "hcl","ecl","pid"];
    correct_fields.sort();
    for field in correct_fields.iter(){
        if !sorted_fields.contains(field){
            return false;
        }
    }
    true
}
impl Passport{
    fn new(string:String) -> Option<Self>{
        let mut components = string.split(" ").map(|s|{
            let component:[&str;2] = s.split(":").collect::<Vec<&str>>().try_into().unwrap();
            component
        }).collect::<Vec<[&str;2]>>();
        components.sort();
        // Verify that all fields are present.
        let fields = components.iter().map(|pair| pair[0]).collect::<Vec<&str>>();
        if !is_present(fields){
            return None;
        }
        // Make a passport:
        let byr = components[0][1].parse::<u32>().unwrap();
        let ecl = components[1][1].to_string();
        let eyr = components[2][1].parse::<u32>().unwrap();
        let hcl = components[3][1].to_string();
        let hgt = components[4][1].parse::<u32>().unwrap();
        let iyr = components[5][1].parse::<u32>().unwrap();
        let pid = components[6][1].parse::<u32>().unwrap();
        Some( Self { byr, ecl, eyr, hcl, hgt, iyr, pid } )
    }
    fn is_valid(&self) -> bool{
        if self.byr < 1920 || self.byr > 2002{
            return false;
        }else if self.iyr < 2010 || self.iyr > 2020{
            return false;
        }else if self.eyr < 2020 || self.eyr > 2030{
            return false;
        }
        todo!()
    }
}

pub fn main_2(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut batch = passport_batch(&data_string);
    let passports = batch.iter().filter_map(|b|{
        Passport::new(b.to_string())
    }).collect::<Vec<Passport>>();
    todo!()
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
