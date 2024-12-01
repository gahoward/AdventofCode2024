use std::fs;
use std::str::Lines;


fn main() {
    let input_file_location: String = String::from("input.txt");
    let lines: Vec<String> = file_to_vector(input_file_location);
    
    /* 
    First attempt before I realised I could do this as a Vector of Tuples and use map/reduce/etc.
    
    //let two_lists = extract_lists(lines);
    //let mut list1 = two_lists.0;
    //let mut list2 = two_lists.1;
    //list1 = sort_list(list1);
    //list2 = sort_list(list2);
    //let difference_vec = difference_calculation(list1, list2);
    //let final_value: i32 = difference_vec.iter().sum();
    //println!("Final value is: {}", final_value);

    */

    let two_lists = extract_lists_tuples(lines);
    println!("two_list values is: {:?}", two_lists);

    let part1_value: i32 = two_lists.iter().map(|x| (x.0 - x.1).abs()).sum();
     println!("Part 1 value is: {}", part1_value);

     let part2_value: i32 = similarity_calculator(two_lists);
     println!("Part 2 value is: {}", part2_value);


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

/*

Original solution, before I realised we could simply do map/reduce/filter/fold on Tuples.

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

    */

//This seems more idiomatic: we return a Vector of (i32, i32) tuples;
fn extract_lists_tuples(input: Vec<String>) -> Vec<(i32, i32)>
{
    let mut unsorted_left: Vec<i32> = Vec::new();
    let mut unsorted_right: Vec<i32> = Vec::new();


    //Line by line iteration.
    for line in input {
    //Horribly unreadable mess but: split each string into an iterator, providing 2 elements sans whitespace use map to parse each element into a number, returning (hopefully!) two numeric values.
    let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
    match (parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b))) => {
            unsorted_left.push(a);
            unsorted_right.push(b);
        }
        //Code will panic here if we get a malformed list ... but the input is well-formed so I'm not doing any additional checks here.
        _ => {} 
    }
    }

    //Sort both lists as it's required for the first part of the task, and doesn't harm the second part.
    unsorted_left.sort();
    unsorted_right.sort();

    //This ensures that the lowest index of the Vector has the lowest values of each of the two lists, the second lowest has the second lowest of the two lists, etc.
    let mut combined_vector: Vec<(i32, i32)> = Vec::new();
    for (pos, element) in unsorted_left.iter().enumerate() {
        combined_vector.push((*element, *unsorted_right.get(pos).unwrap()));
    }

    //Return the Vector of Tuples;
    return combined_vector
}

//Mostly I am showing off here.
fn similarity_calculator(input: Vec<(i32, i32)>) -> i32 {
    let mut similarity = 0;
    for (pos, element) in input.iter().enumerate() {
        similarity += element.0 * input.iter().map(| x | x.1 - element.0).filter(| y | *y == 0).fold(0, | acc, element | acc + 1 );
        
    }
    return similarity
}