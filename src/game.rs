pub(crate) const BOARD_HEIGHT: usize = 24;
pub(crate) const BOARD_HEIGHT_SHOWN: usize = 20;
pub(crate) const BOARD_WIDTH: usize = 10;
const FULL_ROW: u16 = 1023; //2 ^ 10 - 1

enum Tetromino {
    TetI,
    TetT,
    TetO,
    TetZ,
    TetS,
    TetJ,
    TetL,
}
pub(crate) struct GameState {
    board: [u16; BOARD_HEIGHT],
    score: u16,
    // curr_piece: Tetromino,
}

impl GameState {
    fn new(board: [u16; BOARD_HEIGHT], score: u16) -> Self {
        Self { board, score }
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
    pub(crate) fn make_game() -> Self {
        GameState::new([0; BOARD_HEIGHT], 0)
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
}

fn row_full(row: u16) -> bool {
    row > FULL_ROW
}
