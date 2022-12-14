use std::fmt::Debug;
use rand::prelude::Rng;
use crate::mancala::*;
use cursive::{
    event::{Event, EventResult, Key, Callback},
    theme::ColorStyle,
    view::View,
    Printer, Vec2, XY,
};
use cursive::Cursive;

#[derive(Debug)]
pub enum PlayState {
    Playing,
    Finish,
}

#[derive(Debug, Clone, Copy)]
pub struct GameSettings{
     pub mode: GameMode,
     pub difficulty: Difficulty
}

pub struct MancalaBoard {
    game_state: Box<dyn GameState>,
    play_state: PlayState,
    name: String,
    win_callback: Callback
}

const BOARD_LEN: usize = 14;
const BOARD_POS_X: usize = 0;
const BOARD_POS_Y: usize = 0;

// hold the board info  
impl MancalaBoard {
    pub fn new<F>(settings: &GameSettings, call_back: F) -> Self
    where 
        F: 'static + Fn(&mut Cursive),
    {
        Self {
            game_state: match settings.mode {
                GameMode::Capture => {
                    Box::new(Capture{
                        game_board: MancalaBoard::generate_game_board(settings.difficulty),
                        selected_cell: Cell{side: Side::Bottom, pos: 0},
                    })
                }
                GameMode::Avalanche => {
                    Box::new(Avalanche{
                        game_board: MancalaBoard::generate_game_board(settings.difficulty),
                        selected_cell: Cell{side: Side::Bottom, pos: 0},
                    })
                }
            },
            play_state: PlayState::Playing,
            name: match settings.mode{
                GameMode::Capture => "Capture".to_string(),
                GameMode::Avalanche => "Avalanche".to_string(),
            },
            win_callback: Callback::from_fn(call_back)
        }
    }

    fn generate_game_board(difficulty: Difficulty) -> [usize; BOARD_LEN] where Self: Sized{
        let mut _game_board: [usize; BOARD_LEN] = [0; BOARD_LEN];
        let mut rng = rand::thread_rng();
        for p in 0..7{
            let bowl_value: usize;
            match difficulty{
                Difficulty::Normal => bowl_value = 4,
                Difficulty::Random => bowl_value = rng.gen_range(1..6),
            }
            _game_board[p] = bowl_value;
            _game_board[p+7] = bowl_value;
        }
        // set the cups to 0
        _game_board[6] = 0;
        _game_board[13] = 0;
        _game_board
    }

    fn draw_playing(&self, printer: &Printer) {
        let offset: Printer = printer.offset(XY{x:BOARD_POS_X, y:BOARD_POS_Y});
        self.draw_base_layer(&offset);
        self.draw_arrow(&offset);
        self.draw_selected_cell(&offset);
        self.draw_values(&offset);
    }

    
    fn draw_values(&self, printer: &Printer){
        for side in [Side::Top, Side::Bottom]{
            for pos in 0..=6{
                let cell = Cell{side: side, pos: pos};
                let (x, y) = MancalaBoard::cell_to_xy(cell);
                printer.print((x+2, y+1), &format!("{}", self.game_state.get_value(cell)));
            }
        }
    }
    
    fn draw_selected_cell(&self, printer: &Printer){
        let selected_cell: Cell = self.game_state.get_selected_cell();
        let (x, y) = MancalaBoard::cell_to_xy(selected_cell);
        printer.with_color(ColorStyle::title_primary(), |printer| {
            printer.print((x, y), "???????????????");
            printer.print((x, y+1), "???   ???");
            printer.print((x, y+2), "???????????????");
        });
    }
    
    fn draw_arrow(&self, printer: &Printer){
        let selected_cell: Cell = self.game_state.get_selected_cell();
        let (x, _) = MancalaBoard::cell_to_xy(selected_cell);
        match selected_cell {
            Cell{side: Side::Top, pos: _} => printer.with_color(ColorStyle::title_primary(), |printer| {
                printer.print((x + 2, 4), "???")
            }),
            Cell{side: Side::Bottom, pos: _} => printer.with_color(ColorStyle::title_primary(), |printer| {
                printer.print((x + 2, 4), "???")
            }),
        }
    }
    
    fn draw_base_layer(&self, printer: &Printer){
        let base_layer = String::from_utf8_lossy(include_bytes!("../assets/boardSchematic.txt"));
        for (position, line) in base_layer.lines().enumerate(){
            printer.print((0, position), &line);
        }
    }

    fn draw_win(&self, printer: &Printer) {
        let offset: Printer = printer.offset(XY{x:BOARD_POS_X, y:BOARD_POS_Y});
        self.draw_congrats(&offset);
        self.draw_who_won(&offset);
    }

    fn draw_congrats(&self, printer: &Printer){
        let congrats = String::from_utf8_lossy(include_bytes!("../assets/congrats.txt"));
        for (position, line) in congrats.lines().enumerate(){
            printer.print((0, position), &line);
        }
    }

    fn draw_who_won(&self, printer: &Printer){
        match self.game_state.winner(){
            Some(Side::Bottom) => printer.print((14, 6), "Bottom Player has won!"),
            Some(Side::Top) => printer.print((16, 6), "Top Player has won!"),
            None => printer.print((10, 6), "The match has ended in a draw!"),
        }
    }
    
    fn cell_to_xy(cell: Cell) -> (usize, usize){
        // bunch of magic to make the formating work with my data scheme
        match cell {
            Cell{side: Side::Top, pos: 6} => (2, 3),
            Cell{side: Side::Top, pos: _} => (9 + 5 * cell.pos, 1),
            Cell{side: Side::Bottom, pos: 6}  => (41, 3),
            Cell{side: Side::Bottom, pos: _} => (9 + 5 * cell.pos, 5),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

}

// holds how to draw the board on the screen
impl View for MancalaBoard {
    fn draw(&self, printer: &Printer) {
        match self.play_state {
            PlayState::Playing => self.draw_playing(printer),
            PlayState::Finish => self.draw_win(printer),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(48, 9)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.play_state {
            PlayState::Playing => {
                match event {
                    Event::Key(Key::Right) => self.game_state.move_right(),
                    Event::Key(Key::Left) => self.game_state.move_left(),
                    Event::Char(' ') | Event::Key(Key::Enter) => {
                        self.game_state.play();
                        if self.game_state.has_ended(){
                            self.play_state = PlayState::Finish;
                        }
                    },
                    Event::Key(Key::Esc) => return EventResult::Consumed(Some(self.win_callback.clone())),
                    _ => return EventResult::Ignored
                }
                EventResult::Consumed(None)
            }
            // no clue how the call back works it just works
            PlayState::Finish => {
                match event {
                    _ => return EventResult::Consumed(Some(self.win_callback.clone()))
                }
            }
        }
    }
}