use std::{fs::read_to_string, ops::RangeInclusive, collections::HashMap};

struct UniqueID{
    id:u32
}
struct Template{
    template: HashMap<String,[RangeInclusive<u32>;2]>,
}
impl Template{
    fn new(template_info:&Vec<&str>) -> Self{
        let keys_and_vals = template_info.iter()
            .map(|&s|{
                let [key,v_str]:[&str;2] = s.split(": ").collect::<Vec<&str>>().try_into().unwrap();
                let [left_range, right_range]:[&str;2] = v_str.split("-").collect::<Vec<&str>>().try_into().unwrap();
                let [lower_left, upper_left]:[&str;2] = left_range.split("-").collect::<Vec<&str>>().try_into().unwrap();
                let [lower_right, upper_right]:[&str;2] = right_range.split("-").collect::<Vec<&str>>().try_into().unwrap();
                (key.to_string(),
                 [lower_left.parse::<u32>().unwrap(),
                 upper_left.parse::<u32>().unwrap(),
                 lower_right.parse::<u32>().unwrap(),
                 upper_right.parse::<u32>().unwrap()]
                )
            });//.collect::<Vec<(String,[u32;4])>>();
        let mut template = HashMap::<String,[RangeInclusive<u32>;2]>::new();
        for (key,[lower_left,upper_left,lower_right,upper_right]) in keys_and_vals{
            template.insert(key, [lower_left..=upper_left, lower_right..=upper_right]);
        }
        Self{template}
    }
}
struct Ticket<'a>{
    template: &'a Template,
    nums_for_fields: HashMap<String,Vec<UniqueID>>,
    fields_for_nums: HashMap<UniqueID,Vec<String>>,
    
}
impl Ticket{
    fn new(ticket_line:&str, template:&'a Template)->Self{
        let nums = ticket_line.split(",").map(|s| s.parse::<u32>().unwrap() ).collect::<Vec<u32>>();
        let nums_for_fields = template.template.iter().map(|(key,ranges){
            let filtered_vals = nums.iter().filter_map(|v|{
                if ranges[0].contains(v) || ranges[1].contains(v){
                    Some(v)
                }else{
                    None
                }
            }).collect::<Vec<u32>>();
            (key.clone(),filtered_vals)
        })
        Self{template,uncertain_nums:nums, }
    }

}

pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
