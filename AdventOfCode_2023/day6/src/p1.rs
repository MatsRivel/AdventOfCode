use std::fs::read_to_string;

pub struct Race{
    pub time: u128,
    pub dist: u128
}
pub fn get_reaces(data_string:String)->Vec<Race>{
    let [time_str, distance_str]: [&str;2] = data_string.lines().collect::<Vec<&str>>().try_into().unwrap();
    let times = time_str.split_whitespace().skip(1).map(|s| s.parse::<u128>().unwrap() );
    let distances = distance_str.split_whitespace().skip(1).map(|s| s.parse::<u128>().unwrap() );
    let races = times.zip(distances).map(|(t,d)| Race{ time:t, dist:d } ).collect::<Vec<Race>>();
    races
}

pub fn dist(time_held:u128, total_time:u128)->u128{
    if time_held > total_time{
        return 0;
    }
    (total_time-time_held)*time_held
}
pub fn ways_to_win_race(race:&Race)->u128{
    (1..race.time).filter(|n| dist(*n, race.time) > race.dist).count() as u128
}
pub fn main_1(file_name:&str)->Option<u128>{
    let data_string = read_to_string(file_name).unwrap();
    let races = get_reaces(data_string);
    let ways_to_win_each_race = races.into_iter().map(|r| ways_to_win_race(&r));
    let output = ways_to_win_each_race.product();
    Some(output)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
