use std::fs::read_to_string;
fn print_spaces(x:&usize){
    for _ in 0..*x{
        print!("- ");
    }
}
pub fn depth_first_search(adapters:&Vec<i32>,current:&usize)->i32{
    if *current == adapters.len()-1{
        return 1;
    }
    let mut neighbours = Vec::new();
    let val = adapters[*current];
    for i in (current+1)..adapters.len() {
        let neigh_val = adapters[i];
        if neigh_val - val <=3{
            neighbours.push(i);
        }else{
            break;
        }
    }
    let mut current_sum = 0;
    for n in neighbours.iter() {
        current_sum += depth_first_search(adapters, n);
    }

    current_sum
}
pub fn main_2(file_name:&str)->Option<i64>{
    let data_string = read_to_string(file_name).unwrap();
    let mut adapters = vec![0];
    adapters.append(&mut data_string.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    adapters.sort();
    adapters.push(adapters.last().unwrap()+3);

    let mut neighbours = vec![];
    let mut complexity = vec![];
    for i in 0..adapters.len(){
        neighbours.push(vec![]);
        complexity.push(0);
        for j in 0..i{
            if adapters[i] - adapters[j] <=3{
                neighbours[i].push(j);
            }
        }
        if neighbours[i].len() == 0{
            complexity[i] = 1;
        }else{
            for neighbour in neighbours[i].iter(){
                complexity[i] += complexity[*neighbour];
            }
        }
    }

    #[cfg(test)]
    println!("{adapters:?}");
    #[cfg(test)]
    println!("{complexity:?}");
    let final_complexity = *complexity.last().unwrap();
    Some(final_complexity)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
