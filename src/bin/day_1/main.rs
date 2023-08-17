use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let data = fs::read_to_string("inputs\\day_1.txt")?;

    let mut elves = vec![0];

    for line in data.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {

            let last = elves.last_mut().unwrap();
            *last += line.parse::<i32>()?;
        }
    }

    let mut top_3 = [elves[0], elves[1], elves[2]];

    top_3.sort_by(|a, b| b.cmp(a));

    println!("{:?}", top_3);

    for calories in elves[3..].into_iter() {
        let mut current = *calories;
        for max in top_3.iter_mut() {
            if current > *max {
                let holder = current;
                current = *max;
                *max = holder;
            }
        }
    }

    println!("{}", top_3.iter().sum::<i32>());

    Ok(())
}