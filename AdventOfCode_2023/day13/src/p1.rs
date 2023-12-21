use std::fs::read_to_string;
struct Pattern<T>
where T: Sized{
    matrix: Vec<T>,
    xmax:usize,
    ymax:usize
}
impl Pattern<bool>{
    fn new(lines:&Vec<&str>)->Self{
        let xmax = lines.len();
        let ymax = lines[0].len();
        let matrix = lines
            .iter()
            .flat_map(|line|{
                line.chars().map(|c| {
                    match c{
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid character")
                    }
                }).collect::<Vec<bool>>()
        }).collect::<Vec<bool>>();
        Self{matrix,xmax,ymax}
    }
    fn coord_to_idx(&self,coord: [usize;2])-> usize{
        coord[0]*self.ymax + coord[1]
    }
    fn idx_to_coord(&self,idx:usize) -> [usize;2]{
        [idx/self.ymax, idx%self.ymax]
    }
    fn get(&self,x:usize,y:usize)->bool{
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx]
    }
    fn set(&mut self,x:usize,y:usize, val:bool){
        let idx = self.coord_to_idx([x,y]);
        self.matrix[idx] = val;
    }
    fn print_pattern(&self){
        for x in 0..self.xmax{
            for y in 0..self.ymax{
                if self.get(x, y){
                    print!(".");
                }else{
                    print!("#");
                }
            }
            println!();
        }
        println!();
    }
}

fn process_data_string(data_string:String)->Vec<Pattern<bool>>{
    let mut segments = vec![vec![]];
    let mut idx = 0;
    for line in data_string.lines(){
        if line == ""{
            idx +=1;
            segments.push(vec![]);
        }else{
            segments[idx].push(line);
        }
    }
    let output = segments
        .iter()
        .map(|segment|{
            Pattern::new(segment)
        }).collect::<Vec<Pattern<bool>>>();
    
    output
}
fn exstensive_mirror_check_vertical(pattern:&Pattern<bool>, idx: usize)->bool{
    (0..=idx)
        .rev()
        .zip((idx+1)..pattern.ymax)
        .fold(true,|acc, (left,right)|{
            let current_truth = (0..pattern.xmax)
                .fold(true, |inner_acc,i| {
                    let [lval,rval] = [pattern.get(i,left),pattern.get(i,right)];
                    let inner_truth = lval == rval;
                    #[cfg(test)]{
                        if !inner_truth{
                            println!(">>>\tInner truth failed at idx {i}. ||{lval}\t!= {rval} \t||<<<");
                        }
                    } 
                    inner_acc && inner_truth
                });
            acc && current_truth  
                    
        })
}
fn exstensive_mirror_check_horizontal(pattern:&Pattern<bool>, idx: usize)->bool{
    (0..=idx)
        .rev()
        .zip((idx+1)..pattern.xmax)
        .fold(true,|acc, (left,right)|{
            #[cfg(test)]
            println!("columns: {left} {right}");
            let current_truth = (0..pattern.ymax)
                .fold(true, |inner_acc,i| {
                    let [lval,rval] = [pattern.get(left, i), pattern.get(right, i)];
                    let inner_truth = lval == rval;
                    #[cfg(test)]{
                        if !inner_truth{
                            println!(">>>\tInner truth failed at idx {i}. ||{lval}\t!= {rval} \t||<<<");
                        }
                    } 
                    inner_acc && inner_truth
                });
            acc && current_truth   
        })
}
fn find_mirror(pattern:&Pattern<bool>)->Option<(usize,bool)>{ // -> ( idx, is_vertical )
    'y_loop: for y in 0..pattern.ymax-1{
        #[cfg(test)]
        println!("y: {y}");
        for x in 0..pattern.xmax-1{
            #[cfg(test)]{
                println!("x: {x} | {} {}",pattern.get(x, y),pattern.get(x,y+1))
            }
            if pattern.get(x, y) != pattern.get(x,y+1){
                #[cfg(test)]
                println!();
                continue 'y_loop;
            }
        }
        if exstensive_mirror_check_vertical(pattern, y){
            #[cfg(test)]
            println!(" >>> Extensive vertical mirror check succeeded! <<< ");
            return Some((y,true));
        }else{
            #[cfg(test)]
            println!("Extensive vertical mirror check failed")
        }
    }day14
    'x_loop: for x in 0..pattern.xmax-1{
        #[cfg(test)]
        println!("x: {x}");
        for y in 0..pattern.ymax-1{
            #[cfg(test)]{
                println!("y: {y} | {} {}",pattern.get(x, y),pattern.get(x+1,y))
            }
            if pattern.get(x, y) != pattern.get(x+1,y){
                #[cfg(test)]
                println!();
                continue 'x_loop;
            }
        }
        if exstensive_mirror_check_horizontal(pattern, x){
            #[cfg(test)]
            println!(" >>> Extensive horizontal mirror check succeeded! <<< ");
            return Some((x,false));
        }else{
            #[cfg(test)]
            println!("Extensive horizontal mirror check failed")
        }
    }
    None
}
fn process_pattern(pattern:&Pattern<bool>)->usize{
    if let Some((mirror_idx,is_vertical)) = find_mirror(pattern){
        if is_vertical{
            return mirror_idx+1;
        }else{
            return (mirror_idx+1)*100;
        }
    }else{
        panic!("There is always ONE mirror axis");
    }
}
pub fn main_1(file_name:&str)->Option<usize>{
    let data_string = read_to_string(file_name).unwrap();
    let patterns = process_data_string(data_string);
    let mut total = 0;
    #[cfg(test)]
    println!("There are {} patterns.",patterns.len());
    #[cfg(test)]
    let expected_outputs = vec![5,400];
    for (_idx,pattern) in patterns.iter().enumerate(){
        #[cfg(test)]{
            pattern.print_pattern();
        }
        let process_value = process_pattern(pattern); 
        #[cfg(test)]{
            assert_eq!(process_value,expected_outputs[_idx]);
        }
        total += process_value;
    }
    Some(total)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn small_case(){
        let data_string = "#######\n......#\n##..##.".to_string();
        let patterns = process_data_string(data_string);
        let mut total = 0;
        println!("There are {} patterns.",patterns.len());
        #[cfg(test)]
        let expected_outputs = vec![5,400];
        for (_idx,pattern) in patterns.iter().enumerate(){
            pattern.print_pattern();
            let process_value = process_pattern(pattern); 

            total += process_value;
        }
        assert_eq!(total,1);
    }

}
