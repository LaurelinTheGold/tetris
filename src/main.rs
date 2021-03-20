//importing in execute! macro
#[macro_use]
extern crate crossterm;

use core::time::Duration;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
        ScrollDown, ScrollUp, SetSize,
    },
};
use display::draw_game;
use game::{GameState, BOARD_HEIGHT_SHOWN, BOARD_WIDTH};
use std::{
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self},
    u16,
};

use std::io::stderr;
use std::thread::sleep;

mod display;
mod game;

const ONE_SEC: Duration = Duration::from_secs(1);
/// number of polls per sec
const FPS: f32 = 60.;
/// in seconds
const TIME_STEP: f32 = 1.;

fn main() -> () {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = setup();
    let mut tetris_game = GameState::make_game(stderr());
    draw_game(&mut tetris_game);
    loop {
        let thread_tx = tx.clone();
        // Thread that waits for a time then signals before ending
        let timer_thread = thread::spawn(move || {
            sleep(ONE_SEC.mul_f32(TIME_STEP)); // wait for time
            let _ = thread_tx.send(true).unwrap(); // use channel to signal about to done
        });
        while rx.try_recv().is_err() {
            if poll_for_keystroke(&mut tetris_game) {
                cleanup();
                return;
            }
            sleep(ONE_SEC.div_f32(FPS)); // wait
        }
        let _ = timer_thread.join().unwrap();
        GameState::advance_timestep(&mut tetris_game);
        draw_game(&mut tetris_game);
    }
}

/// test
fn poll_for_keystroke<W>(game: &mut GameState<W>) -> bool
where
    W: Write,
{
    if poll(Duration::from_secs(0)).unwrap() {
        // true means found
        let event: Event = read().unwrap();
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => {
                return true;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.rotate_piece();
                draw_game(game);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_left(true);
                draw_game(game)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_left(false);
                draw_game(game)
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_down();
                draw_game(game)
            }
            _ => {}
        };
    }
    return false;
}

fn setup() -> (Sender<bool>, Receiver<bool>) {
    println!("Press Enter to play tetris. Q quits. ");
    enable_raw_mode().unwrap();
    while read().unwrap() != Event::Key(KeyCode::Enter.into()) {}
    execute!(
        stderr(),
        EnterAlternateScreen,
        // SetSize(BOARD_WIDTH as u16 * 2, BOARD_HEIGHT_SHOWN as u16),
        // ScrollDown(BOARD_HEIGHT_SHOWN as u16 + 1) // TODO what is this???
    )
    .unwrap();
    let (t, r) = channel();
    return (t, r);
}

fn cleanup() -> () {
    execute!(
        stderr(),
        LeaveAlternateScreen,
        // SetSize(cols, rows),
    )
    .unwrap();
    disable_raw_mode().unwrap();
}
