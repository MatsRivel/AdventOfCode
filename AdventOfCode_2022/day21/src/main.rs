mod p1{
    use std::fs::read_to_string;
    use std::collections::HashMap;

    pub struct YellApe<'a>{
        name: &'a str,
        num: i64
    }
    impl<'a> YellApe<'a>{
        fn new(line:&'a str) -> Self{
            let [mut name, mut num_str]:[&str;2] = line.split(":").collect::<Vec<&str>>().try_into().expect("Should not be falable");
            name = name.trim();
            num_str = num_str.trim();
            let num = num_str.parse::<i64>().expect(&format!("Failed to parse\"{:?}\" into i64",num_str));
            Self{name,num}
        }
    }
    fn add_funk(a:i64,b:i64) -> i64{
        a+b
    }
    fn sub_funk(a:i64,b:i64) -> i64{
        a-b
    }
    fn div_funk(a:i64,b:i64) -> i64{
        a/b
    }
    fn mult_funk(a:i64,b:i64) -> i64{
        a*b
    }
    pub struct ActApe<'a>{
        name: &'a str,
        neighbours: [&'a str;2],
        opr: fn(i64,i64) -> i64
    }
    impl<'a> ActApe<'a>{
        fn new(line:&'a str, opr:char) -> Self{
            let [mut name, rest_str]:[&str;2] = line.split(":").collect::<Vec<&str>>().try_into().expect("Should not be falable");
            name = name.trim();
            let neighbours:[&str;2] = rest_str.split(opr).map(|s| s.trim()).collect::<Vec<&str>>().try_into().expect("Should be infallable");
            let f = match opr{
                '+' => add_funk,
                '-' => sub_funk,
                '*' => mult_funk,
                '/' => div_funk,
                _ => panic!()
            };
            Self { name, neighbours, opr: f }
        }

    }
    enum ApeVersion<'a>{
        YellApeVersion(YellApe<'a>),
        ActApeVersion(ActApe<'a>)
    }


    fn line_is_yeller(line:&str) -> (bool,char){
        for opr in ['+', '-', '/','*'].iter(){
            if line.contains(*opr){
                return (false,*opr);
            }
        }
        return (true,'.');
    }

    fn process_data_string(data_string:&str) -> HashMap<&str,ApeVersion>{
        let data: HashMap<&str,ApeVersion> = data_string
            .lines()
            .map(|line|{
                let (is_yeller, opr) = line_is_yeller(line); 
                if is_yeller{
                    let ape = YellApe::new(line);
                    (ape.name,ApeVersion::YellApeVersion(ape))
                }else {
                    let ape = ActApe::new(line,opr);
                    (ape.name,ApeVersion::ActApeVersion(ape))
                }
            }).collect::<HashMap<&str,ApeVersion>>();
        data
    }
    
    fn traverse_recursively<'a>(apes:&HashMap<&'a str,ApeVersion>, current:&'a str) -> i64{
        match apes.get(current).unwrap(){
            ApeVersion::YellApeVersion(ape) => {
                return ape.num;
            }
            ApeVersion::ActApeVersion(ape) => {
                let num_a = traverse_recursively(apes, ape.neighbours[0]);
                let num_b= traverse_recursively(apes, ape.neighbours[1]);
                return (ape.opr)(num_a,num_b)
            }
        }
    }
    pub fn main_1(file_name:&str)->Option<i64>{
        let data_string = read_to_string(file_name).expect("We know the file exists.");
        let mut apes: HashMap<&str, ApeVersion> = process_data_string(&data_string);
        let result = traverse_recursively(&mut apes,"root");
        Some(result)
    }

#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}
mod p2{

    use std::fs::read_to_string;
    use std::collections::HashMap;

    pub struct YellApe<'a>{
        name: &'a str,
        num: i64
    }
    impl<'a> YellApe<'a>{
        fn new(line:&'a str) -> Self{
            let [mut name, mut num_str]:[&str;2] = line.split(":").collect::<Vec<&str>>().try_into().expect("Should not be falable");
            name = name.trim();
            num_str = num_str.trim();
            let num = num_str.parse::<i64>().expect(&format!("Failed to parse\"{:?}\" into i64",num_str));
            Self{name,num}
        }
    }
    fn add_funk(a:i64,b:i64) -> i64{
        a+b
    }
    fn sub_funk(a:i64,b:i64) -> i64{
        a-b
    }
    fn div_funk(a:i64,b:i64) -> i64{
        a/b
    }
    fn mult_funk(a:i64,b:i64) -> i64{
        a*b
    }
    pub struct ActApe<'a>{
        name: &'a str,
        neighbours: [&'a str;2],
        opr: fn(i64,i64) -> i64
    }
    impl<'a> ActApe<'a>{
        fn new(line:&'a str, opr:char) -> Self{
            let [mut name, rest_str]:[&str;2] = line.split(":").collect::<Vec<&str>>().try_into().expect("Should not be falable");
            name = name.trim();
            let neighbours:[&str;2] = rest_str.split(opr).map(|s| s.trim()).collect::<Vec<&str>>().try_into().expect("Should be infallable");
            let f = match opr{
                '+' => add_funk,
                '-' => sub_funk,
                '*' => mult_funk,
                '/' => div_funk,
                _ => panic!()
            };
            Self { name, neighbours, opr: f }
        }
        fn do_opr(&self,a: i64,b: i64)->i64{
            (self.opr)(a,b)
        }

    }
    pub struct HumanApe<'a>{
        name: &'a str,
        opr_chain: Vec<fn(i64,i64) -> i64>
    }
    impl<'a> HumanApe<'a>{
        fn new(line:&'a str) -> Self{
            let [mut name, mut num_str]:[&str;2] = line.split(":").collect::<Vec<&str>>().try_into().expect("Should not be falable");
            name = name.trim();
            Self{name, opr_chain: Vec::<fn(i64,i64) -> i64>::new() }
        }
    }
    pub enum ApeVersion<'a>{
        YellApeVersion(YellApe<'a>),
        ActApeVersion(ActApe<'a>),
        HumanApeVersion(HumanApe<'a>)
    }


    pub fn line_is_yeller(line:&str) -> (bool,char){
        for opr in ['+', '-', '/','*'].iter(){
            if line.contains(*opr){
                return (false,*opr);
            }
        }
        return (true,'.');
    }
    pub fn line_is_human(line:&str) -> (bool,char){
        return (line.contains(HUMAN_NAME), '.')
    }

    fn process_data_string(data_string:&str) -> HashMap<&str,ApeVersion>{
        let data: HashMap<&str,ApeVersion> = data_string
            .lines()
            .map(|line|{
                let (is_yeller, opr) = line_is_yeller(line); 
                let (is_human,_) = line_is_human(line);
                if is_human{
                    let ape = HumanApe::new(line);
                    (ape.name, ApeVersion::HumanApeVersion(ape))
                }else if is_yeller{
                    let ape = YellApe::new(line);
                    (ape.name,ApeVersion::YellApeVersion(ape))
                }else {
                    let ape = ActApe::new(line,opr);
                    (ape.name,ApeVersion::ActApeVersion(ape))
                }
            }).collect::<HashMap<&str,ApeVersion>>();
        data
    }
    
    fn traverse_recursively<'a>(apes:&HashMap<&'a str,ApeVersion>, current:&'a str) -> (Option<i64>,Option<fn(i64,i64) -> i64>){
        match apes.get(current).unwrap(){
            ApeVersion::YellApeVersion(ape) => {
                return (Some(ape.num),None);
            }
            ApeVersion::ActApeVersion(ape) => {
                let num_a: (Option<i64>, Option<fn(i64, i64) -> i64>) = traverse_recursively(apes, ape.neighbours[0]);
                let num_b: (Option<i64>, Option<fn(i64, i64) -> i64>) = traverse_recursively(apes, ape.neighbours[1]);
                match (num_a,num_b){
                    ((Some(a),None),(Some(b),None)) => {
                        let result = ape.do_opr(a, b);
                        return (Some(result),None);
                    },
                    ((Some(a),None),(None,Some(op))) => {

                    }
                }

            },
            ApeVersion::HumanApeVersion(ape) => {

                todo!()
            }
        }
    }
    const HUMAN_NAME:&str= "humn:";
    pub fn main_2(file_name:&str)->Option<i64>{
        let data_string = read_to_string(file_name).expect("We know the file exists.");
        let mut apes: HashMap<&str, ApeVersion> = process_data_string(&data_string);
        let result = traverse_recursively(&mut apes,"root");
        Some(result)
    }
#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn my_test(){
	
	}
	
}

}

use p1::main_1;
use p2::main_2;
use std::time::Instant;
fn main() {
    let file_name = r"src\dummy_input.txt";
    let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.unwrap(),152,"Wrong dummy output!")
    }
    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
    if file_name == r"src\dummy_input.txt"{
        assert_eq!(count.unwrap(),301,"Wrong dummy output!")
    }
}
