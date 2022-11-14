use cursive::{Cursive, CursiveExt};
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, SliderView, TextView};
use cursive::align::{Align, HAlign};
use cursive::traits::*;
use cursive::{view::Nameable};
use crate::{mancala::{GameMode, Difficulty}, game};
use std::io::Write;
use std::str::FromStr;
use std::error::Error;
use crate::board::{MancalaBoard, GameSettings};

pub fn run(){
    // gonna have to redo a lot of the ui
    // let mut select_mode = SelectView::<GameMode>::new().h_align(HAlign::Left);
    // select_mode.add_item("Capture", GameMode::Capture);
    // select_mode.add_item("Avalanche", GameMode::Avalanche);

    // select_mode.set_on_submit(|s, time| {
    //     // s.pop_layer();
    //     // let text = format!("You will wait for {} minutes...", time);
    //     // s.add_layer(
    //     //     Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
    //     // );
    // });

    // let mut siv = Cursive::default();
    // siv.add_layer(Dialog::around(
    //     LinearLayout::horizontal()
    //     .child(select_mode)
    // ).title("Mighty Mancala"));
    // siv.run(); 
    // set settings and other stuff
    let mut siv = Cursive::default();
    let mut settings: GameSettings = GameSettings{mode: GameMode::Capture, difficulty: Difficulty::Normal };
    siv.add_global_callback('q', Cursive::quit);
    
    // make the selection windows
    let mut select_mode = SelectView::<GameMode>::new()
    .item("Capture", GameMode::Capture)
    .item("Avalanche", GameMode::Avalanche)
    .align(Align::top_center());

    let mut select_difficulty = SelectView::<Difficulty>::new()
    .item("Normal", Difficulty::Normal)
    .item("Random", Difficulty::Random)
    .align(Align::top_center());

    
    select_mode.set_on_submit(|s, mode|{
        
    });

    select_difficulty.set_on_select(|s, difficulty|{
        dbg!(difficulty);
    });
    select_difficulty.set_on_submit(|s, difficulty|{
        dbg!(difficulty);
    });


    let slider_view = SliderView::horizontal(10)
    .on_enter(|s, n| match n {
        5 => s.add_layer(Dialog::info("You did it!")),
        n => s.add_layer(Dialog::info(format!("Why {}? Why not 5?", n))),
    });

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(Dialog::around(select_mode).title("Mode").fixed_width(17))
        .child(DummyView)
        .child(Dialog::around(select_difficulty).title("Difficulty").fixed_width(17))
        .child(slider_view))
    );
    
    //let board: MancalaBoard = MancalaBoard::new(GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Normal });
    //siv.add_layer(Dialog::around(board.with_name("board")));
    
    siv.run();
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}