use rand::prelude::*;
use crate::mancala::{GameMode, Difficulty, Side, GameState, Capture, Avalanche, Cell};
use cursive::{
    event::{Event, EventResult, Key, MouseEvent},
    theme::ColorStyle,
    view::View,
    Printer, Vec2, XY,
};

use crate::lib::read_lines;

#[derive(Debug)]
pub enum PlayState {
    Config,
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
    game_settings: GameSettings,
}

const BOARD_LEN: usize = 14;
const BOARD_POS_X: usize = 0;
const BOARD_POS_Y: usize = 0;
// hold the board info  
impl MancalaBoard {
    pub fn new(settings: GameSettings) -> Self{
        Self {
            game_state: 
                match settings.mode {
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
            play_state: PlayState::Config,
            game_settings: settings
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

    fn draw_config(&self, printer: &Printer) {
        printer.print((0, 0), "Press <Enter> to Start!");
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
        printer.print((x, y), "╭╌╌╌╮");
        printer.print((x, y+1), "╎   ╎");
        printer.print((x, y+2), "╰╌╌╌╯");
    }

    fn draw_arrow(&self, printer: &Printer){
        let selected_cell: Cell = self.game_state.get_selected_cell();
        let (x, _) = MancalaBoard::cell_to_xy(selected_cell);
        match selected_cell {
            Cell{side: Side::Top, pos: _} => printer.print((x + 2, 4), "↑"),
            Cell{side: Side::Bottom, pos: _} => printer.print((x + 2, 4), "↓"),
        }
    }

    fn draw_base_layer(&self, printer: &Printer){
        if let Ok(lines) = read_lines("assets/boardSchematic.txt"){
            for (position, line) in lines.enumerate(){
                if let Ok(ip) = line{
                    printer.print((0, position), &ip);
                }
            }
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

}

// holds how to draw the board on the screen
impl View for MancalaBoard {
    fn draw(&self, printer: &Printer) {
        match self.play_state {
            PlayState::Config => self.draw_config(printer),
            PlayState::Playing => self.draw_playing(printer),
            PlayState::Finish => (),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //  Vec2::new(19, 19)
        Vec2::new(48, 9)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.play_state {
            PlayState::Config => {
                match event {
                    Event::Key(Key::Enter) => {
                        self.play_state = PlayState::Playing;
                    }
                    _ => return EventResult::Ignored,
                }
                EventResult::Consumed(None)
            }
            
            PlayState::Playing => {
                match event {
                    Event::Key(Key::Right) => self.game_state.move_right(),
                    Event::Key(Key::Left) => self.game_state.move_left(),
                    Event::Char(' ') | Event::Key(Key::Enter) => self.game_state.play(),
                    _ => return EventResult::Ignored
                }
                EventResult::Consumed(None)
            }
            
            PlayState::Finish => EventResult::Ignored,
        }
    }

}