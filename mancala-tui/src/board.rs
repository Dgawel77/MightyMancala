use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

pub struct GameSettings{
     pub mode: GameMode,
     pub difficulty: Difficulty
}

pub struct MancalaBoard {
    game_state: Option<Box<dyn GameState>>,
    play_state: PlayState,
}

const BOARD_POS_X: usize = 0;
const BOARD_POS_Y: usize = 0;
// hold the board info  
impl MancalaBoard {
    pub fn new() -> Self{
        let settings: GameSettings = GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Random };
        Self {
            game_state: 
                match settings.mode {
                    GameMode::Capture => {
                        Some(Box::new(Capture{
                            game_board: <Capture as GameState>::generate_game_board(settings.difficulty),
                            in_play: Side::Bottom,
                            selected_cell: 0,
                        }))
                    }
                    GameMode::Avalanche => {
                        Some(Box::new(Avalanche{
                            game_board: <Avalanche as GameState>::generate_game_board(settings.difficulty),
                            in_play: Side::Bottom,
                            selected_cell: 0,
                        }))
                    }
                },
            play_state: PlayState::Config,
        }
    }

    fn draw_config(&self, printer: &Printer) {
        printer.print((0, 0), "Press <Enter> to Start!");
    }

    fn draw_playing(&self, printer: &Printer) {
        let offset: Printer = printer.offset(XY{x:BOARD_POS_X, y:BOARD_POS_Y});
        // will probably refactor how printing is done
        // currently based off array index but game_state is based off who is playing
        self.draw_base_layer(&offset);
        self.draw_cells(&offset);
        self.draw_arrow(&offset);
    }

    fn get_stuff(){
        
    }

    fn draw_cells(&self, printer: &Printer){
        for cell in 0..=13{
            let (x, y) = MancalaBoard::cell_to_xy(cell);
            if cell == self.game_state.get_selected_index() {
                printer.print((x, y), "╭╌╌╌╮");
                printer.print((x, y+1), &format!("╎ {:<2}╎", self.game_state.game_board[cell as usize]));
                printer.print((x, y+2), "╰╌╌╌╯");
            }else{
                printer.print((x, y), "╭───╮");
                printer.print((x, y+1), &format!("│ {:<2}│", self.game_state.game_board[cell as usize]));
                printer.print((x, y+2), "╰───╯");
            }
        }
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
                    _ => return EventResult::Ignored
                }
                EventResult::Consumed(None)
            }
            
            PlayState::Finish => EventResult::Ignored,
        }
    }

}