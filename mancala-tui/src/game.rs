use cursive::{views::Dialog, Cursive};

use crate::{mancala::{self, Mancala, GameSettings, GameMode, Difficulty}, game};

pub fn run(){
    // let mut siv = cursive::default();
    
    // siv.add_global_callback('q', Cursive::quit);

    // siv.add_layer(Dialog::text("Hey!"));
    
    // siv.run();
    let settings: GameSettings = GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Random };
    let mut game_state: Mancala = Mancala::new(settings);
    //dbg!(game_state);
    println!("{}", game_state);
    game_state.play(1);
    println!("{}", game_state);
    game_state.play(4);
    println!("{}", game_state);
    game_state.play(8);
    println!("{}", game_state);
    game_state.play(14);
    println!("{}", game_state);
}