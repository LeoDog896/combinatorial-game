pub mod util;

pub mod chomp;
pub mod domineering;
pub mod nim;
pub mod order_and_chaos;
pub mod reversi;
pub mod tic_tac_toe;

use clap::Subcommand;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::{
    chomp::cli::ChompArgs,
    domineering::cli::DomineeringArgs,
    nim::cli::NimArgs,
    order_and_chaos::cli::OrderAndChaosArgs,
    reversi::cli::ReversiArgs,
    tic_tac_toe::cli::TicTacToeArgs,
};

#[derive(Subcommand, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Games {
    Reversi(ReversiArgs),
    TicTacToe(TicTacToeArgs),
    OrderAndChaos(OrderAndChaosArgs),
    Nim(NimArgs),
    Domineering(DomineeringArgs),
    Chomp(ChompArgs),
}

pub static DEFAULT_GAMES: Lazy<[Games; 6]> = Lazy::new(|| [
    Games::Reversi(Default::default()),
    Games::TicTacToe(Default::default()),
    Games::OrderAndChaos(Default::default()),
    Games::Nim(Default::default()),
    Games::Domineering(Default::default()),
    Games::Chomp(Default::default()),
]);

impl Games {
    pub fn name(&self) -> String {
        match self {
            &Self::Reversi(_) => "Reversi".to_string(),
            &Self::TicTacToe(_) => "Tic Tac Toe".to_string(),
            &Self::OrderAndChaos(_) => "Order and Chaos".to_string(),
            &Self::Nim(_) => "Nim".to_string(),
            &Self::Domineering(_) => "Domineering".to_string(),
            &Self::Chomp(_) => "Chomp".to_string(),
        }
    }
}
