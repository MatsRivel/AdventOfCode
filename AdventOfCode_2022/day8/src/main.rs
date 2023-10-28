mod part1{
    use std::fs::read_to_string;

    pub fn get_grid(file_name:&str)->Vec<Vec<u32>>{
        let grid_string = read_to_string(file_name).expect("Failed to read file!\n");
        let trees = grid_string.lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).expect("Failed to convert char to u32")).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        trees
    }
    fn highest_from_left(grid:&Vec<Vec<u32>>)->Vec<Vec<u32>>{
        grid.iter()
        .map(|inner| {
            // Output the highest element from 0 to current idx, left to right.
            inner.iter()
                .enumerate()
                .map(|(idx, _)|{
                    match inner[..idx].iter().max(){
                        Some(v) => *v,
                        None => 0u32,
                    }
                }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>()
    }
    fn highest_from_right(grid:&Vec<Vec<u32>>)->Vec<Vec<u32>>{
        grid.iter()
        .map(|inner| {
            // Output the highest element from n to current idx, right to left.
            inner.iter()
                .enumerate()
                .map(|(idx, _)|{
                    match inner[(idx+1)..].iter().max(){
                        Some(v) => *v,
                        None => 0u32,
                    }
                }).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>()
    }
    fn highest_from_top(grid:&Vec<Vec<u32>>)->Vec<Vec<u32>>{
        let mut max_vec:Vec<Vec<u32>> = Vec::with_capacity(grid[0].len());
        for j in 0..grid[0].len(){
            let inner_vec = grid.iter().map(|_| 0).collect::<Vec<u32>>();
            max_vec.push(inner_vec);
        }
        //NOTE: Only works when the inner vecs have the same number of elements as all other inner vecs,
        //      and the outer vecs have the same number of elements as the other outer vecs.
        for (i, _) in grid[0].iter().enumerate(){
            for (j, _) in grid.iter().enumerate(){
                let vertical_vec = grid[..j].iter()
                    .map(|inner|{inner[i]});
                max_vec[j][i] = match vertical_vec.max(){
                    Some(v) => v,
                    None => 0u32,
                }
            }
        }
        max_vec
    }
    fn highest_from_bottom(grid:&Vec<Vec<u32>>)->Vec<Vec<u32>>{
        let mut max_vec:Vec<Vec<u32>> = Vec::with_capacity(grid[0].len());
        for j in 0..grid[0].len(){
            let inner_vec = grid.iter().map(|_| 0).collect::<Vec<u32>>();
            max_vec.push(inner_vec);
        }
        //NOTE: Only works when the inner vecs have the same number of elements as all other inner vecs,
        //      and the outer vecs have the same number of elements as the other outer vecs.
        for (i, _) in grid[0].iter().enumerate(){
            for (j, _) in grid.iter().enumerate(){
                let vertical_vec = grid[(j+1)..].iter()
                    .map(|inner|{inner[i]});
                max_vec[j][i] = match vertical_vec.max(){
                    Some(v) => v,
                    None => 0u32,
                }
            }
        }
        max_vec
    }
    pub fn print_grid(grid:&Vec<Vec<u32>>){
        grid.iter().for_each(|inner| {
            inner.iter().for_each(|x| print!("{x} "));
            println!()});
    }
    fn count_visible(grid:Vec<Vec<u32>>) -> u32{
        let left_grid:Vec<Vec<u32>> = highest_from_left(&grid);
        let right_grid:Vec<Vec<u32>> = highest_from_right(&grid);
        let top_grid:Vec<Vec<u32>> = highest_from_top(&grid);
        let bottom_grid:Vec<Vec<u32>> = highest_from_bottom(&grid);
        // All edge-trees are visible
        let mut visible_counter = (2* (grid.len() + grid[0].len()) -4) as u32;
        for i in 1..(grid.len()-1){
            for j in 1..(grid[i].len()-1){
                let current_height = grid[i][j];
                // If there are any trees from any direction shorter than the current,
                // then the current is visible.
                if  left_grid[i][j] < current_height ||
                    right_grid[i][j] < current_height ||
                    top_grid[i][j] < current_height ||
                    bottom_grid[i][j] < current_height{
                        visible_counter += 1;
                }
            }
        }
        visible_counter
    }
    pub fn main_1(file_name:&str){
        let grid = get_grid(file_name);
        let visible_count = count_visible(grid);
		println!("Part 1: {}",visible_count);
    }
}
mod part2{
    use std::fs::read_to_string;
    use crate::part1::{get_grid, print_grid};

    pub fn main_2(file_name:&str){
        let mut best_score = 0;
        // let mut best_idx = [0u32,0u32];
		let grid = get_grid(file_name);
        let mut score_grid = grid.iter().map(|inner| inner.iter().map(|_| 0u32).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        for x in 1..(grid.len()-1){
            for y in 1..(grid[x].len()-1){
                let current_tree = grid[x][y];
                let mut left_score = 0u32;
                let mut right_score = 0u32;
                let mut top_score = 0u32;
                let mut bottom_score = 0u32;
                for other_tree in grid[x][..y].iter().rev(){
                    if other_tree < &current_tree{
                        left_score += 1;
                    }else if other_tree == &current_tree{
                        left_score += 1;
                        break;
                    }else{
                        break;
                    }
                }
                for other_tree in grid[x][(y+1)..].iter(){
                    if [x,y] == [3,2]{
                    }
                    if other_tree < &current_tree{
                        right_score += 1;
                    }else{
                        right_score += 1;
                        break;
                    }
                }
                for temp_grid in grid[..x].iter().rev(){
                    let other_tree = &temp_grid[y];
                    if other_tree < &current_tree{
                        top_score += 1;
                    }else if other_tree == &current_tree{
                        top_score += 1;
                        break;
                    }else{
                        break;
                    }
                }
                for temp_grid in grid[(x+1)..].iter(){
                    let other_tree = &temp_grid[y];
                    if other_tree < &current_tree{
                        bottom_score += 1;
                    }else if other_tree == &current_tree{
                        bottom_score += 1;
                        break;
                    }else{
                        break;
                    }
                }
                let total_score = left_score * right_score * top_score * bottom_score;
                // if total_score > 0{
                //     println!("\n[{x},{y}] -> {}",grid[x][y]);
                //     println!("{} * {} * {} * {} = {}", left_score, right_score, top_score, bottom_score, total_score);
                // }
                score_grid[x][y] = total_score;
                if best_score < total_score{
                    best_score = total_score;
                    // best_idx = [x as u32,y as u32];
                    //println!("[{x},{y}] -> {}\n", grid[x][y]);
                }
            }
        }
        // println!();
        // print_grid(&score_grid);
		println!("Part 2: {}",best_score);
    }
}

use part1::main_1;
use part2::main_2;
fn main() {
    let file_name = r"src\dummy_input.txt";
    let file_name = r"src\puzzle_input.txt";
    main_1(file_name);
    main_2(file_name);
}
