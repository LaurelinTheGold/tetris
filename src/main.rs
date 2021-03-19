//importing in execute! macro
#[macro_use]
extern crate crossterm;

use core::time::Duration;
use crossterm::event::poll;
use crossterm::terminal::size;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ScrollUp};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::SetSize,
};
use display::draw_game;
use game::GameState;
use std::{
    io::stdout,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self},
    u16,
};

use std::io::stderr;
use std::thread::sleep;

mod display;
mod game;

const ONE_SEC: Duration = Duration::from_secs(1);
const FPS: f32 = 60.; // number of polls per sec
const TIME_STEP: f32 = 1.; // in seconds

fn main() -> () {
    let (cols, rows, tx, rx): (u16, u16, Sender<bool>, Receiver<bool>) = setup();
    let mut tetris_game = game::GameState::make_game();
    loop {
        let thread_tx = tx.clone();
        // Thread that waits for a time then signals before ending
        let timer_thread = thread::spawn(move || {
            sleep(ONE_SEC.mul_f32(TIME_STEP)); // wait for time
            let _ = thread_tx.send(true); // use channel to signal about to done
        });
        while rx.try_recv().is_err() {
            if poll_for_keystroke(&mut tetris_game) {
                cleanup(cols, rows);
                return;
            }
            sleep(ONE_SEC.div_f32(FPS)); // wait
        }
        let _ = timer_thread.join().unwrap();
        GameState::advance_timestep(&mut tetris_game);
        draw_game(&tetris_game, &mut stdout());
    }
}

fn poll_for_keystroke(game: &mut GameState) -> bool {
    if poll(Duration::from_secs(0)).unwrap() {
        // true means found
        let event: crossterm::event::Event = read().unwrap();
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
                draw_game(game, &mut stderr());
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_left(true);
                draw_game(game, &mut stderr())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_left(false);
                draw_game(game, &mut stderr())
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
            }) => {
                game.move_down();
                draw_game(game, &mut stderr())
            }
            _ => {}
        };
    }
    return false;
}

fn setup() -> (u16, u16, Sender<bool>, Receiver<bool>) {
    let (cols, rows) = size().unwrap();
    println!("Press Enter to play tetris. Q quits. ");
    enable_raw_mode().unwrap();
    while read().unwrap() != Event::Key(KeyCode::Enter.into()) {}
    execute!(
        stdout(),
        SetSize(
            game::BOARD_WIDTH as u16 * 2,
            game::BOARD_HEIGHT_SHOWN as u16
        ),
        ScrollUp(5) // TODO what is this???
    )
    .unwrap();
    let (t, r) = channel();
    return (cols, rows, t, r);
}

fn cleanup(cols: u16, rows: u16) -> () {
    execute!(stdout(), SetSize(cols, rows)).unwrap();
    disable_raw_mode().unwrap();
}
