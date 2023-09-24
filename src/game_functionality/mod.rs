use crate::architecture::GameState;

use self::{shuffle::shuffle_deck, deal::deal_cards, bet::bet_round};

pub mod shuffle;
pub mod deal;
pub mod bet;

pub fn play_game(mut state: GameState) {
    while !state.game_over {
        state.community_cards = vec![];
        state.current_bet = state.big_blind as u32;
        state.pot = (state.big_blind + state.small_blind) as u32;

        state.deck = shuffle_deck(state.deck);
        state = deal_cards(state);
        state = bet_round(state);
    }
}