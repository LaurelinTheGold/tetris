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
enum TetrominoShape {
    O(TetO),
    // I(TetI),
    // T(TetT),
    // Z(TetZ),
    // S(TetS),
    // J(TetJ),
    // L(TetL),
}
trait TetrominoOps {
    fn rotate(&mut self) -> bool;
    fn go_left(&mut self, left: bool) -> bool;
    fn go_down(&mut self) -> bool;
}

struct Tetromino {
    shape: TetrominoShape,
    /// 9 to 0 as doing bitwise ops, Top left of box.
    x: u16,
    y: u16,
}

impl Tetromino {}

impl TetrominoOps for Tetromino {
    fn go_left(&mut self, left: bool) -> bool {
        if left {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }

    fn go_down(&mut self) -> bool {
        todo!()
    }
    fn rotate(&mut self) -> bool {
        todo!()
    }
}

fn check_collision(piece: Tetromino, board: [u16; crate::game::BOARD_HEIGHT]) -> bool {
    unimplemented!()
}
