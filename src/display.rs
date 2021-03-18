use std::io::{self, Write};

use crate::game::{self};

pub(crate) const TWENTY_NEWLINES: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

pub(crate) fn clear_screen() -> () {
    print!("\x1B[2J\x1B[1;1H"); // Thanks Muhammad Tariq Baig from stackoverlfwo
}

pub(crate) fn draw_game(current_state: &game::GameState) -> () {
    for (idx, tet_row) in current_state.get_board().iter().enumerate() {
        print!("      "); // more centeredish
        if idx < 4 {
            println!();
            continue;
        }
        for pos in (0..10).rev() {
            if *tet_row & (1 << pos) != 0 {
                print!("x ");
            } else {
                print!(". ")
            }
        }
        println!("  line {}", 23 - idx);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::GameState;

    const TEST_CENTER_SQUARE: [u16; crate::game::BOARD_HEIGHT] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48,
    ];

    fn test_setup() -> Vec<GameState> {
        let mut test_game_states: Vec<GameState> = Vec::new();
        test_game_states.push(GameState::make_test(TEST_CENTER_SQUARE));
        return test_game_states;
    }
    #[test]
    fn draw_test() {
        let test_game_states_vec: Vec<GameState> = test_setup();
        crate::display::draw_game(&test_game_states_vec[0]);
        // assert_eq!(2, 2);
    }
}

use crate::game::GameState;

const TEST_CENTER_SQUARE: [u16; crate::game::BOARD_HEIGHT] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48,
];

fn test_setup() -> Vec<GameState> {
    let mut test_game_states: Vec<GameState> = Vec::new();
    test_game_states.push(GameState::make_test([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 0, 0,
    ]));
    test_game_states.push(GameState::make_test([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 0,
    ]));
    test_game_states.push(GameState::make_test(TEST_CENTER_SQUARE));
    return test_game_states;
}
// #[test]
pub(crate) fn draw_test(idx: usize) {
    let test_game_states_vec: Vec<GameState> = test_setup();
    crate::display::draw_game(&test_game_states_vec[idx % test_game_states_vec.len()]);
    // println!("{:?}", test_game_states_vec[0].get_board());
}
