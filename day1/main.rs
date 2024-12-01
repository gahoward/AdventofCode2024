use std::fs;
use std::str::Lines;


fn main() {
    let input_file_location: String = String::from("input.txt");
    let lines: Vec<String> = file_to_vector(input_file_location);
    let two_lists = extract_lists(lines);
    let mut list1 = two_lists.0;
    let mut list2 = two_lists.1;
    list1 = sort_list(list1);
    list2 = sort_list(list2);
    let difference_vec = difference_calculation(list1, list2);
    let final_value: i32 = difference_vec.iter().sum();
    //Ta-da!
    println!("Final value is: {}", final_value);
}

//Read file line-by-line, push each line into a Vector of type String, which we will subsequently manipulate into two independent lists of i32s.
fn file_to_vector(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut line_vec: Vec<String> = Vec::new();
    let lines: Lines = contents.lines();
    for line in lines {
        line_vec.push(line.to_string());
    }

    return line_vec;
}

//Produce two Vectors in a tuple, each representing one column of digits (as i32s), eliminating the whitespace in the input that separates the two values.
fn extract_lists(input: Vec<String>) -> (Vec<i32>, Vec<i32>)
{
    //Left-side list
    let mut sample1: Vec<i32> = Vec::new();
    //Right-side list.
    let mut sample2: Vec<i32> = Vec::new();

    //Line by line iteration.
    for line in input {
    //Horribly unreadable mess but: split each string into an iterator, providing 2 elements sans whitespace use map to parse each element into a number, returning (hopefully!) two numeric values.
    let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            sample1.push(a);
            sample2.push(b);
            
        }
        // handle other problems: not enough numbers, numbers are invalid, etc
        _ => {}  // ignore invalid input
    }
    }
    //Return tuple, which we can unpack back in main into two separate lists for the next steps.
    return (sample1, sample2);
}

//Sorts a list. I'm aware this doesn't save any time. I'm thinking in terms of primitive operations.
fn sort_list(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    return input;
}

//Iterate through one list, using the position variable to also get the value from the other. Subtract the second number from the first, push the absolute form of that number to a vector, and then return that vector.
fn difference_calculation(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    let mut difference_vec: Vec<i32> = Vec::new();

    for (pos, e) in list1.iter().enumerate() {
        let diff: i32 = e - list2.get(pos).unwrap();
        difference_vec.push(diff.abs());
    }
    
    return difference_vec;
}