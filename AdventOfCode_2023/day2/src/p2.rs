use std::fs::read_to_string;
struct Game{
    max_red: u32,
    max_green: u32,
    max_blue: u32
}
impl Game{
    fn new(rounds:Vec<Round>)->Self{
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in rounds{
            max_red = std::cmp::max(round.red, max_red);
            max_green = std::cmp::max(round.green, max_green);
            max_blue = std::cmp::max(round.blue, max_blue);
        }
        Game{max_red, max_green, max_blue}
    }
}
struct Round{
    red: u32,
    green: u32,
    blue: u32
}
enum Cube{
    Red,
    Green,
    Blue
}
impl Cube{
    fn new(cube:&str)->Self{
        match cube{
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            _ => panic!()
        }
    }
}
fn process_data_string(data_string:&str)->Vec<Game>{
    data_string
        .lines()
        .map(|line| {
            line.split(": ").collect::<Vec<&str>>()[1]
        })
        .map(|game_line| {
            let rounds = game_line
                .split("; ")
                .map(|section| {
                    let cubes_found = section
                        .split(", ")
                        .map(|pair|{
                            let cubes = TryInto::<[&str;2]>::try_into(pair.split(" ").collect::<Vec<&str>>()).unwrap();
                            let cube_count = cubes[0].parse::<u32>().unwrap();
                            let cube_type = Cube::new(cubes[1]);
                            (cube_count,cube_type)
                        });

                    let mut red_count = 0;
                    let mut blue_count = 0;
                    let mut green_count = 0;
                    for (count, ctype) in cubes_found{
                        match ctype{
                            Cube::Red=> red_count = count,
                            Cube::Blue => blue_count = count,
                            Cube::Green => green_count = count
                        }
                    }
                    Round{red:red_count, blue:blue_count, green:green_count}
                }).collect::<Vec<Round>>();
            Game::new(rounds)
        }).collect::<Vec<Game>>()
}

pub fn main_2(file_name:&str)->Option<u32>{
    let data_string = read_to_string(file_name).unwrap();
    let rounds = process_data_string(&data_string);
    let output = rounds.iter().map(|game| {
       game.max_red*game.max_blue*game.max_green
    }).fold(0, |acc,v| acc+v);
    Some(output)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
