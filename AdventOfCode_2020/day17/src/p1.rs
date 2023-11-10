use std::{fs::read_to_string, collections::{HashSet, HashMap}};

struct NDimConway<const N_DIMS:usize>{
    grid: HashSet<[i32;N_DIMS]> 
}
impl<const N_DIMS:usize> NDimConway<N_DIMS>{
    fn new_from_string(data_string:&str) -> Self{
        let mut grid = data_string.lines().enumerate().flat_map(|(i,line)|{
            line.char_indices().filter_map(move |(j,c)|{
                match c{
                    '#' => {
                        let mut base = [0;N_DIMS];
                        base[0] = i as i32;
                        base[1] = j as i32;
                        Some(base)
                    }, // Input only occurs in a single z-layer.
                    _ => None
                }
            })
        }).collect::<HashSet<[i32;N_DIMS]>>();
        Self{grid}
    }
    fn recursive_neighbour_iteration(pos:&[i32;N_DIMS], idx:usize)->Vec<[i32;N_DIMS]>{
        if idx == N_DIMS{
            return vec![*pos];
        }
        let collected_positions = ([-1,0,1].iter()).flat_map(|i|{
            let mut new_pos = pos.clone();
            new_pos[idx] += i;
            let neighbours = NDimConway::recursive_neighbour_iteration(&new_pos, idx+1);
            neighbours
        }).collect::<Vec<[i32;N_DIMS]>>();
        collected_positions
    }
    
    fn get_neighbours_incl_self(pos:&[i32;N_DIMS])->Vec<[i32;N_DIMS]>{
        let mut all_neighbours = NDimConway::recursive_neighbour_iteration(pos, 0);

        all_neighbours.sort();
        let processed_neighbours = all_neighbours.windows(2)
        .filter_map(|pair|{
            let [previous,current] = [pair[0],pair[1]];
            if previous == current{
                None
            }else{
                Some(current)
            }
        }).collect::<Vec<[i32;N_DIMS]>>();

        all_neighbours
    }

    fn get_neighbours_excl_self(pos:&[i32;N_DIMS])->Vec<[i32;N_DIMS]>{
        let mut output = NDimConway::get_neighbours_incl_self(pos);
        if let Some(index_to_remove) = output.iter().position(|inner_pos| inner_pos == pos){
            output.remove(index_to_remove);
        }
        output
    }
    fn conway_step(&mut self){
        let grid = &self.grid;
        let mut initial_neighbour_count = HashMap::new();
        // We store neighbour count early the first time around to prevent doing the same thing twice.
        let mut all_unsorted_neighbours = grid.iter()
            .flat_map(|pos|{
                let current_neighbours = NDimConway::get_neighbours_incl_self(pos);
                let active_neighbours_count = current_neighbours
                    .iter()
                    .filter_map(|pos|{
                        grid.get(pos)
                    }).fold(0, |acc,_| acc+1);
                initial_neighbour_count.insert(*pos,active_neighbours_count-1); // -1 to account for self.
                current_neighbours
            }).collect::<Vec<[i32;N_DIMS]>>();
        // Find all unique original AND neighbouring positions. These are all candidates to change.
        all_unsorted_neighbours.sort();
        // Clear out duplicates.
        let all_neighbours = all_unsorted_neighbours
            .windows(2)
            .filter_map(|pair|{
                let [previous,current] = [pair[0],pair[1]];
                if previous == current{
                    None
                }else{
                    Some(current)
                }
            }).collect::<Vec<[i32;N_DIMS]>>();
        // Note that we've already checked the currently active positions, so if there is an entry in the neighbours_count already, we get that one.
        let new_grid = all_neighbours
        .iter()
        .filter_map(|pos|{
            match initial_neighbour_count.get(pos){
                Some(count) => { // This means that the position in currently active.
                    #[cfg(none)]{
                        println!("{pos:?}: {count}");
                    }
                    if count == &2usize || count == &3usize{
                        Some(*pos)
                    }else{
                        None
                    }
                },
                None => { // This means that the position in currently active.
                    let count = NDimConway::get_neighbours_excl_self(pos)
                        .iter()
                        .filter_map(|pos|{
                            grid.get(pos)
                        }).fold(0, |acc,_| acc+1);
                    #[cfg(none)]{
                        println!("{pos:?}: {count}");
                    }
                    if count == 3{
                        Some(*pos)
                    }else{
                        None
                    }
                }
            }
        }).collect::<HashSet<[i32;N_DIMS]>>();
        #[cfg(test)]{
            if N_DIMS == 3{
                let mut temp_grid = HashSet::<[i32;3]>::new();
                for pos in new_grid.iter(){
                    let temp_val = [pos[0],pos[1],pos[2]];
                    temp_grid.insert(temp_val);
                }
                print_grid(&temp_grid);
            }
        }
        self.grid = new_grid;
    }
}

fn get_3d_neighbours_inc_self(pos:&[i32;3])->Vec<[i32;3]>{
    (-1..=1).flat_map(|i|{
        (-1..=1).flat_map(move |j|{
            (-1..=1).map(move |k| {
                [pos[0]+i,pos[1]+j, pos[2]+k]
            })
        })
    }).collect::<Vec<[i32;3]>>()
}
fn get_3d_neighbours_exc_self(pos:&[i32;3])->Vec<[i32;3]>{
    (-1..=1).flat_map(|i|{
        (-1..=1).flat_map(move |j|{
            (-1..=1).filter_map(move |k| {
                if i == 0 && j == 0 && k == 0{
                    None
                }else{
                    Some([pos[0]+i,pos[1]+j, pos[2]+k])
                }
            })
        })
    }).collect::<Vec<[i32;3]>>()
}
fn three_dimensional_conway_step(grid: HashSet<[i32;3]>) -> HashSet<[i32;3]>{
    let mut initial_neighbour_count = HashMap::new();
    // We store neighbour count early the first time around to prevent doing the same thing twice.
    let mut all_unsorted_neighbours = grid
        .iter()
        .flat_map(|pos|{
            let current_neighbours = get_3d_neighbours_inc_self(pos);
            let active_neighbours_count = current_neighbours
                .iter()
                .filter_map(|pos|{
                    grid.get(pos)
                }).fold(0, |acc,_| acc+1);
            initial_neighbour_count.insert(*pos,active_neighbours_count-1); // -1 to account for self.
            current_neighbours
        }).collect::<Vec<[i32;3]>>();

    // Find all unique original AND neighbouring positions. These are all candidates to change.
    all_unsorted_neighbours.sort();
    let all_neighbours = all_unsorted_neighbours
        .windows(2)
        .filter_map(|pair|{
            let [previous,current] = [pair[0],pair[1]];
            if previous == current{
                None
            }else{
                Some(current)
            }
        }).collect::<Vec<[i32;3]>>();
    // Note that we've already checked the currently active positions, so if there is an entry in the neighbours_count already, we get that one.
    let new_grid = all_neighbours
        .iter()
        .filter_map(|pos|{
            match initial_neighbour_count.get(pos){
                Some(count) => { // This means that the position in currently active.
                    #[cfg(none)]{
                        println!("{pos:?}: {count}");
                    }
                    if count == &2usize || count == &3usize{
                        Some(*pos)
                    }else{
                        None
                    }
                },
                None => { // This means that the position in currently active.
                    let count = get_3d_neighbours_exc_self(pos)
                        .iter()
                        .filter_map(|pos|{
                            grid.get(pos)
                        }).fold(0, |acc,_| acc+1);
                    #[cfg(none)]{
                        println!("{pos:?}: {count}");
                    }
                    if count == 3{
                        Some(*pos)
                    }else{
                        None
                    }
                }
            }
        }).collect::<HashSet<[i32;3]>>();
    print_grid(&new_grid);
    return new_grid;

}

fn print_grid(grid:&HashSet<[i32;3]>){
    let min_max_x = {
        grid.iter()
            .map(|arr|arr[0])
            .fold([i32::MAX,i32::MIN], |mut acc,v|{
                if acc[0] > v{
                    acc[0] = v;
                }else if acc[1] < v{
                    acc[1] = v;
                }
                acc
        })
    };
    let min_max_y= {
        grid.iter()
            .map(|arr|arr[1])
            .fold([i32::MAX,i32::MIN], |mut acc,v|{
                if acc[0] > v{
                    acc[0] = v;
                }else if acc[1] < v{
                    acc[1] = v;
                }
                acc
            })
    };
    let min_max_z= {
        grid.iter()
            .map(|arr|arr[2])
            .fold([i32::MAX,i32::MIN], |mut acc,v|{
                if acc[0] > v{
                    acc[0] = v;
                }else if acc[1] < v{
                    acc[1] = v;
                }
                acc
            })
    };
    println!("___________________________");
    for z in min_max_z[0]..=min_max_z[1]{
        println!("z: {z}");
        for x in min_max_x[0]..=min_max_x[1]{
            for y in min_max_y[0]..=min_max_y[1]{
                match grid.get(&[x,y,z]){
                    Some(_) => print!("#"),
                    None => print!(".")
                }
            }
            println!();
        }
        println!();
    }
    
}

pub fn ndim_main<const N_DIMS:usize>(file_name:&str, n_cycles:usize)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut conway_grid: NDimConway<N_DIMS> = NDimConway::new_from_string(&data_string);

    for _ in 0..n_cycles{
        conway_grid.conway_step();
    }
    let total_active = conway_grid.grid.len() as i32;
    Some(total_active)
}
pub fn old_main(file_name:&str, n_cycles:usize)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let mut grid = data_string.lines().enumerate().flat_map(|(i,line)|{
        line.char_indices().filter_map(move |(j,c)|{
            match c{
                '#' => Some([i as i32,j as i32, 0 ]), // Input only occurs in a single z-layer.
                _ => None
            }
        })
    }).collect::<HashSet<[i32;3]>>();
    for _ in 0..n_cycles{
        #[cfg(none)]{
            for key in grid.iter() {
                print!("{key:?}, ");
            }
            println!("\r");
        }
        grid = three_dimensional_conway_step(grid);
    }
    #[cfg(none)]{
        for key in grid.iter() {
            print!("{key:?}, ");
        }
        println!("\r");
    }
    let total_active = grid.len() as i32;
    Some(total_active)

}
pub fn main_1(file_name:&str, n_cycles:usize)->Option<i32>{
    // old_main(file_name, n_cycles)
    ndim_main::<3usize>(file_name, n_cycles)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn n_dim_solution_test(){
        let file_name = r"src\dummy.txt";
        for n_cycles in 1..=6{
            println!("############ Main c:{n_cycles} ############");
            let original_ans = old_main(file_name,n_cycles);
            println!("############ ndim c:{n_cycles} ############");
            let ndim_ans = ndim_main::<3usize>(file_name,n_cycles);

            assert_eq!(original_ans,ndim_ans,"Failed at n_cycles = {n_cycles}")
        }
    }
    #[test]
    fn n_dim_correctness_basic_test(){
        let neighbours = NDimConway::get_neighbours_incl_self(&[1,1]);
        let expected_positions = [
            [0,0],
            [0,1],[1,0],
            [1,1],
            [0,2],[2,0],
            [1,2],[2,1],
            [2,2]
        ];
        for pos in expected_positions{
            assert!(neighbours.contains(&pos));
        }
        assert_eq!(neighbours.len(),expected_positions.len());

    }
    #[test]
    fn n_dim_correctness_3d_inclusive_test(){
        let initial_pos = [45,-80,0];
        let neighbours = NDimConway::get_neighbours_incl_self(&initial_pos);
        let expected_positions = get_3d_neighbours_inc_self(&initial_pos);
        for pos in expected_positions.iter(){
            assert!(neighbours.contains(&pos));
        }
        assert_eq!(neighbours.len(),expected_positions.len());
    }

    #[test]
    fn n_dim_correctness_3d_exclusive_test(){
        let initial_pos = [45,-80,0];
        let neighbours = NDimConway::get_neighbours_excl_self(&initial_pos);
        let expected_positions = get_3d_neighbours_exc_self(&initial_pos);
        for pos in expected_positions.iter(){
            assert!(neighbours.contains(&pos));
        }
        assert_eq!(neighbours.len(),expected_positions.len());
    }

}
