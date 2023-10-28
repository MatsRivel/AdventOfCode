mod part1{
    use std::fs::read_to_string;

    pub struct Section{
        pub lower_bound: u32,
        pub upper_bound: u32,
    }
    impl Section{
        fn new(line_half: &str) -> Self{
            let [lower_str, upper_str]:[&str;2] = line_half.split("-").collect::<Vec<&str>>().try_into().expect("Failed line-split!\n");
            Self { 
                lower_bound: lower_str.parse::<u32>().expect("Failed to convert lower_str &str to u32") ,
                upper_bound: upper_str.parse::<u32>().expect("Failed to convert upper_str &str to u32")
             }
        }
        pub fn contains_other(&self, other: &Section) -> bool{
            if self.lower_bound <= other.lower_bound && self.upper_bound >= other.upper_bound{
                return true;
            }else if other.lower_bound <= self.lower_bound && other.upper_bound >= self.upper_bound{
                return true;
            }
            return false;
        }
    }
    pub struct Pair{
        pub section_a: Section,
        pub section_b: Section,
    }
    impl Pair{
        fn new(line:&str) -> Self{
            let [left, right]:[&str;2] = line.split(",").collect::<Vec<&str>>().try_into().expect("Failed line-split!\n");
            Self { 
                section_a: Section::new(left),
                section_b: Section::new(right) 
            }
        }
    }

    pub fn process_line(line:&str) -> Pair{
        Pair::new(line)
    }
    pub fn get_pairs(file_name: &str, line_processer: fn(line:&str) -> Pair) -> Result<Vec<Pair>,std::io::Error>{
        let pairs = read_to_string(file_name)?
            .lines()
            .map(|line| {
                line_processer(line)
            })
            .collect::<Vec<Pair>>();
        Ok(pairs)
    }
    fn count_overlapping_pairs(pairs: Vec<Pair>) -> u32{
        pairs.iter()
            .map(|pair| {
                match &pair.section_a.contains_other(&pair.section_b){
                    true => 1,
                    false => 0,
                }
            }).sum()
    }
    pub fn main_1(file_name:&str){
        let pairs = get_pairs(file_name, process_line).expect("Failed to process line");
        let count = count_overlapping_pairs(pairs);
        println!("Part1 output: {count}");

    }
}
mod part2{
    use crate::part1;
    
    fn is_middle(left:u32, middle:u32,right:u32) -> bool{
        left <= middle && middle <= right
    }
    trait PartialOverlap{
        fn partial_overlap(&self, other:&Self) -> bool;
    }
    impl PartialOverlap for part1::Section{
        fn partial_overlap(&self, other:&Self) -> bool{
            let lower_overlap = is_middle(self.lower_bound, other.lower_bound, self.upper_bound);
            let upper_overlap = is_middle(self.lower_bound, other.upper_bound, self.upper_bound);
            lower_overlap || upper_overlap
        }
    }
    fn count_overlapping_pairs(pairs: Vec<part1::Pair>) -> u32{
        pairs.iter()
            .map(|pair| {
                let overlap = pair.section_a.partial_overlap(&pair.section_b) || pair.section_a.contains_other(&pair.section_b);
                match overlap {
                    true => 1,
                    false => 0,
                }
            }).sum()
    }
    pub fn main_2(file_name:&str){
        let pairs = part1::get_pairs(file_name, part1::process_line).expect("Failed to process line");
        let count = count_overlapping_pairs(pairs);
        println!("Part2 output: {count}");
    }
}

use part1::main_1;
use part2::main_2;
fn main() {
    //let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
    main_1(file_name);
    main_2(file_name);
}
