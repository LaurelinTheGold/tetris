//importing in execute! macro
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

use std::thread::sleep;

mod display;
mod game;

fn main() {
    println!("{}", display::TWENTY_NEWLINES);

    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, 
        Clear(ClearType::All), 
        cursor::MoveTo(0, 0), 
        Print(r#"ctrl + q exit, ctrl + h "Hello world", alt + t "crossterm is cool""#))
            .unwrap();


    let mut game_tick: usize = 0;
    let mut tetris_game = game::GameState::make_game();
    loop {
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        //matching the key
        match read().unwrap() {
            //i think this speaks for itself
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => execute!(stdout, Clear(ClearType::All), Print("Hello world!")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::ALT,
            }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            _ => (),
        }

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: no_modifiers,
            }) => //--code--
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }

        // display::draw_game(&tetris_game);
        display::draw_test(game_tick);
        tetris_game.do_thing();
        game_tick += 1;
        // println!("Tick {}", game_tick);
        sleep(core::time::Duration::from_secs(1));
        display::clear_screen();
    }
}

/*
board width 10
board height 20 on screen 24 total
tetrominos I T Z S J L O rotated

*/