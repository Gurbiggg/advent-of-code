use std;

pub fn parse_input_into_lists(left:&mut Vec<u32>, right: &mut Vec<u32>, file: &str) {
    let input = std::fs::read_to_string(file)
        .expect("Should have been able to read file");
    let vec: Vec<&str> = input.split("\n").collect();
    let input_iter = vec.iter();

    for x in input_iter {
        let y: Vec<&str> = x.split_whitespace().collect();
        if ! y.is_empty() {
            let left_item: u32 = y[0].parse().unwrap();
            let right_item: u32 = y[1].parse().unwrap();

            left.push(left_item);
            right.push(right_item);
        }
    }
}
