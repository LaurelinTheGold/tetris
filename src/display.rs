use std::io::Write;

use crate::game::{self};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;

pub(crate) fn draw_game<W>(current_state: &game::GameState, write: &mut W) -> ()
where
    W: Write,
{
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for (idx, tet_row) in current_state.get_board().iter().enumerate() {
        if idx < 4 {
            continue;
        }
        for pos in (0..game::BOARD_WIDTH).rev() {
            queue!(write, MoveTo(x, y)).unwrap();
            if *tet_row & (1 << pos) != 0 {
                queue!(write, Print("x ")).unwrap();
            } else {
                queue!(write, Print(". ")).unwrap();
            }
            x += 2;
        }
        // println!("  line {}", 23 - idx);
        y += 1;
        x = 0;
    }
    write.flush().unwrap();
}
