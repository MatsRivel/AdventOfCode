use std::fs::read_to_string;

fn process_data_string(data_string:&str)->Vec<Vec<i32>>{
    data_string.lines()
        .map(|line| {
            line.split(" ")
            .map(|s| {
                s.parse::<i32>().unwrap()
            } ).collect::<Vec<i32>>() 
        })
        .collect::<Vec<Vec<i32>>>()
}
fn process_row(row: &Vec<i32>)->i32{
    let diff_vec = row.iter().rev().zip(row.iter().rev().skip(1)).map(|(right,left)| left-right).rev().collect::<Vec<i32>>();
    let only_zero = diff_vec.iter().fold(true, |acc,v| acc && *v == 0);
    if only_zero{
        // println!("0, {diff_vec:?}");
        return 0;
    }
    let diff = process_row(&diff_vec);
    let output = diff + diff_vec.first().unwrap();
    // println!("{output}, {diff_vec:?}");
    output
}
pub fn main_2(file_name:&str)->Option<i32>{
    #[cfg(test)]
    let ans = [-3,0,5];
    let data_string = read_to_string(file_name).unwrap();
    let rows = process_data_string(&data_string);
    let final_value = rows
        .into_iter()
        .enumerate()
        .map(|(idx,row)| {
            let start = *row.first().unwrap();
            let row_diff = process_row(&row);
            let final_val = row_diff + start;
            // println!("\n{final_val}, {row:?}\n");
            #[cfg(test)]{
                assert_eq!(final_val,ans[idx]);
            }
            final_val
        })
        .fold(0, |acc,v| acc+v);
    Some(final_value)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
