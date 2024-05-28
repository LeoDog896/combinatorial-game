//! Nim is the heart of impartial combinatorial game theory.
//! Its a game about removing objects from heaps.
//! Despite its ability to be rigidly analyzed,
//! it still makes a great example as an implementation of the `Game` trait.
//!
//! Learn more about Nim here: <https://en.wikipedia.org/wiki/Nim>

pub mod cli;

use game_solver::game::{Game, ZeroSumPlayer};
use std::hash::Hash;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Nim {
    heaps: Vec<usize>,
    move_count: usize,
    max_score: usize,
}

impl Nim {
    /// Create a new game of Nim with the given heaps,
    /// where heaps is a list of the number of objects in each heap.
    fn new(heaps: Vec<usize>) -> Self {
        Self {
            heaps: heaps.clone(),
            move_count: 0,
            // sum of all the heaps is the upper bound for the amount of moves
            max_score: heaps.iter().sum::<usize>(),
        }
    }
}

impl Game for Nim {
    /// where Move is a tuple of the heap index and the number of objects to remove
    type Move = (usize, usize);
    type Iter<'a> = std::vec::IntoIter<Self::Move>;
    /// Define Nimbers as a zero-sum game
    type Player = ZeroSumPlayer;

    fn max_moves(&self) -> Option<usize> {
        Some(self.max_score)
    }

    fn player(&self) -> ZeroSumPlayer {
        if self.move_count % 2 == 0 {
            ZeroSumPlayer::One
        } else {
            ZeroSumPlayer::Two
        }
    }

    fn move_count(&self) -> usize {
        self.move_count
    }

    fn make_move(&mut self, m: &Self::Move) -> bool {
        let (heap, amount) = *m;
        // check for indexing OOB
        if heap >= self.heaps.len() {
            return false;
        }

        // check for removing too many objects
        if amount > self.heaps[heap] {
            return false;
        }

        self.heaps[heap] -= amount;
        self.move_count += 1;
        true
    }

    fn possible_moves(&self) -> Self::Iter<'_> {
        let mut moves = Vec::new();

        // loop through every heap and add every possible move
        for (i, &heap) in self.heaps.iter().enumerate() {
            for j in 1..=heap {
                moves.push((i, j));
            }
        }

        moves.into_iter()
    }

    // a move is winning if the next player
    // has no possible moves to make (normal play for Nim)
    fn is_winning_move(&self, m: &Self::Move) -> Option<Self::Player> {
        let mut board = self.clone();
        board.make_move(m);
        // next player can't play - this player won!
        if board.possible_moves().next().is_none() {
            Some(self.player())
        } else {
            None
        }
    }

    // Nim can never be a draw -
    // if there are no possible moves, the game is already won
    fn is_draw(&self) -> bool {
        false
    }
}