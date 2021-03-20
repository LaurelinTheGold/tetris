use std::{borrow::BorrowMut, io::Write};

pub(crate) const BOARD_HEIGHT: usize = 24;
pub(crate) const BOARD_HEIGHT_SHOWN: usize = 20;
/// Number of Columns
pub(crate) const BOARD_WIDTH: usize = 10;
/// 2 ^ 10 - 1
const FULL_ROW: u16 = 1023;

enum Tetromino {
    TetI,
    TetT,
    TetO,
    TetZ,
    TetS,
    TetJ,
    TetL,
}
pub(crate) struct GameState<W>
where
    W: Write,
{
    board: [u16; BOARD_HEIGHT],
    score: u16,
    write: W,
    // curr_piece: Tetromino,
}

impl<W> GameState<W>
where
    W: Write,
{
    fn new(board: [u16; BOARD_HEIGHT], score: u16, write: W) -> Self {
        Self {
            board,
            score,
            write,
        }
    }
    fn clear_rows(&mut self) -> () {
        let mut count: u16 = 0;
        for row in self.board.iter_mut() {
            if row_full(*row) {
                *row = 0;
                count += 1;
            }
        }
        self.score += count;
    }
    pub(crate) fn make_game(write: W) -> Self {
        GameState::new([0; BOARD_HEIGHT], 0, write)
    }
    pub(crate) fn advance_timestep(&mut self) -> () {
        self.score += 1;
        self.clear_rows();
    }
    pub(crate) fn get_board(&self) -> [u16; BOARD_HEIGHT] {
        self.board
    }

    pub(crate) fn rotate_piece(&mut self) -> () {
        //
    }
    pub(crate) fn move_left(&mut self, left: bool) -> () {
        //
    }
    pub(crate) fn move_down(&mut self) -> () {
        //
    }
    pub(crate) fn get_write(&mut self) -> &mut W {
        &mut self.write
    }
}

fn row_full(row: u16) -> bool {
    row > FULL_ROW
}
