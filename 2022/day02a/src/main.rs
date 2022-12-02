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
            "X" => Ok(HandSign::ROCK),
            "Y" => Ok(HandSign::PAPER),
            "Z" => Ok(HandSign::SCISSORS),
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

fn round_outcome(theirs: &HandSign, ours: &HandSign) -> RoundOutcome {
    match ours {
        HandSign::ROCK => match theirs {
            HandSign::ROCK      => RoundOutcome::DRAW,
            HandSign::PAPER     => RoundOutcome::LOSS,
            HandSign::SCISSORS  => RoundOutcome::WIN,
        },
        HandSign::PAPER => match theirs {
            HandSign::ROCK      => RoundOutcome::WIN,
            HandSign::PAPER     => RoundOutcome::DRAW,
            HandSign::SCISSORS  => RoundOutcome::LOSS,
        },
        HandSign::SCISSORS => match theirs {
            HandSign::ROCK      => RoundOutcome::LOSS,
            HandSign::PAPER     => RoundOutcome::WIN,
            HandSign::SCISSORS  => RoundOutcome::DRAW,
        },
    }
}

fn round_score(theirs: &HandSign, ours: &HandSign) -> i32 {
    let hand_score = match ours {
        HandSign::ROCK      => 1,
        HandSign::PAPER     => 2,
        HandSign::SCISSORS  => 3
    };
    let match_score = match round_outcome(theirs, ours) {
        RoundOutcome::WIN   =>  6,
        RoundOutcome::DRAW  =>  3,
        RoundOutcome::LOSS  =>  0
    };
    hand_score + match_score
}

fn main() {
    let input = include_str!("../input");
    let plays: _ = 
    input
        .split("\n")
        .map(|play| {
            play.split(" ")
            .map(|item| {
                HandSign::from_str(item).unwrap()
            })
            .collect::<Vec<HandSign>>()
        })
        .map(|play| round_score(&play[0], &play[1]))
        .sum::<i32>()
        ;
    println!("{:?}", plays);
}
