//! `game-solver` is a library for solving games.
//!
//! If you want to read how to properly use this library,
//! [the book](https://leodog896.github.io/game-solver/book) is
//! a great place to start.

use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
};

/// Represents a player in a two-player combinatorial game.
#[derive(PartialEq, Eq, Debug)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    /// Get the player opposite to this one.
    #[must_use]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
        }
    }
}

/// Represents a combinatorial game.
pub trait Game {
    /// The type of move this game uses.
    type Move: Clone;

    /// The iterator type for possible moves.
    type Iter<'a>: Iterator<Item = Self::Move> + 'a
    where
        Self: 'a;

    /// Returns the player whose turn it is.
    fn player(&self) -> Player;

    /// Scores a position. The default implementation uses the size minus the number of moves (for finite games)
    fn score(&self) -> u32;

    /// Get the max score of a game.
    fn max_score(&self) -> u32;

    /// Get the min score of a game. This should be negative.
    fn min_score(&self) -> i32;

    /// Returns true if the move was valid, and makes the move if it was.
    fn make_move(&mut self, m: Self::Move) -> bool;

    /// Returns a vector of all possible moves.
    ///
    /// If possible, this function should "guess" what the best moves are first.
    /// For example, if this is for tic tac toe, it should give the middle move first.
    /// This allows alpha-beta pruning to move faster.
    fn possible_moves(&self) -> Self::Iter<'_>;

    /// Returns true if the move is a winning move.
    fn is_winning_move(&self, m: Self::Move) -> bool;

    /// Returns true if the game is a draw.
    fn is_draw(&self) -> bool;
}

/// A memoization strategy for a perfect-information sequential game.
///
/// Transposition tables should optimally be a form of hash table.
pub trait TranspositionTable<T: Eq + Hash + Game> {
    fn get(&self, board: &T) -> Option<i32>;
    fn insert(&mut self, board: T, score: i32);
    fn has(&self, board: &T) -> bool;
}

impl<K: Eq + Hash + Game, S: BuildHasher + Default> TranspositionTable<K> for HashMap<K, i32, S> {
    fn get(&self, board: &K) -> Option<i32> {
        self.get(board).copied()
    }

    fn insert(&mut self, board: K, score: i32) {
        self.insert(board, score);
    }

    fn has(&self, board: &K) -> bool {
        self.contains_key(board)
    }
}

/// Runs the two-player minimax variant on a game.
/// It uses alpha beta pruning (e.g. you can specify \[-1, 1\] to get only win/loss/draw moves).
///
/// This function requires a transposition table. If you only plan on running this function once,
/// you can use a the in-built `HashMap`.
fn negamax<T: Game + Clone + Eq + Hash>(
    game: &T,
    transposition_table: &mut dyn TranspositionTable<T>,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if game.is_draw() {
        return 0;
    }

    for m in &mut game.possible_moves() {
        if game.is_winning_move(m.clone()) {
            let mut board = game.clone();
            board.make_move(m);
            return board.score() as i32;
        }
    }

    {
        let max = transposition_table
            .get(game)
            .unwrap_or(game.max_score() as i32);
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }
    }

    for m in &mut game.possible_moves() {
        let mut board = game.clone();
        board.make_move(m);

        let score = -negamax(&board, transposition_table, -beta, -alpha);

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    transposition_table.insert(game.clone(), alpha);

    alpha
}

/// Solves a game, returning the evaluated score.
///
/// The score of a position is defined by the best possible end result for the player whose turn it is.
/// In 2 player games, if a score > 0, then the player whose turn it is has a winning strategy.
/// If a score < 0, then the player whose turn it is has a losing strategy.
/// Else, the game is a draw.
pub fn solve<T: Game + Clone + Eq + Hash>(
    game: &T,
    transposition_table: &mut dyn TranspositionTable<T>,
) -> i32 {
    let min = game.min_score();
    let max = game.max_score() as i32 + 1;

    let mut alpha = min;
    let mut beta = max;

    while alpha < beta {
        let med = alpha + (beta - alpha) / 2;
        let r = negamax(game, transposition_table, med, med + 1);

        if r <= med {
            beta = r;
        } else {
            alpha = r;
        }
    }

    alpha
}

/// Utility function to get a list of the move scores of a certain game.
///
/// This is mainly intended for front-facing visual interfaces
/// for each move.
pub fn move_scores<'a, T: Game + Clone + Eq + Hash>(
    game: &'a T,
    transposition_table: &'a mut dyn TranspositionTable<T>,
) -> impl Iterator<Item = (<T as Game>::Move, i32)> + 'a {
    game.possible_moves().map(move |m| {
        let mut board = game.clone();
        board.make_move(m.clone());
        // We flip the sign of the score because we want the score from the
        // perspective of the player playing the move, not the player whose turn it is.
        (m, -solve(&board, transposition_table))
    })
}
