use std::fs;

#[derive(PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Move {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => {
                Ok(Self::Rock)
            },
            b'B' | b'Y' => {
                Ok(Self::Paper)
            },
            b'C' | b'Z' => {
                Ok(Self::Scissors)
            },
            _ => {
                Err(())
            }
        }
    }
}

const SCORE_LIST: [Move; 4] = [Move::Rock, Move::Paper, Move::Scissors, Move::Rock];

impl Move {

    fn match_score(&self, other: Move) -> u32 {
        if *self == other {
            3
        } else {
            if *self == SCORE_LIST[other as usize] {
                6
            } else {
                0
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let data = fs::read_to_string("inputs\\day_2.txt")?;

    let total_score = data.lines()
        .map(|game| {
            let game = game.as_bytes();
            let their_pick = Move::try_from(game[0]).unwrap();
            let my_pick = Move::try_from(game[2]).unwrap();
            
            my_pick.match_score(their_pick) + my_pick as u32
        })
        .sum::<u32>();

    println!("{}", total_score);

    Ok(())
}