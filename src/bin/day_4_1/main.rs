use std::fs;

fn main() {
    
    let data = fs::read_to_string("inputs\\day_4.txt").unwrap();

    let result = data.lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(',').collect();
            let pair_1 = get_pair(pair[0]);
            let pair_2 = get_pair(pair[1]);

            ((pair_1[0], pair_1[1]), (pair_2[0], pair_2[1]))
        })
        .filter(|(pair_1, pair_2)| {
            // Part 1
            // if pair_1.0 <= pair_2.0 && pair_1.1 >= pair_2.1 {
            //     true
            // } else if pair_2.0 <= pair_1.0 && pair_2.1 >= pair_1.1 {
            //     true
            // } else {
            //     false
            // }
            // Part 2
            if pair_1.0 >= pair_2.0 && pair_1.0 <= pair_2.1 {
                true
            } else if pair_2.0 >= pair_1.0 && pair_2.0 <= pair_1.1 {
                true
            } else {
                false
            }
        })
        .count();

    println!("{}", result);
}

fn get_pair(rng: &str) -> Vec<u16> {
    rng.split('-')
        .map(|s| {
            s.parse::<u16>().unwrap()
        })
        .collect()
}