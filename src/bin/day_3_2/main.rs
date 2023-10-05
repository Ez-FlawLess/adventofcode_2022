use std::fs;

fn main() {
    
    let data = fs::read_to_string("inputs\\day_3.txt").unwrap();

    let result = data.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            // 52 = number of letters
            let mut letters = Vec::with_capacity(group[0].len());

            for letter in group[0].bytes() {
                if !letters.contains(&letter) {
                    letters.push(letter);
                }
            }

            for rucksack in &group[1..] {
                let mut remain = Vec::with_capacity(letters.len());

                for letter in rucksack.bytes() {
                    
                    if !remain.contains(&letter) && letters.contains(&letter) {
                        remain.push(letter);
                    }
                    
                }

                letters = remain;
            }

            let result = letters[0];

            println!("{}", result);

            if result > 96 && result < 123 {
                (result - 96) as u64
            } else if result > 64 && result < 91 {
                (result - 38) as u64
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("{}", result);

}