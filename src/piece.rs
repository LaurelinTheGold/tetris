use crate::board::BoardRow;

const UNIT_SIZE: usize = 16;
const NUM_ROTS: usize = 4;
const TET_BOX_SIZE: usize = 4;
/// contains 4 TetBoxes 1 for each orientation, 0 is the Most Sig 16 bits, etc
type TetRotFull = u64;
/// 4x4 bit box, MSB top left, bit12 top right, bit3 bottom left, LSB bottom right
type TetBox = u16;
/// MSB and 2nd most are rot info, next 7 bits are unisgned x, next 7 are unsigned y
type TetOther = u16;
type BoardBoxRows = [BoardRow; TET_BOX_SIZE];

/*
Comparison traits: Eq, PartialEq, Ord, PartialOrd.
Clone, to create T from &T via a copy.
Copy, to give a type 'copy semantics' instead of 'move semantics'.
Hash, to compute a hash from &T.
Default, to create an empty instance of a data type.
Debug, to format a value using the {:?} formatter.
*/

const TET_O: TetRotFull =
    0b1100_1100_0000_0000_1100_1100_0000_0000_1100_1100_0000_0000_1100_1100_0000_0000u64;
const TET_I: TetRotFull =
    0b1111_0000_0000_0000_1000_1000_1000_1000_1111_0000_0000_0000_1000_1000_1000_1000u64;
const TET_L: TetRotFull =
    0b1000_1000_1100_0000_1110_1000_0000_0000_1100_0100_0100_0000_0010_1110_0000_0000u64;
const TET_J: TetRotFull =
    0b0100_0100_1100_0000_1000_1110_0000_0000_1100_1000_1000_0000_1110_0010_0000_0000u64;
const TET_S: TetRotFull =
    0b0110_1100_0000_0000_1000_1100_0100_0000_0110_1100_0000_0000_1000_1100_0100_0000u64;
const TET_Z: TetRotFull =
    0b1100_0110_0000_0000_0100_1100_1000_0000_1100_0110_0000_0000_0100_1100_1000_0000u64;
const TET_T: TetRotFull =
    0b1110_0100_0000_0000_0100_1100_0100_0000_0100_1110_0000_0000_1000_1100_1000_0000u64;

#[derive(Debug)]
struct InvalidIndexErr;
trait TetOtherTraits {
    // cannot error
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
    fn get_r(&self) -> usize;
    // can error
    fn set_x(&mut self, new_x: usize) -> ();
    fn set_y(&mut self, new_y: usize) -> ();
    fn set_r(&mut self, new_r: usize) -> ();
    // can error at edge for x y
    fn inc_x(&mut self, move_left: bool) -> ();
    fn inc_y(&mut self, move_down: bool) -> ();
    fn inc_r(&mut self, clockwise: bool) -> ();
}

trait TetRotTraits {
    fn get_tet_box(&self, idx: usize) -> Result<TetBox, InvalidIndexErr>;
    fn make_from_idx(num: usize) -> Result<TetRotFull, InvalidIndexErr>;
}
trait TetOps {
    fn move_x(&mut self, move_left: bool) -> ();
    fn move_y(&mut self, move_down: bool) -> ();
    fn rotate(&mut self, clockwise: bool) -> ();
    fn get_curr_piece(&self) -> BoardBoxRows;
}

trait TetBoxTraits {
    fn reshape_to_array(&self) -> [BoardRow; TET_BOX_SIZE];
}

struct Tetromino {
    piece_bits: TetRotFull,
    piece_meta: TetOther,
}

impl TetRotTraits for TetRotFull {
    fn get_tet_box(&self, idx: usize) -> Result<TetBox, InvalidIndexErr> {
        match idx {
            0 => Ok((self >> (3 * UNIT_SIZE) & (1 << UNIT_SIZE - 1)) as u16),
            1 => Ok((self >> (2 * UNIT_SIZE) & (1 << UNIT_SIZE - 1)) as u16),
            2 => Ok((self >> (1 * UNIT_SIZE) & (1 << UNIT_SIZE - 1)) as u16),
            3 => Ok((self >> (0 * UNIT_SIZE) & (1 << UNIT_SIZE - 1)) as u16),
            _ => Err(InvalidIndexErr),
        }
    }

    fn make_from_idx(num: usize) -> Result<TetRotFull, InvalidIndexErr> {
        match num {
            0 => Ok(TET_O),
            1 => Ok(TET_I),
            2 => Ok(TET_L),
            3 => Ok(TET_J),
            4 => Ok(TET_S),
            5 => Ok(TET_Z),
            6 => Ok(TET_T),
            _ => Err(InvalidIndexErr),
        }
    }
}

impl TetOtherTraits for TetOther {
    fn get_x(&self) -> usize {
        todo!()
    }

    fn get_y(&self) -> usize {
        todo!()
    }

    fn get_r(&self) -> usize {
        todo!()
    }

    fn set_x(&mut self, new_x: usize) -> () {
        todo!()
    }

    fn set_y(&mut self, new_y: usize) -> () {
        todo!()
    }

    fn set_r(&mut self, new_r: usize) -> () {
        todo!()
    }

    fn inc_x(&mut self, move_left: bool) -> () {
        todo!()
    }

    fn inc_y(&mut self, move_down: bool) -> () {
        todo!()
    }

    fn inc_r(&mut self, clockwise: bool) -> () {
        todo!()
    }
}

impl TetBoxTraits for TetBox {
    fn reshape_to_array(&self) -> [BoardRow; TET_BOX_SIZE] {
        todo!()
    }
}

impl TetOps for Tetromino {
    fn move_x(&mut self, move_left: bool) -> () {
        self.piece_meta.inc_x(move_left);
    }

    fn move_y(&mut self, move_down: bool) -> () {
        self.piece_meta.inc_y(move_down);
    }

    fn rotate(&mut self, clockwise: bool) -> () {
        self.piece_meta.inc_r(clockwise);
    }

    fn get_curr_piece(&self) -> BoardBoxRows {
        self.piece_bits
            .get_tet_box(self.piece_meta.get_r())
            .unwrap()
            .reshape_to_array()
    }
}
