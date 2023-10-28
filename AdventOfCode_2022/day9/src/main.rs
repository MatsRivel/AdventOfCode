mod part1{
    use std::{fs::read_to_string, collections::HashMap};
    #[derive(Debug)]
    enum Move{
        R(i32),
        U(i32),
        L(i32),
        D(i32),
    }
    fn get_moves(file_name:&str) -> Vec<Move>{
        read_to_string(file_name)
            .expect("Failed to read file!\n")
            .lines()
            .map(|line| {
                let val = line[2..].parse::<i32>().expect("Failed to parse step-length!\n");
                match &line[0..1]{
                    "R" => Move::R(val),
                    "U" => Move::U(val),
                    "L" => Move::L(val),
                    "D" => Move::D(val),
                    _ => panic!("Unexpected character ({}) in the \"move\" position!\n",&line[0..0])
                }
            }).collect::<Vec<Move>>()
    }
    fn get_pos_diff(pos_a:&[i32;2], pos_b:&[i32;2]) -> [i32;2] {
        [pos_a[0] - pos_b[0], pos_a[1] - pos_b[1]]
    }
    fn next_move(h_pos: &[i32;2], t_pos: &[i32;2]) -> Option<[i32;2]>{
        let diff = get_pos_diff(&h_pos, &t_pos);
        match diff{
            [-1..=1, -1..=1] => None,
            [_,_] => Some(diff)
        }
    }
    fn perform_move(pos:&[i32;2], dir: &[i32;2]) -> [i32;2]{
        [pos[0] + dir[0], pos[1] + dir[1]]
    }
    fn perform_step(pos:&[i32;2], dir: &[i32;2]) -> [i32;2]{
        let mut normalized_dir:[i32;2] = match *dir{
            [0,0] => [0,0],
            [v,0]=> {
                let norm_val = v / v.abs();
                [norm_val,0]
            },
            [_,v] => {
                let norm_val = v / v.abs();
                [0,norm_val]
            }

        };
        [pos[0] + normalized_dir[0], pos[1] + normalized_dir[1]]
    }
    fn perform_head_move(h_pos:[i32;2], m: &Move) -> [i32;2]{
        match m{
            Move::R(_) => perform_move(&h_pos, &[1,0]),
            Move::U(_) => perform_move(&h_pos, &[0,-1]),
            Move::L(_) => perform_move(&h_pos, &[-1,0]),
            Move::D(_) => perform_move(&h_pos, &[0,1]),
        }
    }
    fn get_move_dist(m: &Move) -> i32{
        match m{
            Move::R(v) => *v,
            Move::U(v) => *v,
            Move::L(v) => *v,
            Move::D(v) => *v
        }
    }
    fn print_area(h_pos: &[i32;2], t_pos: &[i32;2], last_t_pos: &[i32;2], move_log: &HashMap<[i32;2],i32>){
        const GRID_SIZE: usize = 7usize;
        const HALF_SIZE: usize = GRID_SIZE/2usize;
        const EMPTY_TILE: char = '.';
        const SEEN_TILE: char = ' ';
        let center = t_pos;
        // Generate an empty grid.
        let mut grid = (0..GRID_SIZE).map(|_| (0..GRID_SIZE).map(|_| EMPTY_TILE).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        
        // Using a log of all visited positions:
        // - Filter away all positions not withing viewing distance of the center.
        // - Adjust their positions to be relative to the center.
        // - Collect their adjusted positions.
        let visited = move_log.iter()
            .filter(|(pos,_)| {
                let diff = get_pos_diff(pos, &center);
                diff[0].abs() < HALF_SIZE as i32 && diff[1].abs() < HALF_SIZE as i32
            })
            .map(|(pos,_)| {
                let diff = get_pos_diff(&pos, &center);
                perform_move(&[HALF_SIZE as i32, HALF_SIZE as i32], &diff)
            })
            .collect::<Vec<[i32;2]>>();
        
        // Plot all adjusted visited positions
        for vis in visited.iter(){
            //println!("Vis: {vis:?}");
            grid[vis[0] as usize][vis[1] as usize] = SEEN_TILE;
        }

        // Position "H" relative to "T".
        let t_t_diff = get_pos_diff(last_t_pos, &center);
        let adj_pos = [(HALF_SIZE as i32 + t_t_diff[0]) as usize, (HALF_SIZE as i32 + t_t_diff[1]) as usize];
        grid[adj_pos[0]][adj_pos[1]] = 't';

        //grid[HALF_SIZE][HALF_SIZE] = 'T'; // Put "T" in the center.
        // Position "T" relative to center.
        let t_diff = get_pos_diff(&t_pos, &center);
        let adj_pos = [(HALF_SIZE as i32 + t_diff[0]) as usize, (HALF_SIZE as i32 + t_diff[1]) as usize];
        grid[adj_pos[0]][adj_pos[1]] = 'T';
        
        // Position "H" relative to "T".
        let h_t_diff = get_pos_diff(h_pos, &center);
        let adj_pos = [(h_t_diff[0] + HALF_SIZE as i32) as usize, (HALF_SIZE as i32 + h_t_diff[1]) as usize];
        grid[adj_pos[0]][adj_pos[1]] = 'H';

        // Print the board-
        grid.iter().for_each(|row| {
            println!();
            row.iter().for_each(|v| print!("{v} "))
        });
        println!();
    }

    pub fn main_1(file_name:&str){
        let moves = get_moves(file_name);
        let mut h_pos = [0i32;2];
        let mut t_pos = [0i32;2];
        let mut last_t_pos = [0i32;2];
        let mut move_log = HashMap::new();
        let mut h_move_log = HashMap::new();
        move_log.insert(t_pos, 1);
        h_move_log.insert(h_pos, 1);
        for m in moves{
            println!("\n{:?}",m);
            let dist = get_move_dist(&m);
            for _step in 0..dist{
                last_t_pos = t_pos;
                h_pos = perform_head_move(h_pos, &m);
                h_move_log.insert(h_pos, 1);
                // println!("h");
                println!("Distance: {:?}",get_pos_diff(&h_pos, &t_pos));
                if let Some(t_move) = next_move(&h_pos, &t_pos){
                    t_pos = perform_step(&t_pos,&t_move);
                    move_log.insert(t_pos, 1);
                    // println!("t");
                }
                // print_area(&h_pos, &t_pos, &last_t_pos, &move_log); 

            }
        }
        let n_unique_spots = move_log.iter()
            .map(|(_key,_)| {
                println!("{_key:?}");
                1
            }).sum::<i32>(); 
        println!("Part 1: {n_unique_spots}");
        let h_spots = h_move_log.iter()
            .map(|(_key,_)| {
                // println!("{_key:?}");
                1
            }).sum::<i32>(); 
        println!("And {h_spots} h_pos'");
    }
}
mod part2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str){
		// let output = todo!();
		// println!("Part 2: {}",output);
    }
}

use part1::main_1;
use part2::main_2;
fn main() {
    let file_name = r"src\dummy_input.txt";
    //let file_name = r"src\puzzle_input.txt";
    main_1(file_name);
    main_2(file_name);
}
