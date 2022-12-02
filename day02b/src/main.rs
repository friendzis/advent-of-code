use std::str::FromStr;

#[derive(Debug)]
enum HandSign {
    ROCK,
    PAPER,
    SCISSORS,
}

impl FromStr for HandSign {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "X" => Ok(HandSign::ROCK),
            // "Y" => Ok(HandSign::PAPER),
            // "Z" => Ok(HandSign::SCISSORS),
            "A" => Ok(HandSign::ROCK),
            "B" => Ok(HandSign::PAPER),
            "C" => Ok(HandSign::SCISSORS),
            _   => Err(()),
        }
    }
}

enum RoundOutcome {
    WIN,
    LOSS,
    DRAW
}

impl FromStr for RoundOutcome {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundOutcome::LOSS),
            "Y" => Ok(RoundOutcome::DRAW),
            "Z" => Ok(RoundOutcome::WIN),
            _   => Err(())
        }
    }
}

fn round_score(theirs: HandSign, outcome: RoundOutcome) -> i32 {
    let ours: HandSign = match outcome {
        RoundOutcome::WIN   => match theirs {
            HandSign::ROCK      =>  HandSign::PAPER,
            HandSign::PAPER     =>  HandSign::SCISSORS,
            HandSign::SCISSORS  =>  HandSign::ROCK
        },
        RoundOutcome::LOSS   => match theirs {
            HandSign::ROCK      =>  HandSign::SCISSORS,
            HandSign::PAPER     =>  HandSign::ROCK,
            HandSign::SCISSORS  =>  HandSign::PAPER
        },
        RoundOutcome::DRAW   => theirs
    };
    
    let match_score = match outcome {
        RoundOutcome::WIN   =>  6,
        RoundOutcome::DRAW  =>  3,
        RoundOutcome::LOSS  =>  0
    };
    let hand_score = match ours {
        HandSign::ROCK      => 1,
        HandSign::PAPER     => 2,
        HandSign::SCISSORS  => 3
    };

    match_score + hand_score
}

fn main() {
    let input = include_str!("../input");
    let plays: _ = 
    input
        .split("\n")
        .map(|play| {
            play.split(" ").collect::<Vec<&str>>()
        })
        .map(|play| round_score(HandSign::from_str(play[0]).unwrap(), RoundOutcome::from_str(&play[1]).unwrap()))
        .sum::<i32>()
        ;
    println!("{:?}", plays);
}
