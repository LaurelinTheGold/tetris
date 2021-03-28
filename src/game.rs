use std::{borrow::BorrowMut, convert::TryInto, io::Write};

use tetromino::Tetromino;

use crate::tetromino::{self, TetVar, TetrominoOps, TETRO_BOX};

pub(crate) const BOARD_HEIGHT: usize = 24;
const BOARD_HEIGHT_EXTEND: usize = BOARD_HEIGHT + TETRO_BOX - 1;
pub(crate) const BOARD_HEIGHT_SHOWN: usize = 20;
/// Number of Columns
pub(crate) const BOARD_WIDTH: usize = 10;
/// 2 ^ 10 - 1
const FULL_ROW: u16 = 1023;

pub(crate) struct GameState<W>
where
    W: Write,
{
    board: [u16; BOARD_HEIGHT_EXTEND],
    score: u16,
    write: W,
    game_over: bool,
    curr_piece: Tetromino,
}

impl<W> GameState<W>
where
    W: Write,
{
    fn new(
        board: [u16; BOARD_HEIGHT_EXTEND],
        score: u16,
        write: W,
        game_over: bool,
        curr_piece: Tetromino,
    ) -> Self {
        Self {
            board,
            score,
            write,
            game_over,
            curr_piece,
        }
    }
    fn clear_rows(&mut self) -> () {
        for row in self.board.iter_mut() {
            if row_full(*row) {
                *row = 0;
                self.score += 1;
            }
        }
    }
    pub(crate) fn make_game(write: W) -> Self {
        GameState::new(
            [0; BOARD_HEIGHT_EXTEND],
            0,
            write,
            false,
            Tetromino::new(TetVar::make_o(), 3, 0, 0),
        )
    }
    pub(crate) fn advance_timestep(&mut self) -> () {
        self.score += 1;
        self.clear_rows();
        self.move_down();
        // todo!()
    }
    pub(crate) fn get_board(&self) -> [u16; BOARD_HEIGHT] {
        self.board[0..BOARD_HEIGHT]
            .try_into()
            .expect("wack board height")
    }

    pub(crate) fn draw_board_and_piece(&self) -> [u16; BOARD_HEIGHT] {
        // let mut myBoard: [u16; BOARD_HEIGHT] = self.get_board().clone();
        let mut my_board = self.board;
        for i in 0..TETRO_BOX {
            my_board[*self.curr_piece.y() as usize + i] = my_board
                [*self.curr_piece.y() as usize + i]
                | self.curr_piece.shape().get_shape()[*self.curr_piece.idx() as usize][i];
        }
        my_board[0..BOARD_HEIGHT]
            .try_into()
            .expect("wack board height")
    }

    pub(crate) fn rotate_piece(&mut self) -> () {
        //
    }
    pub(crate) fn move_left(&mut self, left: bool) -> () {
        //
    }
    pub(crate) fn move_down(&mut self) -> () {
        let tmp = self.get_board_chunk();
        self.curr_piece.go_down(&self.get_board_chunk().clone()); // need board chunk as immutable
    }
    pub(crate) fn get_write(&mut self) -> &mut W {
        &mut self.write
    }
    pub(crate) fn get_win(&self) -> bool {
        self.game_over
    }
    pub(crate) fn get_score(&self) -> u16 {
        self.score
    }

    /// Get a reference to the game state's curr piece.
    pub(crate) fn curr_piece(&self) -> &Tetromino {
        &self.curr_piece
    }
    fn get_board_chunk(&self) -> crate::tetromino::BoardChunk {
        let tmp = *self.curr_piece.y() as usize;
        self.board[tmp..(tmp + 4)]
            .try_into()
            .expect("slice with wrong length")
    }
}

fn row_full(row: u16) -> bool {
    row > FULL_ROW
}
