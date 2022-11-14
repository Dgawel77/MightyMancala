use cursive::{Cursive, CursiveExt};
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, SliderView, TextView, BoxedView, TextArea, TextContent};
use cursive::align::{Align, HAlign};
use cursive::traits::*;
use cursive::{view::Nameable};
use crate::{mancala::{GameMode, Difficulty}, game};
use std::io::Write;
use std::str::FromStr;
use std::error::Error;
use cursive_aligned_view::AlignedView;
use crate::board::{MancalaBoard, GameSettings};
use crate::lib::read_string;

pub fn run(){
    let mut siv = Cursive::default();
    let settings: GameSettings = GameSettings{mode: GameMode::Capture, difficulty: Difficulty::Normal };
    siv.set_user_data(settings);
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(get_home_page());
    
    //let board: MancalaBoard = MancalaBoard::new(GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Normal });
    //siv.add_layer(Dialog::around(board.with_name("board")));
    siv.run();
}

fn get_home_page() -> Dialog{
// =================================================
    // Make the Mode selection
    // =================================================
    let mode_closure = |s: &mut Cursive, mode: &GameMode|{
        s.with_user_data(|settings: &mut GameSettings| settings.mode = *mode);
    };
    let select_mode = SelectView::<GameMode>::new()
    .item("Capture", GameMode::Capture)
    .item("Avalanche", GameMode::Avalanche)
    .align(Align::top_center())
    .on_submit(mode_closure)
    .on_select(mode_closure);

    // =================================================
    // Make the Difficulty selection
    // =================================================
    let dificulty_closure = |s: &mut Cursive, difficulty: &Difficulty|{
        s.with_user_data(|settings: &mut GameSettings| settings.difficulty = *difficulty);
    };
    let select_difficulty = SelectView::<Difficulty>::new()
    .item("Normal", Difficulty::Normal)
    .item("Random", Difficulty::Random)
    .align(Align::top_center())
    .on_submit(dificulty_closure)
    .on_select(dificulty_closure);

    // =================================================
    // Make the Buttons
    // =================================================
    let buttons = LinearLayout::vertical()
        .child(Button::new("Play", play_game))
        .child(Button::new("About", about_window));

    // =================================================
    // Make the Logo
    // =================================================
    let logo = TextView::new(read_string("assets/logo.txt"));
    
    // =================================================
    // Make the Homepage
    // =================================================
    Dialog::around(
        LinearLayout::vertical()
        .child(logo)
        .child(DummyView)
        .child(
        AlignedView::with_center(
            LinearLayout::horizontal()
            .child(Dialog::around(select_mode)
                .title("Mode")
                .fixed_width(17))
            .child(DummyView)
            .child(Dialog::around(select_difficulty)
                .title("Difficulty")
                .fixed_width(17))
            .child(buttons)))
    )
}

fn play_game(s: &mut Cursive) {
    let settings: GameSettings = s.take_user_data().unwrap();
    let board: MancalaBoard = MancalaBoard::new(settings);
    s.pop_layer();
    s.add_layer(
        Dialog::around(board.with_name("board"))
        //.button("Quit", |s: &mut Cursive|{
        //    s.pop_layer();
        //    s.add_layer(get_home_page());
        //})
        .fixed_height(20).fixed_width(70)
    );
}

fn about_window(s: &mut Cursive) -> (){
    s.pop_layer();
    s.add_layer(
        Dialog::around(TextView::new(read_string("assets/help_page.txt")))
        .button("Quit", |s: &mut Cursive|{
            s.pop_layer();
            s.add_layer(get_home_page());
        })
    );
}