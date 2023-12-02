use std::{fs::read_to_string, collections::VecDeque};
#[derive(Debug,Clone)]
struct Crab{
    rot: VecDeque<u8>,
    mem: VecDeque<u8>,
}
impl Crab{
    fn new(s:&str) -> Self{
        let rot = s.chars().filter_map(|c| c.to_digit(10)).map(|n| n as u8).collect::<VecDeque<u8>>();
        let mem = VecDeque::<u8>::new();
        Self{rot,mem}
    }
    fn cycle_clockwise(&mut self){
        let v = self.rot.pop_front().unwrap();
        self.rot.push_back(v)
    }
    fn cycle_counter_clockwise(&mut self){
        let v = self.rot.pop_back().unwrap();
        self.rot.push_front(v)
    }
    fn pick_up_3(&mut self){
        self.cycle_clockwise();
        for _ in 0..3{
            self.mem.push_back( self.rot.pop_front().unwrap() )
        }
        self.cycle_counter_clockwise()
    }
    fn get_current(&self) -> u8{
        *self.rot.get(0).unwrap()
    }
    fn insert_mem_at_val(&mut self, target_val: u8){
        let current_val = self.get_current();
        #[cfg(test)]
        let mut counter = 0;
        while self.get_current() != target_val {
            #[cfg(test)]{
                counter += 1;
                if counter > 10{
                    println!("target_val: {target_val}");
                    print!("rot: ");
                    for v in self.rot.iter(){
                        print!("{v}");
                    }
                    println!();
                    print!("mem: ");
                    for v in self.mem.iter(){
                        print!("{v}");
                    }
                    println!();
                    panic!();
                }
            }
            self.cycle_clockwise()
        }
        self.cycle_clockwise();
        for _ in 0..3{
            self.rot.push_front( self.mem.pop_back().unwrap() );
        }
        #[cfg(test)]
        let mut counter = 0;
        while self.get_current() != current_val{
            #[cfg(test)]{
                counter += 1;
                if counter > 10{
                    panic!();
                }
            }
            self.cycle_counter_clockwise(); // Rotate back to put the correct val first.
        }
        
    }
    fn get_destination(&self)->u8{
        let current = self.get_current();
        let mut destination = match current{
            1 => 9,
            v => v-1
        };
        #[cfg(test)]
        let mut counter = 0;
        while self.mem.contains(&destination){
            #[cfg(test)]{
                counter += 1;
                if counter > 10{
                    panic!();
                }
            }
            destination = match destination{
                1 => 9,
                v => v-1
            };
        }
        return destination
    }
    #[cfg(test)]
    fn get_final_order(&self)->u32{
        // Temp solution to make sure we retain order during testing.
        let mut other = self.clone();
        #[cfg(test)]
        let mut counter = 0;
        while other.get_current() != 1{
            #[cfg(test)]{
                counter += 1;
                if counter > 10{
                    panic!();
                }
            }
            other.cycle_clockwise()
        }
        other.rot.iter().skip(1).map(|n| format!("{n}")).collect::<String>().parse::<u32>().unwrap()
    }
    #[cfg(not(test))]
    fn get_final_order(&mut self)->u32{
        // Temp solution to make sure we retain order during testing.
        #[cfg(test)]
        let mut counter = 0;
        while self.get_current() != 1{
            #[cfg(test)]{
                counter += 1;
                if counter > 10{
                    panic!();
                }
            }
            self.cycle_clockwise()
        }
        self.rot.iter().skip(1).map(|n| format!("{n}")).collect::<String>().parse::<u32>().unwrap()
    }

}

pub fn main_1(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut crab = Crab::new(&data_string);
    #[cfg(test)]
    assert_eq!(crab.get_final_order(),25467389, "Failed after 0 iter");
    #[cfg(test)]
    println!("{:?} | {:?}",crab.rot,crab.mem);
    for _i in 1..=100{
        crab.pick_up_3();
        let desitnation  = crab.get_destination();
        crab.insert_mem_at_val(desitnation);

        #[cfg(test)]{
            match _i{
                1 => assert_eq!(crab.get_final_order(),54673289, "Failed after {_i} iter"),
                2 => assert_eq!(crab.get_final_order(),32546789, "Failed after {_i} iter"),
                3 => assert_eq!(crab.get_final_order(),34672589, "Failed after {_i} iter"),
                4 => assert_eq!(crab.get_final_order(),32584679, "Failed after {_i} iter"),
                5 => assert_eq!(crab.get_final_order(),36792584, "Failed after {_i} iter"),
                6 => assert_eq!(crab.get_final_order(),93672584, "Failed after {_i} iter"),
                7 => assert_eq!(crab.get_final_order(),92583674, "Failed after {_i} iter"),
                8 => assert_eq!(crab.get_final_order(),58392674, "Failed after {_i} iter"),
                9 => assert_eq!(crab.get_final_order(),83926574, "Failed after {_i} iter"),
                _ => ()
            }
        }
        crab.cycle_clockwise();

    }
    return Some(crab.get_final_order());

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
