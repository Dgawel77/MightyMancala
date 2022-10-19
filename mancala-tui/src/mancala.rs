#[derive(Debug)]
enum GameMode {
    Capture,
    Avalanche
}

#[derive(Debug)]
enum Difficulty {
    Normal,
    Random
}

pub struct GameSettings{
    mode: GameMode,
    difficulty: Difficulty
}

#[derive(Debug)]
pub struct Mancala{
    game: [u8; 14],
    gameSetting: GameSettings
}

// holds game data
impl mancala{
    pub fn new(settings : GameSettings){
        
    }
}

impl std::fmt::Display for mancala{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(180);

        write!(f, "{}", s)
    }
}