use std::fs;

fn main() {
    let input: String = fs::read_to_string("./day_1/src/resources/input1.txt").expect("Read that file");
    let mut calibration_values: Vec<u32> = Vec::new();

    for line in input.split("\n") {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for char in line.chars() {
            if char.is_numeric() {
                if first == None {
                    first = Some(char);
                } else {
                    last = Some(char);
                }
            }
        }

        if last == None {
            last = first.clone();
        }

        let value :String = first.unwrap().to_string() + &*last.unwrap().to_string();
        calibration_values.push(value.parse::<u32>().unwrap())
    }

    let calibration_sum: u32 = calibration_values.iter().sum();
    println!("{}", calibration_sum);
}
