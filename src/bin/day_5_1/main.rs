use std::fs;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    
    let data = fs::read_to_string("inputs\\day_5.txt").unwrap();
    let mut lines = data.lines();

    let mut cargo: Vec<Vec<u8>>;

    {
        let cargo_lines = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

        let mut cargo_lines_iter = cargo_lines.iter().rev();

        cargo = cargo_lines_iter.next().unwrap()
            .as_bytes()
            .chunks(4)
            .map(|_| Vec::new())
            .collect::<Vec<Vec<u8>>>();

        for line in cargo_lines_iter {
            for (index, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk[1].is_ascii_whitespace() {
                    continue;
                }
                cargo[index].push(chunk[1]);
            }
        }
    }

    for line in lines {
        let m = line.split(' ')
            .enumerate()
            .filter_map(|(index, part)| {
                if index & 1 == 1 {
                    Some(part.parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let m = Move{
            count: m[0],
            from: m[1] - 1,
            to: m[2] - 1,
        };
            
        // Part 1
        // for _ in 0..m.count {
            
            // let c = cargo[m.from].pop().unwrap();
            // cargo[m.to].push(c);
        // }
        // Part 2
        let mut holder = Vec::with_capacity(m.count);
        for _ in 0..m.count {
            holder.push(cargo[m.from].pop().unwrap());
        }
        for _ in 0..m.count {
            cargo[m.to].push(holder.pop().unwrap());
        }

    }

    let result = cargo.iter()
        .map(|stack| {
            *stack.last().unwrap()
        })
        .collect::<Vec<u8>>();

    unsafe {
        println!("{:?}", std::str::from_utf8_unchecked(&result));
    }
    
}