use cursive::{views::{Dialog, LinearLayout}, Cursive, view::Nameable};
use crate::{mancala::{self, Mancala, GameSettings, GameMode, Difficulty}, game};
use std::io::Write;
use std::str::FromStr;
use std::error::Error;
use crate::board::MancalaBoard;

pub fn run(){
    let mut siv = cursive::default();
    
    let board: MancalaBoard = MancalaBoard::new();

    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(Dialog::around(board.with_name("board")));
    
    siv.run();

    // let settings: GameSettings = GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Random };
    // let mut game_state: Mancala = Mancala::new(settings);
    // //dbg!(game_state);
    // loop{
    //     println!("{}", game_state);
    //     println!("Enter a bowl number");
    //     print!("> ");
    //     std::io::stdout().flush()?;
    //     let mut pick_as_string: String = String::with_capacity(64);
    //     let _num_bytes_read = std::io::stdin().read_line(&mut pick_as_string)?;
    //     let pick: usize = usize::from_str(pick_as_string.trim())?;
    //     game_state.play(pick);
    //     if game_state.has_won(){
    //         break;
    //     }
    // }
}