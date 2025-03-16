use crate::lib::parse_input_into_lists;

pub fn main() {
    /*
     * - parse input data into left and right lists
     * - convert strings into u32 ints
     * - sort lists?
     * - iterate through left list
     * - check if i is in right list
     * - add to mult variable
     * - multiply mult by left[i]
     * - add to a multiplied list
     * - sum all elements in list to sim score
     */

    let mut sim_score:u32 = 0;
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let mut mult_list: Vec<u32> = Vec::new();

    let file: &str = "src/input_day1.txt";
    parse_input_into_lists(left_list, right_list, file);

    left_list.sort();
    right_list.sort();

    for i in left_list {
        let mut mult: u32 = 0;
        for j in right_list {
            if i == j {
                mult += 1;
            }
        }
        mult_list.push(i * mult);
    }

    for x in mult_list {
        sim_score += x;
    }
}
