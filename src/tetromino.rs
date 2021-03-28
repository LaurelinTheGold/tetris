use std::convert::TryInto;

use crate::game::{BOARD_HEIGHT, BOARD_WIDTH};

//different struct for each piece all impling a shgared trait OR
//You could also do it with a generic or an enum and have an impl per one, let see if someone else has an opinion :slight_smile:
//I'm not sure where you'd be using a union of pointers here (or what you really mean by that in this case). But if you need a function that takes in a tetromino struct, then you can use a trait bound on it
/*enum MyStructs {
  Cat(CatStruct),
  Dog(DogStruct)
} */
/*match animal {
  Cat(cat) => cat.purr(),
  Dog(dog) => dog.woaf()
} */
// You can think about it this way, an enum is saying "I can be any of these different things", and a trait is saying "I have property X" in your case you want it to say "I act as a tetromino, and therefore have rotate and move functions"
// So when you write your function signature, instead of saying "accept any of this list of things" you say "accept anything that acts as a tetromino"
// thanks alfie and &mut Hazen on the rust discord for help

pub(crate) const NUM_ROT: usize = 4;
pub(crate) const TETRO_BOX: usize = 4;

type TetShape = [[u16; TETRO_BOX]; NUM_ROT];
pub(crate) type BoardChunk = [u16; TETRO_BOX];

const TET_O: TetShape = [[0b1100, 0b1100, 0, 0]; TETRO_BOX];

pub(crate) enum TetVar {
    O(TetShape),
    I(TetShape),
    T(TetShape),
    Z(TetShape),
    S(TetShape),
    J(TetShape),
    L(TetShape),
}

impl From<TetShape> for TetVar {
    fn from(v: TetShape) -> Self {
        Self::O(v)
    }
}

impl TetVar {
    pub(crate) fn get_shape(&self) -> &TetShape {
        match self {
            TetVar::O(a)
            | TetVar::I(a)
            | TetVar::T(a)
            | TetVar::Z(a)
            | TetVar::S(a)
            | TetVar::J(a)
            | TetVar::L(a) => a,
        }
    }
    pub(crate) fn make_o() -> Self {
        TetVar::from(TET_O)
    }
}

pub(crate) trait TetrominoOps {
    fn rotate(&mut self, chunk: &BoardChunk) -> bool;
    fn go_left(&mut self, left: bool, chunk: &BoardChunk) -> bool;
    fn go_down(&mut self, chunk: &BoardChunk) -> bool;
}

pub(crate) struct Tetromino {
    shape: TetVar,
    /// 9 to 0 as doing bitwise ops, Top left of box.
    x: i16,
    /// 0 at top, top 4 hidden, 3 to 23 visible
    y: i16,
    idx: u16,
}

impl Tetromino {
    pub(crate) fn new(shape: TetVar, x: i16, y: i16, idx: u16) -> Self {
        Self { shape, x, y, idx }
    }

    fn do_rotate(&mut self, not_reverse: bool) -> &mut Self {
        self.idx = if not_reverse {
            (self.idx + 1) % NUM_ROT as u16
        } else {
            (self.idx - 1) % NUM_ROT as u16
        };
        self
    }
    fn to_the_left(&mut self, left: bool) -> &mut Self {
        self.x = if left { self.x + 1 } else { self.x - 1 };
        self
    }
    fn to_the_down(&mut self, down: bool) -> &mut Self {
        self.y = if down { self.y + 1 } else { self.x - 1 };
        self
    }
    // pub(crate) fn shifted_piece(&self) -> Self {
    //     let tmp = &mut self.clone();
    //     for row in tmp.into_iter() {
    //         *row = *row >> (TETRO_BOX - 1);
    //     }
    // }

    /// Get a reference to the tetromino's y.
    pub(crate) fn y(&self) -> &i16 {
        &self.y
    }

    /// Get a reference to the tetromino's shape.
    pub(crate) fn shape(&self) -> &TetVar {
        &self.shape
    }

    /// Get a reference to the tetromino's idx.
    pub(crate) fn idx(&self) -> &u16 {
        &self.idx
    }
}

impl TetrominoOps for Tetromino {
    fn rotate(&mut self, chunk: &BoardChunk) -> bool {
        if is_legal_move(self.do_rotate(true), chunk) {
            true
        } else {
            self.do_rotate(false);
            false
        }
    }

    fn go_left(&mut self, left: bool, chunk: &BoardChunk) -> bool {
        if is_legal_move(self.to_the_left(left), chunk) {
            true
        } else {
            self.to_the_left(!left);
            false
        }
    }

    /// if this returns false, fix the tetromino to the board
    fn go_down(&mut self, chunk: &BoardChunk) -> bool {
        if is_legal_move(self.to_the_down(true), chunk) {
            true
        } else {
            self.to_the_down(false);
            false
        }
    }
}

fn no_collision(piece: &BoardChunk, chunk: &BoardChunk) -> bool {
    let tmp = &mut piece.clone();
    for row in tmp.into_iter() {
        *row = *row >> (TETRO_BOX - 1);
    }
    let mut x: [bool; TETRO_BOX] = [false; TETRO_BOX];
    for i in 0..TETRO_BOX {
        x[i] = tmp[i] & chunk[i] == 0
    }
    x.iter().rfold(false, |acc, x| acc || *x)
}

/// Check that the x and y bounds are good and no overlap
fn is_legal_move(shape: &Tetromino, chunk: &BoardChunk) -> bool {
    let tmp = &mut shape.shape.get_shape()[shape.idx as usize].clone();
    for num in tmp.iter_mut() {
        *num = *num << shape.x as u16;
    }
    shape.x < BOARD_WIDTH as i16
        && shape.x >= 0
        && shape.y < BOARD_HEIGHT as i16
        && shape.y >= 0
        && no_collision(tmp, chunk)
}
