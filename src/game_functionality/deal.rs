use crate::architecture::GameState;


pub fn deal_cards(mut state: GameState) -> GameState {
    for player in state.players.iter_mut() {
        player.hand.push(state.deck.pop().unwrap());
        player.hand.push(state.deck.pop().unwrap());
    }

    return state
}