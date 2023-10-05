use std::fs;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Move {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => {
                Ok(Self::Rock)
            },
            b'B' => {
                Ok(Self::Paper)
            },
            b'C' => {
                Ok(Self::Scissors)
            },
            _ => {
                Err(())
            }
        }
    }
}

enum MatchResult {
    Win,
    Draw,
    Lose,
}

impl TryFrom<u8> for MatchResult {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'Z' => {
                Ok(Self::Win)
            },
            b'Y' => {
                Ok(Self::Draw)
            },
            b'X' => {
                Ok(Self::Lose)
            },
            _ => {
                Err(())
            }
        }
    }
}

const SCORE_LIST: [usize; 5] = [
    Move::Rock as usize, 
    Move::Paper as usize, 
    Move::Scissors as usize, 
    Move::Rock as usize, 
    Move::Paper as usize
];
/// tuple index 0 is the score added because of result.
/// index 1 is the needed number added to Move index to get our move
const MATCH_RESULT_LIST: [(usize, usize); 3] = [(6, 1), (3, 0), (0, 2)];
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let data = fs::read_to_string("inputs\\day_2.txt")?;
    
    let total_score = data.lines()
        .map(|game| {
            let game = game.as_bytes();
            let their_pick = Move::try_from(game[0]).unwrap() as usize;
            let math_result = MATCH_RESULT_LIST[MatchResult::try_from(game[2]).unwrap() as usize];

            math_result.0 + (SCORE_LIST[their_pick + math_result.1] as usize) + 1
        })
        .sum::<usize>();

    println!("{}", total_score);
    
    Ok(())
}