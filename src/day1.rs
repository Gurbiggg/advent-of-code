use std;
use crate::lib::parse_input_into_lists;

pub fn main() {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    let file: &str = "src/input_day1.txt";
    parse_input_into_lists(&mut left_list, &mut right_list, file);

    left_list.sort();
    right_list.sort();
    let list_length = left_list.len();
    let mut sum: u32 = 0;

    for i in 0..list_length {
        let left_item = left_list[i];
        let right_item = right_list[i];
        let diff = u32::abs_diff(left_item, right_item);
        println!("ABSDIFF: {left_item} - {right_item} = {diff}");
        sum += diff;
    }

    println!("{}", sum);
}
