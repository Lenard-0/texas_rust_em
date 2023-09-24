
#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub chips: u32,
    pub folded: bool,
    pub all_in: bool,
    pub current_bet: u32,
    pub current_action: Action,
    pub is_human: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Card {
    Ace(CardSuit),
    Two(CardSuit),
    Three(CardSuit),
    Four(CardSuit),
    Five(CardSuit),
    Six(CardSuit),
    Seven(CardSuit),
    Eight(CardSuit),
    Nine(CardSuit),
    Ten(CardSuit),
    Jack(CardSuit),
    Queen(CardSuit),
    King(CardSuit)
}

#[derive(Debug, Clone, Copy)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Fold,
    Check,
    Call,
    Raise(u32),
    AllIn,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub pot: u32,
    pub current_bet: u32,
    pub current_action: Action,
    pub current_player: usize,
    pub dealer: usize,
    pub small_blind: usize,
    pub big_blind: usize,
    pub community_cards: Vec<Card>,
    pub game_over: bool,
}

impl GameState {
    pub fn initialise() -> Self {
        let mut deck = Vec::new();
        for suit in [CardSuit::Hearts, CardSuit::Diamonds, CardSuit::Clubs, CardSuit::Spades].iter() {
            deck.push(Card::Ace(*suit));
            deck.push(Card::Two(*suit));
            deck.push(Card::Three(*suit));
            deck.push(Card::Four(*suit));
            deck.push(Card::Five(*suit));
            deck.push(Card::Six(*suit));
            deck.push(Card::Seven(*suit));
            deck.push(Card::Eight(*suit));
            deck.push(Card::Nine(*suit));
            deck.push(Card::Ten(*suit));
            deck.push(Card::Jack(*suit));
            deck.push(Card::Queen(*suit));
            deck.push(Card::King(*suit));
        }
        let mut players = Vec::new();

        let mut is_human = true;
        for _ in 0..6 {
            players.push(Player {
                hand: Vec::new(),
                chips: 1000,
                folded: false,
                all_in: false,
                current_bet: 0,
                current_action: Action::Check,
                is_human,
            });
            is_human = false;
        }

        return GameState {
            players,
            deck,
            pot: 0,
            current_bet: 0,
            current_action: Action::Check,
            current_player: 0,
            dealer: 0,
            small_blind: 1,
            big_blind: 2,
            community_cards: Vec::new(),
            game_over: false,
        }
    }
}