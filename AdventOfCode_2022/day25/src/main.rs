mod p1{
    use core::panic;
    use std::cmp::Ordering;
    use std::error::Error;
    use std::fs::read_to_string;
    use std::ops::Add;
    #[derive(Clone)]
    struct SNAFU{
        digits: Vec<SNAFUDigit>,
    }
    impl Add<SNAFU> for SNAFU{
        type Output=SNAFU;

        fn add(self, rhs: SNAFU) -> Self::Output {
            todo!()
        }
    }
    impl SNAFU{
        fn len(&self) -> usize{
            self.digits.len()
        }
        fn new(line:&str) -> Self{
            let snafus = line.split("").filter_map(|c| {
                SNAFUDigit::new_from_snafu_str(c)
            } ).collect::<Vec<SNAFUDigit>>();
            let new_snafu = Self{ digits: snafus};
            let new_snafu_string = new_snafu.snafu_to_string();
            assert_eq!(line,&new_snafu_string,"Wrong snafu-reading! {line} != {}",&new_snafu_string);
            new_snafu
        }
        fn depth_search(current:SNAFU, target_value:i128, idx:i32) -> Option<SNAFU>{
            let current_value = current.to_decimal();
            println!("{current_value} | {target_value} |{}", current_value > target_value);
            if current_value == target_value{
                return Some(current);
            }else if idx < 0 || current_value <0{
                return None;
            }
            for dig in [SNAFUDigit::Two, SNAFUDigit::One, SNAFUDigit::Zero, SNAFUDigit::Minus,SNAFUDigit::DoubleMinus].iter(){
                // Prevent searching for bigger numbers if we're too big, and smaller if we're too small.

                // match current.digits[idx as usize]{
                //     SNAFUDigit::Two => {
                //         if current_value < target_value && dig <= &SNAFUDigit::Two{
                //             continue;
                //         }else if current_value > target_value && dig >= &SNAFUDigit::Two{
                //             ()
                //         }
                //     },
                //     SNAFUDigit::One => {
                //         if current_value < target_value && dig <= &SNAFUDigit::One{
                //             continue;
                //         }else if current_value > target_value && dig >= &SNAFUDigit::One{
                //             continue;
                //         }
                //     },
                //     SNAFUDigit::Zero => {
                //         if current_value < target_value && dig <= &SNAFUDigit::Zero{
                //             continue;
                //         }else if current_value > target_value && dig >= &SNAFUDigit::Zero{
                //             continue;
                //         }
                //     },
                //     SNAFUDigit::Minus => {
                //         if current_value < target_value && dig <= &SNAFUDigit::Minus{
                //             continue;
                //         }else if current_value > target_value && dig >= &SNAFUDigit::Minus{
                //             continue;
                //         }
                //     },
                //     SNAFUDigit::DoubleMinus => {
                //         if current_value < target_value && dig <= &SNAFUDigit::DoubleMinus{
                //             continue;
                //         }else if current_value > target_value && dig >= &SNAFUDigit::DoubleMinus{
                //             continue;
                //         }
                //     },
                // }
                let mut new_digits = current.digits.clone();
                new_digits[idx as usize] = dig.clone();
                let new_snafu = SNAFU{digits: new_digits};
                if let Some(snafu_found) = SNAFU::depth_search(new_snafu, target_value, idx-1){
                    return Some(snafu_found);
                }
            }
            return None;
        }
        fn new_from_num(num_in:i128) -> Option<Self>{
            let snafu_of_digits = format!("{num_in}").chars().map(|c|{
                SNAFU{ digits: SNAFUDigit::new_from_digit_char(c)}
            }).collect::<Vec<SNAFU>>();
            let completed_digits = snafu_of_digits.iter().fold(SNAFU{digits:SNAFUDigit::new_from_digit_char('0')}, |acc,rhs|{
                acc + rhs.clone()
            });
            Some(completed_digits)
        }
        fn to_decimal(&self) -> i128{
            self.digits.iter().rev().enumerate().fold(0,|acc,(idx, snafu_dig)|{
                acc + snafu_dig.to_decimal(idx)
            })
        }
        fn snafu_to_string(&self) -> String{
            self.digits.iter().map(|snafu|{
                match snafu{
                    SNAFUDigit::Zero => "0",
                    SNAFUDigit::One => "1",
                    SNAFUDigit::Two => "2",
                    SNAFUDigit::Minus => "-",
                    SNAFUDigit::DoubleMinus => "=",
                }
            }).collect::<String>()
        }

    }

    #[derive(PartialEq, Clone,Copy)]
    enum SNAFUDigit{
        Zero,
        One,
        Two,
        Minus,
        DoubleMinus
    }
    struct SNAFUDigitResult{
        digits: Vec<SNAFUDigit>,
        decremented: bool
    }
    impl SNAFUDigitResult{
        fn new(digits: Vec<SNAFUDigit>, decremented: bool) -> Self{
            SNAFUDigitResult { digits, decremented }
        }
        fn len(&self) -> usize{
            self.digits.len()
        }
    }
    impl PartialOrd<SNAFUDigit> for SNAFUDigit{
        fn partial_cmp(&self, other: &SNAFUDigit) -> Option<Ordering> {
            match &self{
                SNAFUDigit::Two => {
                    match other{
                        SNAFUDigit::Two => Some(Ordering::Equal),
                        _ => Some(Ordering::Greater)
                    }
                },
                SNAFUDigit::One => {
                    match other{
                        SNAFUDigit::Two => Some(Ordering::Less),
                        SNAFUDigit::One => Some(Ordering::Equal),
                        _ => Some(Ordering::Greater)
                    }
                },
                SNAFUDigit::Zero => {
                    match other{
                        SNAFUDigit::Two | SNAFUDigit::One  => Some(Ordering::Less),
                        SNAFUDigit::Zero => Some(Ordering::Equal),
                        _ => Some(Ordering::Greater)
                    }
                },
                SNAFUDigit::Minus => {
                    match other{
                        SNAFUDigit::DoubleMinus => Some(Ordering::Greater),
                        SNAFUDigit::Minus => Some(Ordering::Equal),
                        _ => Some(Ordering::Less)
                    }
                },
                SNAFUDigit::DoubleMinus =>{
                    match other{
                        SNAFUDigit::DoubleMinus => Some(Ordering::Equal),
                        _ => Some(Ordering::Less)
                    }
                },
            }
        }
    }
    impl Add<SNAFUDigit> for SNAFUDigit{
        type Output = SNAFUDigitResult;

        fn add(self, rhs: SNAFUDigit) -> Self::Output {
            match self{
                SNAFUDigit::Two => {
                    match rhs{
                        SNAFUDigit::Two =>          SNAFUDigitResult::new(vec![SNAFUDigit::One, SNAFUDigit::Minus] , false),
                        SNAFUDigit::One =>          SNAFUDigitResult::new(vec![SNAFUDigit::One, SNAFUDigit::DoubleMinus] , false),
                        SNAFUDigit::Zero =>         SNAFUDigitResult::new(vec![SNAFUDigit::Two] , false),
                        SNAFUDigit::Minus =>        SNAFUDigitResult::new(vec![SNAFUDigit::One] , false),
                        SNAFUDigit::DoubleMinus =>  SNAFUDigitResult::new(vec![SNAFUDigit::Zero] , false),
                    }
                },
                SNAFUDigit::One => {
                    match rhs{
                        SNAFUDigit::Two =>          SNAFUDigitResult::new(vec![SNAFUDigit::One, SNAFUDigit::DoubleMinus] , false),
                        SNAFUDigit::One =>          SNAFUDigitResult::new(vec![SNAFUDigit::Two] , false),
                        SNAFUDigit::Zero =>         SNAFUDigitResult::new(vec![SNAFUDigit::One] , false),
                        SNAFUDigit::Minus =>        SNAFUDigitResult::new(vec![SNAFUDigit::Zero] , false),
                        SNAFUDigit::DoubleMinus =>  SNAFUDigitResult::new(vec![SNAFUDigit::One,SNAFUDigit::Minus] , true),
                    }
                },
                SNAFUDigit::Zero => {
                    match rhs{
                        SNAFUDigit::Two =>          SNAFUDigitResult::new(vec![SNAFUDigit::Two] , false),
                        SNAFUDigit::One =>          SNAFUDigitResult::new(vec![SNAFUDigit::One] , false),
                        SNAFUDigit::Zero =>         SNAFUDigitResult::new(vec![SNAFUDigit::Zero] , false),
                        SNAFUDigit::Minus =>        SNAFUDigitResult::new(vec![SNAFUDigit::Minus] , false),
                        SNAFUDigit::DoubleMinus =>  SNAFUDigitResult::new(vec![SNAFUDigit::DoubleMinus] , false),
                    }
                },
                SNAFUDigit::Minus => {
                    match rhs{
                        SNAFUDigit::Two =>          SNAFUDigitResult::new(vec![SNAFUDigit::One] , false),
                        SNAFUDigit::One =>          SNAFUDigitResult::new(vec![SNAFUDigit::Zero] , false),
                        SNAFUDigit::Zero =>         SNAFUDigitResult::new(vec![SNAFUDigit::Minus] , false),
                        SNAFUDigit::Minus =>        SNAFUDigitResult::new(vec![SNAFUDigit::DoubleMinus] , false),
                        SNAFUDigit::DoubleMinus =>  SNAFUDigitResult::new(vec![SNAFUDigit::Two] , true),
                    }
                },
                SNAFUDigit::DoubleMinus => {
                    match rhs{
                        SNAFUDigit::Two =>          SNAFUDigitResult::new(vec![SNAFUDigit::Zero] , false),
                        SNAFUDigit::One =>          SNAFUDigitResult::new(vec![SNAFUDigit::Minus] , false),
                        SNAFUDigit::Zero =>         SNAFUDigitResult::new(vec![SNAFUDigit::DoubleMinus] , false),
                        SNAFUDigit::Minus =>        SNAFUDigitResult::new(vec![SNAFUDigit::Two] , true),
                        SNAFUDigit::DoubleMinus =>  SNAFUDigitResult::new(vec![SNAFUDigit::One] , true),
                    }
                },
            }
        }
    }
    impl SNAFUDigit{
        fn abs(&self) -> SNAFUDigit{
            match self{
                SNAFUDigit::Zero | SNAFUDigit::One | SNAFUDigit::Two => self.clone(),
                SNAFUDigit::Minus => SNAFUDigit::One,
                SNAFUDigit::DoubleMinus => SNAFUDigit::Two,
            }
        }
        fn new_from_snafu_str(c:&str) -> Option<Self>{
            match c.trim(){
                "="=> Some(SNAFUDigit::DoubleMinus),
                "-" => Some(SNAFUDigit::Minus),
                "0" => Some(SNAFUDigit::Zero),
                "1" => Some(SNAFUDigit::One),
                "2" => Some(SNAFUDigit::Two),
                _ => {
                    // println!("Invalic letter: Not valid SNAFU! |{c}|");
                    None
                },
            }
        }
        fn new_from_digit_char(c:char) -> Vec<Self>{
            match c{
                '0' => vec![SNAFUDigit::Zero],
                '1' => vec![SNAFUDigit::One],
                '2' => vec![SNAFUDigit::Two],
                '3' => vec![SNAFUDigit::One, SNAFUDigit::DoubleMinus],
                '4' => vec![SNAFUDigit::One, SNAFUDigit::Minus],
                '5' => vec![SNAFUDigit::One, SNAFUDigit::Zero],
                '6' => vec![SNAFUDigit::One, SNAFUDigit::One],
                '7' => vec![SNAFUDigit::One, SNAFUDigit::Two],
                '8' => vec![SNAFUDigit::Two, SNAFUDigit::DoubleMinus],
                '9' => vec![SNAFUDigit::Two, SNAFUDigit::Minus],
                _ => panic!("Invalic Char: ot a number! {c}"),
            }
        }
        fn to_decimal(&self,location_idx:usize) -> i128{
            let location_value = 5i128.pow(location_idx as u32);
            match &self{
                SNAFUDigit::Zero => 0,
                SNAFUDigit::One => location_value,
                SNAFUDigit::Two => 2*location_value,
                SNAFUDigit::Minus => -location_value,
                SNAFUDigit::DoubleMinus => -2*location_value,
            }
        }
    }

    fn get_data(data_string:&str) -> Vec<SNAFU>{
        data_string.lines().map(|line|{
            SNAFU::new(line)
        }).collect::<Vec<SNAFU>>()
    }
    pub fn main_1(file_name:&str)->Option<String>{
        let data_string = read_to_string(file_name).unwrap();
        let snafus = get_data(&data_string);
        let digits: Vec<i128> = snafus.iter().map(|snafu|{snafu.to_decimal()}).collect::<Vec<i128>>();
        let digits_sum = digits.iter().fold(0, |acc, val| acc+val);
        let sum_as_snafu = SNAFU::new_from_num(digits_sum).expect("Failed, but should not have failed!");
        let snafu_string = sum_as_snafu.snafu_to_string();
        Some(snafu_string)
    }


#[cfg(test)]
mod tests{
    use super::*;
	
	#[test]
	fn snafu_digit_test(){
        let pairs = vec![("1=-0-2", 1747),
            ("12111"    , 906),
            ("2=0="     , 198),
            ("21"       , 11),
            ("2=01"     , 201),
            ("111"      , 31),
            ("20012"    , 1257),
            ("112"      , 32),
            ("1=-1="    , 353),
            ("1-12"     , 107),
            ("12"       , 7),
            ("1="       , 3),
            ("122"      , 37)
        ];
        for (line, digit) in pairs.iter() {
            println!("{} \t== {}", digit, line);
            let snafu = SNAFU::new(line);
            println!("{} \t-> {}", line, snafu.snafu_to_string());
            let snafu_digit = snafu.to_decimal();
            println!("{} \t-> {}",snafu.snafu_to_string(), snafu_digit);
            let reborn_snafu = SNAFU::new_from_num(snafu_digit).expect("Failed, but should not have failed!");
            let reborn_line = reborn_snafu.snafu_to_string();
            println!("{} \t-> {}",snafu_digit, reborn_line);
            assert_eq!(snafu_digit, *digit, "SNAFU to digit failed! Got {snafu_digit}, expected {digit}");
            assert_eq!(&reborn_line, line, "Digit to SNAFU failed! Got {reborn_line}, expected {line}");
            println!("____________________________");
        }
	}
	#[test]
	fn other_snafu_digit_test(){
        let pairs = vec![
            (0        ,"0"),
            (1        ,"1"),
            (2        ,"2"),
            (3        ,"1="),
            (4        ,"1-"),
            (5        ,"10"),
            (6        ,"11"),
            (7        ,"12"),
            (8        ,"2="),
            (9        ,"2-"),
            (10       ,"20"),
            (15       ,"1=0"),
            (20       ,"1-0"),
            (2022     ,"1=11-2"),
            (12345    ,"1-0---0"),
            (314159265,"1121-1110-1=0")
        ];
        for (digit, line) in pairs.iter() {
            let snafu = SNAFU::new(line);
            let snafu_digit = snafu.to_decimal();
            let reborn_snafu = SNAFU::new_from_num(snafu_digit).expect("Failed, but should not have failed!");
            let reborn_line = reborn_snafu.snafu_to_string();
            println!("{} \t== {}", digit, line);
            println!("{} \t-> {}", line, snafu.snafu_to_string());
            println!("{} \t-> {}",snafu.snafu_to_string(), snafu_digit);
            println!("{} \t-> {}",snafu_digit, reborn_line);
            assert_eq!(snafu_digit, *digit, "SNAFU to digit failed! Got {snafu_digit}, expected {digit}");
            assert_eq!(&reborn_line, line, "Digit to SNAFU failed! Got {reborn_line}, expected {line}");
            println!("____________________________");
        }
	}
    #[test]
    fn compare_test(){
        assert!(SNAFUDigit::Zero > SNAFUDigit::Minus);
        assert!(SNAFUDigit::One > SNAFUDigit::DoubleMinus);
        assert!(SNAFUDigit::One == SNAFUDigit::One);
        assert!(SNAFUDigit::DoubleMinus < SNAFUDigit::Minus);
        assert!(! (SNAFUDigit::Two < SNAFUDigit::Zero))
    }


}

}
mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i128>{
      None
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
        let expected_value = "2=-1=0".to_string();
        let actual_value = count.unwrap();
        assert_eq!(actual_value, expected_value, "Wrong answer! Got \"{actual_value}\", expected \"{expected_value}\"");
    }

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}

