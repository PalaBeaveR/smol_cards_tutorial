use std::{
    fmt::Display,
    io::{self, BufRead},
};

use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
enum CardSuit {
    Spade,
    Heart,
}

impl Display for CardSuit {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        use CardSuit::*;

        write!(
            f,
            "{}",
            match self {
                Spade => '♠',
                Heart => '♥',
            }
        )
    }
}

#[derive(Clone, Debug)]
enum CardFace {
    Number(i8),
    Jack,
    Ace,
}

impl Display for CardFace {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        use CardFace::*;

        write!(
            f,
            "{}",
            match self {
                Ace => "A".into(),
                Jack => "J".into(),
                Number(n) => n.to_string(),
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Card {
    face: CardFace,
    suit: CardSuit,
}

impl Display for Card {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.face)
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut deck = Vec::with_capacity(52);

    {
        use CardSuit::*;

        for suit in [Spade, Heart] {
            for n in 2..=10 {
                deck.push(Card {
                    suit: suit.clone(),
                    face: CardFace::Number(n),
                })
            }

            use CardFace::*;

            for face in [Jack, Ace] {
                deck.push(Card {
                    suit: suit.clone(),
                    face,
                })
            }
        }
    }

    assert_eq!(deck.len(), 52);

    let pristine_deck = deck.clone();

    deck.shuffle(&mut rng);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "draw" => {
                let mut drawn: Vec<String> = (0..6)
                    .map(|_| {
                        deck.remove(
                            rng.gen_range(0..deck.len()),
                        )
                        .to_string()
                    })
                    .collect();

                println!("{}", drawn.join("|"));
            }
            _ => {}
        }
    }
}
