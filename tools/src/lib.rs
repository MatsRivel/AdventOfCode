use std::ops::{Add,Sub};
use std::fmt::Debug;
pub fn two_members_sum_to_n<T>(sorted_elements:&mut Vec<T>,n:T) -> Option<[T;2]>
where T: Ord + Add<Output=T> + Copy + Debug{
    let mut left = 0;
    let mut right = sorted_elements.len()-1;
    while left != right{
        let sum:T = sorted_elements[left] + sorted_elements[right];
        if sum == n{
            return Some([sorted_elements[left],sorted_elements[right]]);
        }else if sum > n{
            right -=1;
        }else{
            left += 1;
        }
    }
    None
}
fn two_members_sum_to_n_exclude<T>(sorted_elements:&mut Vec<T>,n:T, exclude_idx:Option<usize>) -> Option<[T;2]>
where T: Ord + Add<Output=T> + Copy + Debug{
    let mut left = 0;
    let mut right = sorted_elements.len()-1;
    if  exclude_idx == Some(left){
        left += 1;
    }
    if  exclude_idx == Some(right){
        right -= 1;
    }
    while left != right{
        let sum:T = sorted_elements[left] + sorted_elements[right];
        if sum == n{
            return Some([sorted_elements[left],sorted_elements[right]]);
        }else if sum > n{
            right -=1;
            if  exclude_idx == Some(right){
                right -= 1;
            }
        }else{
            left += 1;
            if  exclude_idx == Some(left){
                left += 1;
            }
        }
    }
    None
}
pub fn three_members_sum_to_n<T>(sorted_elements:&mut Vec<T>,n:T) -> Option<[T;3]>
where T: Ord + Sub<Output=T> + Add<Output=T> + Copy + Debug{
    let mut sorted_clone = sorted_elements.clone();
    for (i,c) in sorted_elements.iter().enumerate(){
        if let Some([a,b]) = two_members_sum_to_n_exclude(&mut sorted_clone, n-*c, Some(i)){
            return Some([a,b,*c]);
        }

    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
