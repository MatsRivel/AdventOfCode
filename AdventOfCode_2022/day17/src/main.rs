// Rock Shapes:

// ####

//  # 
// ###
//  # 

//   #
//   #
// ###

// #
// #
// #
// #

// ##
// ##



mod p1{
    use std::{fs::read_to_string, cmp::max};
    const LOWER_HORIZONTAL_LIMIT:i32 = 0i32;
    const UPPER_HORIZONTAL_LIMIT:i32 = 6i32;
    const MAP_WIDTH:usize = (UPPER_HORIZONTAL_LIMIT-LOWER_HORIZONTAL_LIMIT +1) as usize;
    const N_ITERATIONS:i32 = 2022i32;
    // const N_ITERATIONS:i32 = 1i32;
    #[derive(Debug,Clone)]
    pub enum Rock{// The nested arrays represent an array of lines that make the shape of each rock shape.
        Minus,
        Plus,
        L,
        I,
        Cube
    }
    enum Wind{
        Left,
        Right,
        NoWind,
    }
    fn get_winds_string(file_name:&str) -> String{
        read_to_string(file_name).unwrap()
    }
    fn get_winds_iter<'a>(wind_string:&'a str) -> impl Iterator<Item=Wind>+'a {
        wind_string.chars().map(|c| match c{
            '<' => Wind::Left,
            '>' => Wind::Right,
            _ => panic!("Unknown wind direction!") // We are ok with panicing here.
        })
    }
    fn make_rock_iterator<'a>()-> std::slice::Iter<'a, Rock> {
        [Rock::Minus, Rock::Plus, Rock::L, Rock::I, Rock::Cube].iter()
    }
    fn get_highest_point(upper_floor:[i32;MAP_WIDTH]) -> i32{
        upper_floor.iter().fold(0, |acc,v|{if acc < *v{ *v }else { acc }})
    }
    fn get_initial_shape_bounds(rock_shape:&Rock,highest_point:i32) -> Vec<[[i32;2];2]>{
        let horizontal = LOWER_HORIZONTAL_LIMIT+2;
        let vertical = highest_point+3;
        match rock_shape{
            Rock::Minus => vec![[[vertical,horizontal]  ,[vertical,horizontal+3]  ]],
            Rock::Plus  => vec![[[vertical,horizontal+1],[vertical+2,horizontal+1]] , [[vertical+1,horizontal],[vertical+1,horizontal+2]] ],
          //Rock::Plus  => vec![[[3,            3],    [5,          3]] ,         [[4,       2],           [4,         4]] ],
            Rock::L     => vec![[[vertical,horizontal]  ,[vertical,horizontal+2]  ] , [[vertical,horizontal+2],[vertical+2,horizontal+2]] ],
            Rock::I     => vec![[[vertical,horizontal]  ,[vertical+3,horizontal]  ]],
            Rock::Cube  => vec![[[vertical,horizontal]  ,[vertical+1,horizontal]  ] , [[vertical,horizontal+1], [vertical+1,horizontal+1]]],
        }
    }
    fn move_pos_left(position:Vec<[[i32;2];2]>) -> Vec<[[i32;2];2]>{
        let mut undo_move: bool = false;
        let new_pos = position.iter().map(|[a,b]|{
            let new_a1 = a[1]-1;
            let new_b1 = b[1]-1;
            if new_b1 < LOWER_HORIZONTAL_LIMIT || new_a1 < LOWER_HORIZONTAL_LIMIT{
                undo_move = true;
            }
            [[a[0], new_a1],[b[0], new_b1]]
        }).collect::<Vec<[[i32;2];2]>>();

        match undo_move{
            true => {
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
                position
            },
            false => {
                new_pos.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{new_pos:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{new_pos:?}");
                });
                new_pos
            }
        }
    }
    fn move_pos_right(position:Vec<[[i32;2];2]>) -> Vec<[[i32;2];2]>{
        let mut undo_move: bool = false;
        let new_pos = position.iter().map(|[a,b]|{
            let new_a1 = a[1]+1;
            let new_b1 = b[1]+1;
            if new_a1 > UPPER_HORIZONTAL_LIMIT || new_b1 > UPPER_HORIZONTAL_LIMIT{
                undo_move = true;
            }
            [[a[0], new_a1],[b[0], new_b1]]
        }).collect::<Vec<[[i32;2];2]>>();

        match undo_move{
            true => {
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
                position
            },
            false => {
                new_pos.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{new_pos:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{new_pos:?}");
                });
                new_pos
            }
        }
    }
    fn move_pos_down(position:&Vec<[[i32;2];2]>, upper_floor:[i32;MAP_WIDTH]) -> (Option<Vec<[[i32;2];2]>>, Option<[i32;2]>){
        let mut undo_move: bool = false;
        let mut impact_point: Option<[i32;2]> = None;
        let new_pos = position.iter().map(|[a,b]|{
            let new_a0 = a[0]-1;
            let new_b0: i32 = b[0]-1;
            let mut range_width = (a[1] as usize )..=(b[1] as usize);
            // assert!(LEFT_LIMIT<= a[1] && a[1]<=RIGHT_LIMIT,"{position:?}");
            // assert!(LEFT_LIMIT<= b[1] && b[1]<=RIGHT_LIMIT,"{position:?}");
            if a[1] > b[1]{
                range_width = (b[1] as usize )..=(a[1] as usize);
            }
            
            for i in range_width{
                if upper_floor[i] == new_b0{
                    undo_move = true;
                    impact_point = Some([new_b0,i as i32]);
                    break;
                }else if  upper_floor[i] == new_a0{
                    undo_move = true;
                    impact_point = Some([new_a0,i as i32]);
                    break;
                }
            }
            [[new_a0, a[1]], [ new_b0,b[1]]] 
        }).collect::<Vec<[[i32;2];2]>>();

        // println!("impact_point: {:?}",impact_point);
        match undo_move{
            true => {
                println!("Down-move cancelled!");
                (None,impact_point)},
            false => (Some(new_pos), impact_point)
        }
    }
    fn is_between_points_2d(point:[i32;2], left:[i32;2],right:[i32;2]) -> bool{
        // NOTE: Only chekcs left-right or top-bottom, not diagonals (or similar)!
        for i in 0..1{
            let [p,l,r] = [point[i],left[i],right[i]];
            if !is_between_points_1d(p, l, r){
                return false;
            }
        }return true;

    }
    fn is_between_points_1d(point:i32, left:i32, right:i32) -> bool{
        (left <= point && point <= right) || (right <= point && point <= left)
    }
    fn get_updated_floor(upper_floor:[i32;MAP_WIDTH], position:&Vec<[[i32;2];2]>) -> [i32;MAP_WIDTH]{
        let updated_floor:[i32;MAP_WIDTH] = upper_floor.iter().enumerate().map(|(horizontal_pos,vertical_pos)|{
            let mut vertical_output = *vertical_pos;
            for [from_point, to_point] in position.iter(){
                // Check if horizontal_pos is withing the given range:
                if is_between_points_1d(horizontal_pos as i32, from_point[1], to_point[1]){
                    let max_height = max(from_point[0], to_point[0]);
                    if vertical_output < max_height{
                        vertical_output = max_height;
                    }
                    
                }
            }
            
            vertical_output
            }).collect::<Vec<i32>>().try_into().expect("This should be reliable, as we just iterate over 7 ints and return one int for each");
        return updated_floor;
    }
    pub fn main_1<'a>(file_name:&str, fixed_rock:Option<&Rock>)->Option<i32>{
        let wind_string = get_winds_string(file_name);
        let mut wind_iterator = get_winds_iter(&wind_string);
        let mut rock_iterator = match fixed_rock{
            None => make_rock_iterator().cycle(),
            Some(v)     => match v{
                Rock::Minus => [Rock::Minus].iter().cycle(),
                Rock::Plus  => [Rock::Plus].iter().cycle(),
                Rock::L     => [Rock::L].iter().cycle(),
                Rock::I     => [Rock::I].iter().cycle(),
                Rock::Cube  => [Rock::Cube].iter().cycle()
            }
        };
        let mut rock;
        let mut upper_floor = [0i32;MAP_WIDTH]; // The floor is at 0 to begin with.
        let mut highest_point = 0;
        let mut wind;
        let mut rock_count = 0;
        println!("Entering \"outer_loop\":");
        'outer_loop: while rock_count < N_ITERATIONS {
            rock = rock_iterator.next().expect("Infalable; rock_iterator cycles infinitely");
            println!("Current rock: {rock:?}");
            rock_count += 1;
            let mut position = get_initial_shape_bounds(&rock, highest_point);
            position.iter().for_each(|[a,b]|{
                assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
            });
            println!("Initial shape: {position:?}");
            // Move position sideways, then down, until we hit the ground:
            println!("Entering \"inner_loop\":");
            // ################### Inner Loop Start ###################
            let (position, impact) = 'inner_loop: loop{
                wind = match wind_iterator.next(){ // Out of wind
                    None => Wind::NoWind,
                    Some(v) => v
                };
                // Move sideways
                position = match wind{
                    Wind::Left => move_pos_right(position),
                    Wind::Right =>  move_pos_left(position),
                    Wind::NoWind => position
                };
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
                // println!("Sideways: {position:?}");
                // Move down:
                position = match move_pos_down(&position, upper_floor) {
                    (_, Some(impact_point)) => {
                        println!("Found collision!");
                        println!("Impact at {impact_point:?}");
                        break 'inner_loop (position,impact_point)
                    }, // Once you hit the ground, stop moving downwards.
                    (Some(pos),None) => {
                        println!("{position:?} | {pos:?}");
                        assert_ne!(&position, &pos, "\n\t# New and old position should not be the same if the \"Down Move\" was successfull!");
                        pos
                    },
                    (None, None) => panic!("(None, None) should not be possible."),
                };
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
                println!("Down:     {position:?}\n");
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
            };
            // ################### Inner Loop End ###################


            // Based on where we impact (and what part of the shape impacts there), we increase the upper_floor position.
            println!("Entering \"floor_overlap_section\":");
            
            upper_floor = get_updated_floor(upper_floor, &position);
            highest_point = get_highest_point(upper_floor);
            println!("New floor: {upper_floor:?}\n_______");
        }   
        Some(highest_point+1) // Add one because we're also count the 0th floor.
    }


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_between_points(){
        // NOTE: Only chekcs left-right or top-bottom, not diagonals (or similar)!
        assert!(is_between_points_1d(1, 0, 2));
        assert!(is_between_points_1d(1, 2, 0));
        assert!(is_between_points_1d(1, 1, 1));
        assert!(!is_between_points_1d(0, 1, 1));
        assert!(!is_between_points_1d(0, 10, 50));
        assert!(is_between_points_1d(25_000_000, 6, 500_000_000));

        assert!(is_between_points_2d([3,2], [3,1], [3,3]));
        assert!(is_between_points_2d([2,2], [2,2], [2,2]));
        assert!(is_between_points_2d([15,2], [16,2], [2,2]));
        assert!(!is_between_points_2d([1,2], [2,1], [2,3]));
    }
    #[test]
    fn floor_update_test(){
        let mut upper_floor = [0i32;7];
        let highest_point = 0;
        let rock_shape = Rock::Minus;
        let position = get_initial_shape_bounds(&rock_shape, highest_point);
        let new_floor = get_updated_floor(upper_floor, &position);
        let correct_new_floor = [0,0,3,3,3,3,0];
        assert_eq!(new_floor,correct_new_floor,"Failed at {:?}",rock_shape);

        let mut upper_floor = [0i32;7];
        let highest_point = 0;
        let rock_shape = Rock::Plus;
        let position = get_initial_shape_bounds(&rock_shape, highest_point);
        let new_floor = get_updated_floor(upper_floor, &position);
        let correct_new_floor = [0,0,4,5,4,0,0];
        assert_eq!(new_floor,correct_new_floor,"Failed at {:?}",rock_shape);
    }
    #[test]
    fn initialize_rocks_limit_test(){
        // Floor at floor level:
        let rocks = make_rock_iterator();
        for rock in rocks{
            let position = get_initial_shape_bounds(&rock, 0);
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
        }
        // Floor at uneven level:
        let rocks = make_rock_iterator();
        for rock in rocks{
            let position = get_initial_shape_bounds(&rock, 25);
                position.iter().for_each(|[a,b]|{
                    assert!(LOWER_HORIZONTAL_LIMIT<= a[1] && a[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                    assert!(LOWER_HORIZONTAL_LIMIT<= b[1] && b[1]<=UPPER_HORIZONTAL_LIMIT,"{position:?}");
                });
        }
    }
    #[test]
    fn initialize_l_rock_test(){
        let rock_shape = Rock::L;
        let position = get_initial_shape_bounds(&rock_shape, 0);
        let correct_position = vec![ [[3,2], [3,4]], [[3,4],[5,4]]];
        assert_eq!(position,correct_position);
    }
    #[test]
    fn initialize_plus_rock_test(){
        let rock_shape = Rock::Plus;
        let position = get_initial_shape_bounds(&rock_shape, 0);
        let correct_position = vec![ [[3,3], [5,3]], [[4,2],[4,4]]];
        assert_eq!(position, correct_position);
    }
    #[test]
    fn move_down_test(){
        let rock_shape = Rock::L;
        let upper_floor = [0i32;MAP_WIDTH];

        let correct_old_position = vec![ [[3,2], [3,4]], [[3,4],[5,4]]];
        let correct_new_position = vec![ [[2,2], [2,4]], [[2,4],[4,4]]];

        let position = get_initial_shape_bounds(&rock_shape, 0);
        let new_position = move_pos_down(&position, upper_floor).0.expect("If we panic, the test is failed.");

        assert_eq!(position, correct_old_position, "Initial position is not as expected!");
        assert_ne!(position,new_position,"Old position should not be same as new position!");
        assert_eq!(new_position,correct_new_position,"New position is not as expected!");
    }
    #[test]
    fn move_right_test(){
        let correct_position = vec![[[3,4],[3,6]],[[2,5],[4,5]]];
        let initial_position = vec![[[6,2+1],[6+2,2+1]] , [[6+1,2],[6+1,2+2]] ];
        let intermediary_position = vec![[[6,2+1+3],[6+2,2+1+3]] , [[6+1,2+3],[6+1,2+2+3]] ];
        let correct_upper_floor = [1,1,1,0,3,4,3];
        let rock = Rock::Plus;
        let mut upper_floor=[1,1,1,0,2,0,0];
        let original_position = get_initial_shape_bounds(&rock,3);
        let mut position = get_initial_shape_bounds(&rock,3);
        assert_eq!(position,initial_position);
        for _ in 0..10{
            position = move_pos_right(position); // Not moving as far as expected...
        }
        assert_eq!(position, intermediary_position);
        loop{
            position = match move_pos_down(&position, upper_floor) {
                (_, Some(impact_point)) => {
                    // println!("Found collision!");
                    // println!("Impact at {impact_point:?}");
                    break;
                }, // Once you hit the ground, stop moving downwards.
                (Some(pos),None) => {
                    // println!("{position:?} | {pos:?}");
                    assert_ne!(&position, &pos, "\n\t# New and old position should not be the same if the \"Down Move\" was successfull!");
                    pos
                },
                (None, None) => panic!("(None, None) should not be possible."),
            };

        }
        upper_floor = get_updated_floor(upper_floor, &position);
        assert_ne!(original_position,position);
        assert_eq!(position,correct_position);
        assert_eq!(upper_floor,correct_upper_floor);

    }
    #[test]
    fn main_test(){
        let shape_heights = [1,3,3,4,2];
        for (rock, rock_height) in make_rock_iterator().zip(shape_heights.iter()){
            let main1_result = main_1(r"src\minimal_input.txt", Some(&rock));
            assert_eq!(main1_result,Some(1 + N_ITERATIONS*rock_height), "Rock Shape: {rock:?}");
        }
        let shape_heights = [1,3,3,4,2];
        for (rock, rock_height) in make_rock_iterator().zip(shape_heights.iter()){
            let main1_result: Option<i32> = main_1(r"src\single_input.txt", Some(&rock));
            match rock{
                Rock::Minus | Rock::Cube => assert_eq!(main1_result,Some(1+ N_ITERATIONS*rock_height), "Rock Shape: {rock:?}"),
                Rock::I => assert_eq!(main1_result,Some(1+ (N_ITERATIONS*rock_height) - 4), "Rock Shape: {rock:?}"),
                Rock::Plus => assert_eq!(main1_result,Some(1+ N_ITERATIONS*rock_height -1 ), "Rock Shape: {rock:?}"),
                Rock::L => assert_eq!(main1_result,Some(1+ N_ITERATIONS*rock_height -2 ), "Rock Shape: {rock:?}"),

            }
            
        }
    }
}


}
mod p2{
    use std::fs::read_to_string;
    pub fn main_2(file_name:&str)->Option<i32>{
      None
    }
}

use p1::main_1;
use p2::main_2;
use std::time::Instant;

fn main() {
    let file_name = r"src\dummy_input.txt";
    // let file_name= r"src\puzzle_input.txt";
    let start = Instant::now();
    let count = main_1(file_name,None);
    let end = start.elapsed();
    println!("\nPart 1: {count:?}\nRuntime: {end:?}");

    let start = Instant::now();
    let count = main_2(file_name);
    let end = start.elapsed();
    println!("\nPart 2: {count:?}\nRuntime: {end:?}");
}