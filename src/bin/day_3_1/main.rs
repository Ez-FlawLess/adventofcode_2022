use std::{fs, collections::HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let data = fs::read_to_string("inputs\\day_3.txt")?;

    let result: u64 = data.lines().map(|rucksack| {
        if (rucksack.len() & 1) == 1 {
            panic!("odd")
        }
        // using hashset is not performent here
        // because the list is short
        let first_compartment = rucksack[..rucksack.len() / 2].to_string();
        
        let mut items = HashSet::with_capacity(first_compartment.len());
        
        for item in first_compartment.bytes() {
            items.insert(item);
        }

        let mut duplicates: HashSet<u64> = HashSet::new();

        let second_compartnemt = rucksack[rucksack.len() / 2..].to_string();

        for item in second_compartnemt.bytes() {
            if let Some(_) = items.get(&item) {
                if item > 96 && item < 123 {
                    duplicates.insert((item - 96) as u64);
                } else if item > 64 && item < 91 {
                    duplicates.insert((item - 38) as u64);
                }
            }
            
        }

        println!("{:?}", duplicates);

        duplicates.into_iter().sum::<u64>()
    })
    .sum();

    println!("{}", result);
    
    Ok(())
}