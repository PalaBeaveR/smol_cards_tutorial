use std::{
    fmt::Display,
    io::{self, BufRead},
};

use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
enum CardSuit {
    Spade,
    Diamond,
    Heart,
    Club,
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
                Diamond => '♦',
                Club => '♣',
            }
        )
    }
}

#[derive(Clone, Debug)]
enum CardFace {
    Number(i8),
    Jack,
    Queen,
    King,
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
                King => "K".into(),
                Queen => "Q".into(),
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

        for suit in [Spade, Heart, Diamond, Club] {
            for n in 2..=10 {
                deck.push(Card {
                    suit: suit.clone(),
                    face: CardFace::Number(n),
                })
            }

            use CardFace::*;

            for face in [Jack, Queen, King, Ace] {
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
