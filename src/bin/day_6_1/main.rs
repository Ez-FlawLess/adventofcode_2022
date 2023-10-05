use std::fs;

// PART 1 = 4, PART 2 = 14
const WINDOW_SIZE: usize = 14;

fn main() {
    
    let data = fs::read_to_string("inputs\\day_6.txt").unwrap();

    let result = data.as_bytes().windows(WINDOW_SIZE).position(|window| {
        let mut hold = Vec::with_capacity(WINDOW_SIZE);
        for char in window {
            if hold.contains(char) {
                return false;
            }
            hold.push(*char)
        }
        true
    });

    println!("{}", result.unwrap() + WINDOW_SIZE);
}
