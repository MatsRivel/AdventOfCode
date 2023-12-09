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
pub fn main_2(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let (seeds_range_data, data) = process_data_string(data_string);
    let almanac = Almanac::new(data);
    let mut min_loc = u64::MAX;
    let mut i = 0;
    let mut seed_ranges = vec![];
    // println!("{seeds_range_data:?}");
    while i < seeds_range_data.len() {
        seed_ranges.push([seeds_range_data[i], seeds_range_data[i]+seeds_range_data[i+1]]);
        i+=2;
    }
    // println!("{seed_ranges:?}");
    seed_ranges = merge_ranges(seed_ranges);
    // println!("{seed_ranges:?}");
    for [lower,upper] in seed_ranges.into_iter() {
        let range = lower..upper;
        let loc = range.map(|seed| almanac.seed_to_loc(seed)).fold(u64::MAX, |acc,val| std::cmp::min(acc,val));
        min_loc = std::cmp::min(loc,min_loc);
        i+=2;
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
