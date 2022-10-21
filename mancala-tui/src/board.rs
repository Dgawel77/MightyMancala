use crate::mancala::{Mancala, GameSettings, GameMode, Difficulty};
use cursive::{
    event::{Event, EventResult, Key, MouseEvent},
    theme::ColorStyle,
    view::View,
    Printer, Vec2,
};

#[derive(Debug)]
pub enum BoardState {
    Config,
    Playing,
    Finish,
}

pub struct MancalaBoard {
    mancala: Mancala,
    focus: [usize; 2],
    //history: Vec<([usize; 2], u8)>,
    //redo: Vec<([usize; 2], u8)>,
    //undos: usize,
    //moves: usize,
    //hints: usize,
    //conflict: Option<[usize; 2]>,
    state: BoardState,
    //stopwatch: Stopwatch,
}

// hold the board info  
impl MancalaBoard {
    pub fn new() -> Self{
        let settings: GameSettings = GameSettings {mode: GameMode::Capture, difficulty: Difficulty::Random };
        Self {
            mancala: Mancala::new(settings),
            focus:  [0, 0],
            state: BoardState::Config
        }
    }

    fn draw_config(&self, printer: &Printer) {
        printer.print((0, 0), "Press <Enter> to Start!");
        printer.print((0, 1), "Press <Enter> to Start!");
        printer.print((0, 2), "Press <Enter> to Start!");
        printer.print((0, 3), "Press <Enter> to Start!");

    }

    fn draw_playing(&self, printer: &Printer) {
        printer.print((0, 0), "╔══════════════════════════════════════════════╗");
        printer.print((0, 1), "║        ╭───╮╭───╮╭───╮╭───╮╭───╮╭───╮        ║");
        printer.print((0, 2), &format!("║      ˂ │ {} ││ {} ││ {} ││ {} ││ {} ││ {} │ ˂      ║", self.mancala.game_board[12], self.mancala.game_board[11], self.mancala.game_board[10], self.mancala.game_board[9], self.mancala.game_board[8], self.mancala.game_board[7]));
        printer.print((0, 3), "║ ╭───╮  ╰───╯╰───╯╰───╯╰───╯╰───╯╰───╯  ╭───╮ ║");
        printer.print((0, 4), &format!("║ │ {} │    ↓                             │ {} │ ║", self.mancala.game_board[6], self.mancala.game_board[13]));
        printer.print((0, 5), "║ ╰───╯  ╭╌╌╌╮╭───╮╭───╮╭───╮╭───╮╭───╮  ╰───╯ ║");
        printer.print((0, 6), &format!("║      ˂ │ {} ││ {} ││ {} ││ {} ││ {} ││ {} │ ˂      ║", self.mancala.game_board[0], self.mancala.game_board[1], self.mancala.game_board[2], self.mancala.game_board[3], self.mancala.game_board[4], self.mancala.game_board[5]));
        printer.print((0, 7), "║        ╰╌╌╌╯╰───╯╰───╯╰───╯╰───╯╰───╯        ║");
        printer.print((0, 8), "╚══════════════════════════════════════════════╝");
        printer.print((0, 9), &format!("{:?}\n  ", self.mancala.game_board));
    }

    fn coord_to_xy(coord: [usize; 2]) -> (usize, usize) {
        const C: [usize; 9] = [1, 2, 3, 5, 6, 7, 9, 10, 11];
        (C[coord[1]], C[coord[0]])
    }

    fn xy_to_coord(xy: (usize, usize)) -> Option<[usize; 2]> {
        const C: [usize; 13] = [
            usize::MAX,
            0,
            1,
            2,
            usize::MAX,
            3,
            4,
            5,
            usize::MAX,
            6,
            7,
            8,
            usize::MAX,
        ];
        let x = C[xy.0];
        let y = C[xy.1];
        if x != usize::MAX && y != usize::MAX {
            Some([x, y])
        } else {
            None
        }
    }
}

// holds how to draw the board on the screen
impl View for MancalaBoard {
    fn draw(&self, printer: &Printer) {
        match self.state {
            BoardState::Config => self.draw_config(printer),
            BoardState::Playing => self.draw_playing(printer),
            BoardState::Finish => (),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        //  Vec2::new(19, 19)
        Vec2::new(100, 50)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.state {
            BoardState::Config => {
                match event {
                    Event::Key(Key::Enter) => {
                        self.state = BoardState::Playing;
                    }
                    _ => return EventResult::Ignored,
                }
                EventResult::Consumed(None)
            }
            BoardState::Playing => EventResult::Ignored,
            BoardState::Finish => EventResult::Ignored,
        }
    }

}