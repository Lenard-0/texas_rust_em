use crate::architecture::{GameState, Player, Action};

// All poker games revolve around betting, and it's important to understand how betting rules work before getting into the game, no matter what poker variation you're playing.

// how to bet in poker

// Many poker variations use the same betting structure and table positions. Texas Hold'em, Pot-Limit Omaha, and many limit poker games all use a system involving the small blind, big blind, and dealer button, with all other table positions relative to those three spots.

// The small blind is always seated to the left of the dealer button, and the big blind to the left of the small blind.

// Betting Order
// In games that use a blinds system, the first round of betting usually starts with the player to the left of the big blind. After that player acts, the action moves clockwise around the table, until all players have the chance to act.

// Some games use a system of antes (a forced bet put in by every player at the table) and a bring-in. The bring-in system usually designates the player with the weakest face up card as the first player to act. After that, the action moves clockwise around the table.

// Virtually all poker games allow the active player to choose from four different betting actions when they're the bettor:

// The Actions
// Call (matching the amount of the previous bet or raise).
// Raise (increase the amount of the current open bet or raise, which any subsequent players must at least match to stay in. Raising when a player in front of you has already raised is known as a re-raise).
// Fold (pushing their cards into the middle and surrendering any chance to win the hand).
// Check (pass the action to the next player without betting anything. Checking can only be used when there's no open bet or raise in front of you.
// Blinds and Antes
// Just about all poker games use some kind of forced bet, which automatically puts money in the main pot before each hand. Texas Hold'em, Pot-Limit Omaha, and many other poker variations use a small blind and big blind as the forced bets.

// Let's say you're playing online poker and see a cash game listed as a $1/$2 NLHE game. The $1/$2 listing means the game uses $1 as the small blind amount and $2 as the amount of the big blind.

// In most poker games, the minimum bet allowed at any given time is equivalent to the amount of the big blind.

// Antes are sometimes included in games that use blinds, but some games are ante-only. Antes generally function as small forced bets that go in from every player, or sometimes only the big blind player.

// For more on how forced bets work, click on the "Game Structure" tab above.

// for each player, starting from the small blind, ask them what they want to do
// then the player chooses an action
// if the action is fold => remove the player from the game
// if the action is call => remove that money from their stack and add it to the pot
// if the action is raise => remove that money from their stack and add it to the pot, then change the bet fulfullment index to the current player
// if the action is check => do nothing
// if the action is all in => remove all of their money from their stack and add it to the pot, then change the bet fulfullment index to the current player
// if the bet fulfillment index is equal to the current player, then the bet round is over
pub fn bet_round(mut state: GameState) -> GameState {
    let mut bet_round_fulfilled = false;
    let bet_fulfillment_index = state.current_player;
    while !bet_round_fulfilled {
        let current_player = &mut state.players[state.current_player];

        if state.current_player == bet_fulfillment_index {
            break;
        }

        if current_player.folded {
            state.current_player += 1;
            continue;
        }

        current_player.take_betting_action();

        match current_player.current_action {
            Action::Fold => {
                current_player.folded = true;
            }
            Action::Call => {
                let amount_to_call = state.current_bet - current_player.current_bet;
                current_player.chips -= amount_to_call;
                state.pot += amount_to_call;
            }
            Action::Raise(raise_amount) => {
                let amount_to_call = state.current_bet - current_player.current_bet;
                current_player.chips -= amount_to_call + raise_amount;
                state.pot += raise_amount + amount_to_call;
                state.current_bet += raise_amount;
                state.current_player = bet_fulfillment_index;
            }
            Action::Check => {},
            Action::AllIn => {
                let amount_to_all_in = current_player.chips;
                current_player.chips = 0;
                state.pot += amount_to_all_in;
                state.current_player = bet_fulfillment_index;
            }
        };

        state.current_player += 1;
    }

    return state
}

impl Player {
    pub fn take_betting_action(&mut self) {
        if self.is_human {
            // ask the player what they want to do
            // then do it
        } else {
            // do the ai
        }
        unimplemented!()
    }
}
