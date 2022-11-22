use cursive::{
    Cursive,
    CursiveExt,
    views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView, ScrollView},
    view::Nameable,
    align::Align,
    traits::Resizable
};
use cursive_aligned_view::AlignedView;
use crate::mancala::{GameMode, Difficulty};
use crate::board::{MancalaBoard, GameSettings};
use crate::lib::read_string;

pub fn run(){
    // backend that does not flicker
    // let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
    //     let backend = cursive::backends::crossterm::Backend::init()?;
    //     let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
    //     Ok(Box::new(buffered_backend))
    // };
    let mut siv = Cursive::new();
    let settings: GameSettings = GameSettings{mode: GameMode::Capture, difficulty: Difficulty::Normal };
    
    siv.set_user_data(settings);
    siv.add_global_callback('q', Cursive::quit);

    siv.add_layer(get_home_page());
    
    //let board: MancalaBoard = MancalaBoard::new(GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Normal });
    //siv.add_layer(Dialog::around(board.with_name("board")));
    //siv.try_run_with(backend_init).ok();
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
        .child(DummyView)
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
    let settings: &mut GameSettings = s.user_data().unwrap();
    let board: MancalaBoard = MancalaBoard::new(settings, |s: &mut Cursive|{
        s.pop_layer();
        s.add_layer(get_home_page());
    });
    let name = board.get_name();
    s.pop_layer();
    s.add_layer(Dialog::around(board.with_name("board")).title(name));
}

fn about_window(s: &mut Cursive) -> (){
    s.pop_layer();
    s.add_layer(
    Dialog::around(
        ScrollView::new(
            TextView::new(read_string("assets/help_page.txt"))
        )
    )
    .button("Quit", |s: &mut Cursive|{
        s.pop_layer();
        s.add_layer(get_home_page());
    }));
}