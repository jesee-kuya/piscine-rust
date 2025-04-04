use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("invalid"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(1..=13);

        match n {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Number(n),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("invalid"),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    let winner = Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    };
    winner == card
}
