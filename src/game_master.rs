use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use crate::game_config::{Config, Player, PlayerNb, PlayerKind};

const COL: usize = 7;
const ROW: usize = 6;
const NB_TURNS: usize = COL * ROW;

#[derive(Clone, PartialEq, Copy)]
pub struct Position {
    x: usize,
    y: usize
}

pub struct GameMaster {
    grid: Vec<Vec<Option<PlayerNb>>>,
    p1: Player,
    p2: Player,
    turn: PlayerNb,
    nb_turn: usize 
}

impl GameMaster {

    pub fn new(config: Config) -> Self {
        GameMaster {
            grid: vec![vec![None; COL]; ROW],
            p1: config.p1,
            p2: config.p2,
            turn: PlayerNb::P1,
            nb_turn: 0
        }
    }
    
    fn inc_diagonal_4(&self, player: PlayerNb, pos:Position) -> bool {
        unimplemented!();
    }

    fn inc_diaginal_4(&self, player: PlayerNb, pos:Position) -> bool {
        unimplemented!();
    }

    fn horizontal_4(&self, player: PlayerNb, row: usize) -> bool {
        let mut count: usize = 0;
        let mut col: usize = 0;

        while (col < COL) {
            if let Some(p) = self.grid[row][col] {
                match p {
                    player => count += 1,
                    _ => count = 0
                }
            }
            else {
                count = 0;
            }
            if count == 4 {
                return true;
            }
            col += 1;
        }
        false
    }

    fn vertical_4(&self, player: PlayerNb, col: usize) -> bool {
        let mut count: usize = 0;
        let mut row: usize = 0;

        while (row < ROW) {
            if let Some(p) = self.grid[row][col] {
                match p {
                    player => count += 1,
                    _ => count = 0
                }
            }
            else {
                count = 0;
            }
            if count == 4 {
                return true;
            }
            row += 1;
        }
        false
    }

    fn check_success(&self, pos: Position) -> bool {
        let player = self.grid[pos.x][pos.y].unwrap();
        let row = pos.x;
        let col = pos.y;
        self.vertical_4(self.turn, col) || self.horizontal_4(self.turn, row) 
        //|| inc_diagonal_4(self.turn, pos) || dec_diagonal_4(self.turn, pos)
    }

    fn check_column(&self, input: String) -> Result<Position, ColError> {
        let mut res = input.parse();
        let col: usize;
        match res {
            Ok(nb) => col = nb,
            _ => return Err(ColError::Invalid(input))
        }
        if col < 1 || col > 8 {
            return Err(ColError::WrongColNb(col))
        }
        let mut row = ROW - 1;
        while row >= 0 {
            if self.grid[row][col - 1].is_none() {
                return Ok(Position {x: row, y: col - 1});
            }
            if row == 0 {
                break;
            }
            row -= 1;
        }
        Err(ColError::FullCol(col))
    }

    fn check_full(&self) -> bool {
        self.nb_turn == NB_TURNS
    }

    fn fill_grid(&mut self, player: PlayerNb, pos: Position) {
        self.grid[pos.x][pos.y] = Some(player);
    }

    fn display_grid(&self) {
        println!("\n  1   2   3   4   5   6   7  ");
        println!("|---+---+---+---+---+---+---|");
        for row in self.grid.iter() {
            print!("|");
            for val in row.iter() {
                match val {
                    None => print!("   |"),
                    Some(p) => print!(" {} |", if *p == PlayerNb::P1 {"O"} else {"X"})
                }
            }
            println!("\n|---+---+---+---+---+---+---|");
        }
        println!("");
    }

    pub fn run(config: Config) -> Result <(), Box<dyn Error>> {
        let mut game_master = GameMaster::new(config);
        println!("\nHere the game begins !\n");
        loop {
            game_master.display_grid();
            /* add a condition in case we are in solo mode and it's computer turn */ 
            println!("P{}, it's your turn.\nPlease choose a column.\n",
                     match game_master.turn {
                         PlayerNb::P1 => 1,
                         PlayerNb::P2 => 2 });
            io::stdout().flush()?;
            let pos: Position;
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                match game_master.check_column(input[..].trim().to_string()) {
                    Err(e) => println!("{}\nPlease try again.\n", e),
                    Ok(p) => { pos = p; break}
                }
            }
            game_master.fill_grid(game_master.turn, pos);
            game_master.nb_turn += 1;
            if game_master.check_success(pos) {
                game_master.display_grid();
                println!("Congrats p{}, you won !\n", match game_master.turn {
                         PlayerNb::P1 => 1,
                         PlayerNb::P2 => 2 });
                return Ok(());
            }
            if game_master.check_full() {
                game_master.display_grid();
                println!("It's a draw !\n");
                return Ok(());
            }
            match game_master.turn {
                PlayerNb::P1 => game_master.turn = PlayerNb::P2,
                PlayerNb::P2 => game_master.turn = PlayerNb::P1
            }
        }
    }
}

pub enum ColError {
    Invalid(String),
    WrongColNb(usize),
    FullCol(usize)
}

impl fmt::Display for ColError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColError::Invalid(s) =>
                write!(f, "\"{}\" is an invalid proposition.", (s)),
            ColError::WrongColNb(nb) =>
                write!(f, "{} is not a correct column number.\n\
                    You should choose a number between 1 and 8 (included).", nb),
            ColError::FullCol(nb) =>
                write!(f, "Column {} is full. You have to choose another one.", nb)
        }
    }
}

impl fmt::Debug for ColError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ColError {}
