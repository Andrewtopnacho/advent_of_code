fn main() {
    let input = std::fs::read_to_string("C:/Users/Andrew/Code/rust/advent_of_code/day1/input.txt").expect("Failed to read file");

    for line in input.lines() {
        let calibration_value = decode_line(line);
    }
}
fn decode_line(line: &str) -> usize {
    let chars = line.chars().collect::<Vec<char>>();
    let mut numbers = Vec::new();
    for i in 0..chars.len() {
        if let Some(number) = chars[i].to_digit(10) {
            numbers.push(number);
            continue;
        }

        let slice = &line[i..];
        println!("{}", slice)
        // If slice starts with "one", "two", etc. Then push the corresponding number and increment i depending on the length of the number string. 
    }

    let first = numbers.iter().nth(0).expect("line should not be empty");
    let last = numbers.iter().last().expect("line should not be empty");

    println!("{}{}: {}", first, last, line);
    
    0
}