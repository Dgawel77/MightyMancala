use cursive::{views::{Dialog, LinearLayout}, Cursive, view::Nameable};
use crate::{mancala::{GameMode, Difficulty}, game};
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
}