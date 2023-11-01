use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
pub struct NumComboManager{
    pub idx:usize,
    pub own_value: i64,
    pub combos: HashMap<i64,usize> // For HashMap<x,y>: You can get value x by combining the values at y and idx
    // NOTE: Only contians indices greater than its own!
}
impl NumComboManager{
    pub fn new(idx:usize,value:i64)->Self{
        Self{idx, own_value: value, combos: HashMap::<i64,usize>::new()}
    }
    pub fn insert(&mut self, idx_in:usize, val_in:i64){
        // We don't mind overwriting in case old value is same as new,
        // because we only need one copy of the values.
        if idx_in <= self.idx{
            panic!("Having a lower index than a previous one means there is a logical error somewhere.")
        }
        #[cfg(test)]
        println!("For [{}] + [{idx_in}]:\n{} + {val_in} == {}\n",self.idx, self.own_value, val_in+self.own_value);
        self.combos.insert(val_in+self.own_value, idx_in);
    }
    pub fn contains(&self, target_val:i64)->Option<usize>{
        // Returns the idx you need to combine this idx with to get the target_val
        if let Some(v) = self.combos.get(&target_val){
            Some(*v)
        }else{
            None
        }
    }
}
pub fn get_invalid_number(numbers:&Vec<i64>, preamble: usize) -> Option<i64>{
    let mut num_combo: HashMap<usize,NumComboManager> = HashMap::new(); 
    // Insert any combination of the first 5 numbers:
    for i in 0..preamble{
        let mut ncm = NumComboManager::new(i,numbers[i]);
        for j in (i+1)..preamble{
            ncm.insert(j,numbers[j]);
        }
        num_combo.insert(i, ncm);
    }

    // Go throug the remaining numbers to find one that has no "matching pair":
    for new_idx in preamble..numbers.len(){
        let new_val = numbers[new_idx];
        let mut combo_found = false;
        for (i, ncm) in num_combo.iter(){
            match ncm.contains(new_val){
                Some(j) => {
                    #[cfg(test)]
                    println!("N[{i}] ({}) + N[{j}] ({})== {new_val}",numbers[*i], numbers[j]);
                    combo_found = true;
                    break;
                },
                None => {continue} 
            }
        }
        if !combo_found{
            return Some(new_val);
        }
        num_combo.remove(&(new_idx-preamble)); // Remove the element at the end of the area we consider.
        let new_ncm = NumComboManager::new(new_idx,numbers[new_idx]);
        num_combo.iter_mut().for_each(|(_, ncm)|{
            ncm.insert(new_idx, numbers[new_idx])
        });
        num_combo.insert(new_idx, new_ncm);
    }
    None
    
}
pub fn main_1(file_name:&str, preamble:usize)->Option<i64>{
    let data_string = read_to_string(file_name).unwrap();
    let numbers = data_string.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    get_invalid_number(&numbers, preamble)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
