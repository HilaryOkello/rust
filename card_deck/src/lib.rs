use rand::Rng;

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Option<Self> {
        match value {
            1 => Some(Suit::Heart),
            2 => Some(Suit::Diamond),
            3 => Some(Suit::Spade),
            4 => Some(Suit::Club),
            _ => None,
        }
    }
}

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(1..=13) {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }

    pub fn translate(value: u8) -> Option<Self> {
        match value {
            1 => Some(Rank::Ace),
            n @ 2..=10 => Some(Rank::Number(n)),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            _ => None,
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match (&card.suit, &card.rank) {
        (Suit::Spade, Rank::Ace) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_random() {
        for _ in 0..100 {
            let suit = Suit::random();
            match suit {
                Suit::Heart | Suit::Diamond | Suit::Spade | Suit::Club => assert!(true),
            }
        }
    }

    #[test]
    fn test_suit_translate() {
        assert!(matches!(Suit::translate(1), Some(Suit::Heart)));
        assert!(matches!(Suit::translate(2), Some(Suit::Diamond)));
        assert!(matches!(Suit::translate(3), Some(Suit::Spade)));
        assert!(matches!(Suit::translate(4), Some(Suit::Club)));
        assert!(Suit::translate(0).is_none());
        assert!(Suit::translate(5).is_none());
    }

    #[test]
    fn test_rank_random() {
        for _ in 0..100 {
            let rank = Rank::random();
            match rank {
                Rank::Ace | Rank::King | Rank::Queen | Rank::Jack | Rank::Number(..) => assert!(true),
            }
        }
    }

    #[test]
    fn test_rank_translate() {
        assert!(matches!(Rank::translate(1), Some(Rank::Ace)));
        assert!(matches!(Rank::translate(2), Some(Rank::Number(2))));
        assert!(matches!(Rank::translate(10), Some(Rank::Number(10))));
        assert!(matches!(Rank::translate(11), Some(Rank::Jack)));
        assert!(matches!(Rank::translate(12), Some(Rank::Queen)));
        assert!(matches!(Rank::translate(13), Some(Rank::King)));
        assert!(Rank::translate(0).is_none());
        assert!(Rank::translate(14).is_none());
    }

    #[test]
    fn test_winner_card() {
        let ace_of_spades = Card { suit: Suit::Spade, rank: Rank::Ace };
        let king_of_hearts = Card { suit: Suit::Heart, rank: Rank::King };

        assert!(winner_card(&ace_of_spades));
        assert!(!winner_card(&king_of_hearts));
    }
}