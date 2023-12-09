use std::fs::read_to_string;

pub fn process_data_string(data_string:String)-> (Vec<u64>,Vec<Vec<[u64;3]>>){
    let mut output = vec![Vec::<[u64;3]>::new();7];
    let mut temp_vec = vec![];
    let mut idx = 0;
    let mut line_iter = data_string.lines();
    let initial_line = line_iter.next().unwrap();
    let seeds = initial_line.split(": ").nth(1).unwrap().split(" ").map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    line_iter.next(); // Skipping the first empty line.
    while let Some(line) = line_iter.next() {
        if line ==""{
            // println!("{temp_vec:?}");
            output[idx] = temp_vec;
            idx += 1;
            temp_vec = vec![];
        }else if line.chars().nth(0).is_some() && line.chars().nth(0).unwrap().is_alphabetic(){
                continue; // We skip the name-line of any section.
        }else{
            let range_nums: [u64;3] = line.split(" ").map(|num_str| num_str.parse::<u64>().unwrap() ).collect::<Vec<u64>>().try_into().unwrap(); 
            temp_vec.push(range_nums);
        }
    }
    output[idx] = temp_vec;
    (seeds,output)


}
pub struct Almanac{
    pub seed_to_soil: Vec<[u64;3]>,
    pub soil_to_fert: Vec<[u64;3]>,
    pub fert_to_water: Vec<[u64;3]>,
    pub water_to_light: Vec<[u64;3]>,
    pub light_to_temp: Vec<[u64;3]>,
    pub temp_to_humid: Vec<[u64;3]>,
    pub humid_to_loc: Vec<[u64;3]>,
}
impl Almanac{
    pub fn new(mut data: Vec<Vec<[u64;3]>>)->Self{
        let humid_to_loc    = data.pop().unwrap();
        let temp_to_humid   = data.pop().unwrap();
        let light_to_temp   = data.pop().unwrap();
        let water_to_light  = data.pop().unwrap();
        let fert_to_water   = data.pop().unwrap();
        let soil_to_fert    = data.pop().unwrap();
        let seed_to_soil    = data.pop().unwrap();
        Self { seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humid, humid_to_loc }
    }
    fn id_in_ranges(id:u64, range: &Vec<[u64;3]>)->u64{
        for [right, left, length] in range.iter(){
            if left <= &id && id <left + length{
                return id - left + right ; 
            }
        }
        return id;
    }


    fn seed_to_soil(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.seed_to_soil)
    }
    fn soil_to_fert(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.soil_to_fert)
    }
    fn fert_to_water(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.fert_to_water)
    }
    fn water_to_light(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.water_to_light)
    }
    fn light_to_temp(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.light_to_temp)
    }
    fn temp_to_humid(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.temp_to_humid)
    }
    fn humid_to_loc(&self,id:u64)->u64{
        Almanac::id_in_ranges(id, &self.humid_to_loc)
    }
    pub fn seed_to_loc(&self,seed:u64)->u64{
        let soil    = self.seed_to_soil(seed);
        let fert    = self.soil_to_fert(soil);
        let water   = self.fert_to_water(fert);
        let light   = self.water_to_light(water);
        let temp    = self.light_to_temp(light);
        let humid   = self.temp_to_humid(temp);
        let loc     = self.humid_to_loc(humid);
        loc
    }
}

pub fn main_1(file_name:&str)->Option<u64>{
    let data_string = read_to_string(file_name).unwrap();
    let (seeds, data) = process_data_string(data_string);
    let almanac = Almanac::new(data);
    let locations = seeds.iter().map(|seed| almanac.seed_to_loc(*seed)).collect::<Vec<u64>>();
    let min_loc = locations.iter().fold(u64::MAX, |acc,val| std::cmp::min(acc,*val));
    Some(min_loc)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn transform_testing(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        let (seeds, data) = process_data_string(data_string);
        let almanac = Almanac::new(data);
        assert_eq!( almanac.seed_to_soil(98),50 );
        assert_eq!( almanac.seed_to_soil(99),51 );
    }

}
