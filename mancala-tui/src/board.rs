use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::prelude::*;
use crate::mancala::{GameMode, Difficulty, Side, GameState, Capture, Avalanche};
use cursive::{
    event::{Event, EventResult, Key, MouseEvent},
    theme::ColorStyle,
    view::View,
    Printer, Vec2, XY,
};

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
                            in_play: Side::Bottom,
                            selected_cell: 0,
                        })
                    }
                    GameMode::Avalanche => {
                        Box::new(Avalanche{
                            game_board: MancalaBoard::generate_game_board(settings.difficulty),
                            in_play: Side::Bottom,
                            selected_cell: 0,
                        })
                    }
                },
            play_state: PlayState::Config,
            game_settings: settings
        }
    }

    fn generate_game_board(difficulty: Difficulty) -> [u8; BOARD_LEN] where Self: Sized{
        let mut _game_board: [u8; BOARD_LEN] = [0; BOARD_LEN];
        let mut rng = rand::thread_rng();
        for p in 0..7{
            let bowl_value: u8;
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
        // will probably refactor how printing is done
        // currently based off array index but game_state is based off who is playing
        self.draw_base_layer(&offset);
        self.draw_arrow(&offset);
        self.draw_selected_cell(&offset);
        self.draw_values(&offset);
    }

    fn draw_values(&self, printer: &Printer){
        for cell in 0..=13{
            let (x, y) = MancalaBoard::cell_to_xy(cell);
            printer.print((x+2, y+1), &format!("{}", self.game_state.get_value(cell as usize)));
        }
    }

    fn draw_selected_cell(&self, printer: &Printer){
        let selected_cell: u8 = self.game_state.get_selected_index();
        let (x, y) = MancalaBoard::cell_to_xy(selected_cell);
        printer.print((x, y), "╭╌╌╌╮");
        printer.print((x, y+1), "╎   ╎");
        printer.print((x, y+2), "╰╌╌╌╯");
    }

    fn draw_arrow(&self, printer: &Printer){
        let (x, _) = MancalaBoard::cell_to_xy(self.game_state.get_selected_index());
        match self.game_state.get_selected_index() {
            7..=12 => printer.print((x+2,4), "↑"),
            0..=5 => printer.print((x+2,4), "↓"),
            _ => unreachable!(),
        }
    }

    fn draw_base_layer(&self, printer: &Printer){
        if let Ok(lines) = MancalaBoard::read_lines("assets/boardSchematic.txt"){
            for (position, line) in lines.enumerate(){
                if let Ok(ip) = line{
                    printer.print((0, position), &ip);
                }
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn cell_to_xy(cell: u8) -> (u8, u8){
        // bunch of magic to make the formating work with my data scheme
        match cell {
            6 => (41, 3),
            13 => (2, 3),
            0..=5 => (9+cell*5, 5),
            7..=12 => (69-cell*5, 1),
            _ => unreachable!()
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
                    Event::Char(' ') => self.game_state.play(),
                    Event::Key(Key::Enter) => self.game_state.play(),
                    _ => return EventResult::Ignored
                }
                EventResult::Consumed(None)
            }
            
            PlayState::Finish => EventResult::Ignored,
        }
    }

}