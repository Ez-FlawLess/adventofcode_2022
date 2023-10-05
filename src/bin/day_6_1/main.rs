use std::fs;

fn main() {
    
    let data = fs::read_to_string("inputs\\day_6.txt").unwrap();

    let result = data.as_bytes().windows(4).position(|window| {
        let mut hold = Vec::with_capacity(4);
        for char in window {
            if hold.contains(char) {
                return false;
            }
            hold.push(*char)
        }
        true
    });

    println!("{}", result.unwrap() + 4);
}
