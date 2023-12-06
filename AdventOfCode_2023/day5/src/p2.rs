use std::fs::read_to_string;
use crate::p1::{process_data_string, Almanac};
fn merge_ranges(mut ranges:Vec<[u64;2]>)->Vec<[u64;2]> {
    ranges.sort();
    for i in 0..ranges.len()-1 {
        let lower = ranges[i];
        let upper = ranges[i+1];
        if (lower[0] <= upper[0] && upper[0] <= lower[1] && lower[1] <= upper[1] ) || lower[1]+1 == upper[0]{
            ranges[i+1] = [lower[0], upper[1]];
            ranges[i] = [0,0]; // Ignore this range for now :)
        }
    }
    ranges.into_iter().filter(|r| *r != [0,0]).collect()

}

fn get_range_overlap(a:[u64;2], b:[u64;2])->Option<[u64;2]>{
    todo!()
}
impl Almanac{
    fn id_in_ranges_reverse(id:u64, range: &Vec<[u64;3]>)->u64{
        for [left, right, length] in range.iter(){
            if left <= &id && id <left + length{
                return id - left + right ; 
            }
        }
        return id;
    }
    fn loc_to_seed_range(loc:u64)->Option<[u64;2]>{

        todo!()
    }
}
pub fn main_2(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let (seeds_range_data, data) = process_data_string(data_string);
    let almanac = Almanac::new(data);
    let mut min_loc = u64::MAX;
    let mut i = 0;
    let mut seed_ranges = vec![];
    while i < seeds_range_data.len() {
        seed_ranges.push([seeds_range_data[i], seeds_range_data[i]+seeds_range_data[i+1]]);
        i+=2;
    }
    seed_ranges = merge_ranges(seed_ranges);
    let mut loc_ranges = vec![];
    for [loc_start, _humid_start, length] in almanac.humid_to_loc.iter() {
        loc_ranges.push([*loc_start, *loc_start + *length]);
    }
    loc_ranges = merge_ranges(loc_ranges);

    for [lower,upper] in loc_ranges.into_iter() {
        let range = lower..upper;

    }


    Some(min_loc)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
