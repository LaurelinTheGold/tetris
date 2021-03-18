pub(crate) const BOARD_HEIGHT: usize = 24;
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
    pub(crate) fn make_test(board: [u16; BOARD_HEIGHT]) -> Self {
        Self { board, score: 0 }
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
    pub(crate) fn do_thing(&mut self) -> () {
        self.score += 1;
        self.clear_rows();
        // println!("Tetris! {}", self.score);
    }
    pub(crate) fn get_board(&self) -> [u16; BOARD_HEIGHT] {
        self.board
    }
}

fn row_full(row: u16) -> bool {
    row > FULL_ROW
}

// struct Example {
//     number: i32,
// }
// impl Example {
//     fn boo() {
//         println!("boo! Example::boo() was called!");
//     }
//     fn answer(&mut self) {
//         self.number += 42;
//     }
//     fn get_number(&self) -> i32 {
//         self.number
//     }
// }
// trait Thingy {
//     fn do_thingy(&self);
// }
// impl Thingy for Example {
//     fn do_thingy(&self) {
//         println!("doing a thing! also, number is {}!", self.number);
//     }
// }
