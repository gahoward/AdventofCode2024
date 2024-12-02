use std::fs;
use std::str::Lines;


fn main() {
    let input_file_location: String = String::from("input.txt");
    let lines: Vec<String> = file_to_vector(input_file_location);
    let new_format = row_of_integers(lines);
    println!("{:?}", new_format.get(0));
    let mut reports: Vec<Report> = convert_to_reports(new_format);
    let mut reports2 = reports.clone();
    reports = check_safety_of_reports(reports);
    println!("Safe reports total for Part 1: {}", reports.iter().filter(|report| report.safe == true).collect::<Vec<&Report>>().len());
    reports2 = check_safety_of_reports_2(reports2);
    println!("Safe reports total for Part 2: {}", reports2.iter().filter(|report| report.safe == true).collect::<Vec<&Report>>().len());

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

fn row_of_integers(input: Vec<String>) -> Vec<Vec<i32>> {

    let mut new_format:Vec<Vec<i32>> = Vec::new();

    for element in input.iter() {
        let mut values: Vec<i32> = Vec::new();
        for item in element.split_whitespace().map(|s| s.parse::<i32>()) {
            match item {
                Ok(a) => {values.push(a)}
                _ => {}
            }
        }
        new_format.push(values);
    }

    return new_format
}

fn convert_to_reports(input:Vec<Vec<i32>>) -> Vec<Report> {
    let mut reports:Vec<Report> = Vec::new();
    for element in input.iter() {
        reports.push(Report::new(element.to_vec()));
    }
    return reports
}

fn check_safety_of_reports(mut input:Vec<Report>) -> Vec<Report> {

    for element in input.iter_mut() {
        element.iterate_and_apply_rules_part1();
    }
    return input.to_vec();
}
fn check_safety_of_reports_2(mut input:Vec<Report>) -> Vec<Report> {

    for element in input.iter_mut() {
        element.iterate_and_apply_rules_part2();
    }
    return input.to_vec();
}


#[derive(Debug, PartialEq, Eq)]
enum LastStep {
    POSITIVE,
    NEGATIVE,
    UNKNOWN
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    raw_values: Vec<i32>,
    safe: bool
}


//I built a struct which turned out to be overkill, but even so, it's neater than lots of giant functions all over the place.
impl Report {
    pub fn new(raw_values: Vec<i32>) -> Self {
        Self {
            raw_values,
            safe: false
        }
    }

    pub fn iterate_and_apply_rules_part1(&mut self) {
        self.safe = Self::part1_rules(self.raw_values.clone());
    }

    pub fn iterate_and_apply_rules_part2(&mut self) {

        //If we're already safe then we don't need to anything special.
        if self.safe == true {
            return
        } 

        //We're going to subset the vector; if we take one index out and can still be safe, then we return. We'll do this for every index of the vector.
        for skip_this in 0..self.raw_values.len() {
            //Literally just get everything from the original vector except one value.
            let subset: Vec<i32> = self.raw_values
            .iter()
            .enumerate()
            .filter(|(pos, _)| *pos != skip_this)
            .map(|(_, &value)| value)
            .collect::<Vec<i32>>();

            if Self::part1_rules(subset) {
                self.safe = true;
                return
            }
        }
        //If even after subsetting we can't prove we're safe, then we are unsafe.
        self.safe = false;
 
    }

    pub fn part1_rules(input: Vec<i32>) -> bool {

        let ascending = input.get(0).unwrap() < input.get(1).unwrap();
        
        for (pos, element) in input.iter().enumerate() {
            if(pos == 0) { continue };
            let difference_to_last: i32 = *element - input.get(pos-1).unwrap();
            if(difference_to_last.abs() > 3 || difference_to_last.abs() < 1) {
                return false;
            }
            if(difference_to_last.is_negative() && ascending) {
                return false;
            } 
            else if difference_to_last.is_positive() && !ascending {
                return false;
            }
        }
        return true;
    }

}