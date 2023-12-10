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
    let diff_vec = row.iter().zip(row.iter().skip(1)).map(|(prev,next)| next-prev).collect::<Vec<i32>>();
    let only_zero = diff_vec.iter().fold(true, |acc,v| acc && *v == 0);
    if only_zero{
        return 0;
    }
    let diff = process_row(&diff_vec);
    let output = diff_vec.last().unwrap() + diff;
    output
}
pub fn main_1(file_name:&str)->Option<i32>{
    let data_string = read_to_string(file_name).unwrap();
    let rows = process_data_string(&data_string);
    let final_value = rows
        .into_iter()
        .map(|row| {
            let end = *row.last().unwrap();
            let row_val = process_row(&row);
            row_val+end
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
