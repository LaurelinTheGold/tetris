use std::io::Write;

use crate::game::{self};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;

pub(crate) fn draw_game<W>(current_state: &mut game::GameState<W>) -> ()
where
    W: Write,
{
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for (idx, tet_row) in current_state.draw_board_and_piece().iter().enumerate() {
        if idx < 4 {
            continue;
        }
        for pos in (0..game::BOARD_WIDTH).rev() {
            queue!(current_state.get_write(), MoveTo(x, y)).unwrap();
            if *tet_row & (1 << pos) != 0 {
                queue!(current_state.get_write(), Print("x ")).unwrap();
            } else {
                queue!(current_state.get_write(), Print(". ")).unwrap();
            }
            x += 2;
        }
        y += 1;
        x = 0;
    }
    current_state.get_write().flush().unwrap();
}
