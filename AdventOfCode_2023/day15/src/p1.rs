use std::fs::read_to_string;

pub fn hash_function(s:&str)->u128{
    s.chars()
        .fold(0u128, |mut acc,c|{
            acc += (c as u8) as u128;
            acc *= 17;
            acc % 256u128
        }
    )
}
pub fn main_1(file_name:&str)->Option<u128>{
    let data_string = read_to_string(file_name).unwrap();
    let output = data_string.lines()
        .flat_map(|line| line.split(','))
        .fold(0, |acc, s| {
            acc + hash_function(s)
        });
    Some(output)

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn hash_test(){
        let combinations = vec![
            ("rn=1",30),
            ("cm-",253),
            ("qp=3",97),
            ("cm=2",47),
            ("qp-",14),
            ("pc=4",180),
            ("ot=9",9),
            ("ab=5",197),
            ("pc-",48),
            ("pc=6",214),
            ("ot=7",231)
            ];
        for (input,expected) in combinations.into_iter(){
            let output = hash_function(input);
            assert_eq!(output,expected, "'{input}' == {output},  != {expected}")
        }
    }

}
