
use crate::architecture::Card;
use rand::seq::SliceRandom;


pub fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    deck.shuffle(&mut rand::thread_rng());
    deck
}