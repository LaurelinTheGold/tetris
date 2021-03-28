use std::collections::VecDeque;

use crate::piece::{TetEnum, TetRotFull, TetRotTraits, Tetromino, TET_BOX_SIZE};
use rand::prelude::*;
use rand::seq::SliceRandom;
/// bits 0 through 2 are for padding, then 10 (3 through 12) for board itself
pub(crate) type BoardRow = u16;
const BOARD_HEIGHT_SHOWN: usize = 20;
type GameBoard = [BoardRow; BOARD_HEIGHT_SHOWN + TET_BOX_SIZE + TET_BOX_SIZE - 1];
type BagOfPieces = Vec<TetEnum>;

enum MoveCommand {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    CLOCKWISE,
    ANTICLOCKWISE,
}
// <--- x, 2,1,0 are empty
struct GameState {
    board: GameBoard,
    curr_tetromino: Tetromino,
    score: usize,
    level: usize,
    piece_bag: BagOfPieces,
}
trait PieceBagTraits {
    fn draw_piece(&mut self) -> Option<TetEnum>; // draw, pop will draw from back, none if empty
    fn shuffle(&mut self) -> (); // shuffle 4x 0-6
    fn make_self() -> Self; // make new piece bag
}
trait GameStateTraits {
    fn make_piece_bag(&mut self) -> (); //
    fn get_new_tetromino(&mut self) -> (); // just draw piece, new piecebag if emtpy, generate tetromino from value
    fn fix_piece_to_board(&mut self) -> (); // piece at bottom so update board, draw new piece/make new bag if needed
    fn collision_free(&self) -> bool; // gets the boardboxrows from currpiece and checks against board for collisions
    fn do_move(&mut self, move_action: MoveCommand) -> (); // does the command to move the piece
    fn advance_timestep(&mut self) -> (); // move down, cond checks
    fn clear_rows(&mut self) -> usize; // clears full rows, collapses board, returns nubmer of rows cleared
    fn cond_checks(&mut self) -> (); // clear rows, update score, level?
}

impl GameState {
    fn new(
        board: GameBoard,
        curr_tetromino: Tetromino,
        score: usize,
        level: usize,
        piece_bag: BagOfPieces,
    ) -> Self {
        Self {
            board,
            curr_tetromino,
            score,
            level,
            piece_bag,
        }
    }
}

impl PieceBagTraits for BagOfPieces {
    fn draw_piece(&mut self) -> Option<TetEnum> {
        self.pop()
    }

    fn shuffle(&mut self) -> () {
        let mut rng = rand::thread_rng();
        &self[..].shuffle(&mut rng);
    }

    fn make_self() -> Self {
        let mut bag = Vec::new();
        for i in 0..7 {
            for _ in 0..4 {
                bag.push(i);
            }
        }
        bag
    }
}
impl GameStateTraits for GameState {
    fn make_piece_bag(&mut self) -> () {
        let mut tmp = BagOfPieces::make_self();
        tmp.shuffle();
        self.piece_bag = tmp;
    }

    fn get_new_tetromino(&mut self) -> () {
        match self.piece_bag.draw_piece() {
            Some(a) => {
                self.curr_tetromino
                    .set_piece_bits(TetRotFull::make_from_idx(a as usize).unwrap());
            }
            None => {
                self.make_piece_bag();
                self.get_new_tetromino()
            }
        }
    }

    fn fix_piece_to_board(&mut self) -> () {
        todo!()
    }

    fn collision_free(&self) -> bool {
        todo!()
    }

    fn do_move(&mut self, move_action: MoveCommand) -> () {
        match move_action {
            MoveCommand::LEFT => {}
            MoveCommand::RIGHT => {}
            MoveCommand::UP => {}
            MoveCommand::DOWN => {}
            MoveCommand::CLOCKWISE => {}
            MoveCommand::ANTICLOCKWISE => {}
        }
        self.cond_checks()
    }

    fn advance_timestep(&mut self) -> () {
        self.do_move(MoveCommand::DOWN);
        self.cond_checks()
    }

    fn clear_rows(&mut self) -> usize {
        todo!()
    }

    fn cond_checks(&mut self) -> () {
        self.score += self.clear_rows();
    }
}
