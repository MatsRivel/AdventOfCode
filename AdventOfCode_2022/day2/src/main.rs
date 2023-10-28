use std::str::Lines;
use PartOne::main_part_one;
use PartTwo::main_part_two;
mod PartOne{
    pub enum Play{
        Rock,
        Paper,
        Scissors,
    }
    impl Play{
        pub fn new(play:&str) -> Self{
            match play{
                "Y"|"B"=> Self::Paper,
                "X"|"A" => Self::Rock,
                "Z"|"C" => Self::Scissors,
                _ => panic!("Invalid play by you!")
            }
        }
    }

    fn get_plays(line:&str) -> (Play, Play) {
        //Returns (opponents play,your play).
        let text = line.split(" ").collect::<Vec<&str>>();
        let text_arr:[&str;2] = text.try_into().unwrap();
        (Play::new(text_arr[0]),Play::new(text_arr[1]))
    }


    pub fn get_score((they_play, you_play): (Play,Play)) -> u32 {
        let (rock,paper,scissors) = (1u32,2u32,3u32);
        let (win,draw,loose) = (6u32,3u32,0u32);
        match (you_play, they_play) {
            (Play::Rock,Play::Scissors) => rock+win,
            (Play::Rock,Play::Rock) => rock+draw,
            (Play::Rock,Play::Paper) => rock+loose,

            (Play::Paper,Play::Scissors) => paper+loose,
            (Play::Paper,Play::Rock) => paper+win,
            (Play::Paper,Play::Paper) => paper+draw,
            
            (Play::Scissors,Play::Scissors) => scissors+draw,
            (Play::Scissors,Play::Rock) => scissors+loose,
            (Play::Scissors,Play::Paper) => scissors+win,

        }
    }

    pub fn main_part_one(){
        let file_name = r"C:\Users\Mats\OneDrive - University of Bergen\Documents\Rust Stuff\AdventOfCode2022\Day2\src\puzzle_input.txt";
        let score:u32 = std::fs::read_to_string(file_name)
            .expect("Failed to read file")
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&line| get_score( get_plays(line) ) )
            .sum();
        println!("Score: {score}")
    }
}

mod PartTwo{
    use crate::PartOne::{get_score,Play};
    #[derive(Debug)]
    enum Outcome{
        Win,
        Loose,
        Draw    
    }
    impl Outcome{
        fn new(play:&str) -> Self{
            match play{
                "X"=> Self::Loose,
                "Y" => Self::Draw,
                "Z" => Self::Win,
                _ => panic!("Invalid play by You!")
            }
        }
    }

    fn tactical_new(they_played:&Play, play:&str) -> Play{
        let your_outcome = Outcome::new(play);
        match (they_played, your_outcome) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Loose) => Play::Scissors,
            (Play::Rock, Outcome::Draw) => Play::Rock,

            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Loose) => Play::Rock,
            (Play::Paper, Outcome::Draw) => Play::Paper,

            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Loose) => Play::Paper,
            (Play::Scissors, Outcome::Draw) => Play::Scissors,
        }
    }

    fn get_plays(line: &str) -> (Play, Play){
        let text = line.split(" ").collect::<Vec<&str>>();
        let text_arr:[&str;2] = text.try_into().unwrap();
        let they_play = Play::new(text_arr[0]);
        let you_play = tactical_new(&they_play,text_arr[1]);
        (they_play,you_play)
    }

    pub fn main_part_two(){
        let file_name = r"C:\Users\Mats\OneDrive - University of Bergen\Documents\Rust Stuff\AdventOfCode2022\Day2\src\puzzle_input.txt";
        let score:u32 = std::fs::read_to_string(file_name)
            .expect("Failed to read file")
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&line| get_score( get_plays(line) ) )
            .sum();
        println!("Part one:\n\tScore: {score}")
    }
}
fn main() {
    //main_part_one();
    main_part_two();
}
