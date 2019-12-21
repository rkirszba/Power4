use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use crate::game_config::{Config, Player, PlayerNb, PlayerKind};
use std::convert::TryInto;

const COL: usize = 7;
const ROW: usize = 6;
const NB_TURNS: usize = COL * ROW;

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct GameMaster {
    grid: [[Option<PlayerNb>; COL]; ROW],
    p1: Player,
    p2: Player,
    turn: PlayerNb,
    nb_turn: usize 
}

impl GameMaster {

    pub fn new(config: Config) -> Self {
        GameMaster {
            grid: [[None; COL]; ROW],
            p1: config.p1,
            p2: config.p2,
            turn: PlayerNb::P1,
            nb_turn: 0
        }
    }

    fn check_success(&self, pos: Position) -> bool {
        let Position { x, y } = pos;
        // 4 directions: horizontal, vertical, diagonal 1 and diagonal 2
        let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];
        directions.iter().any(|&(dx, dy)|
            (0..4).any(|start| // All the possible indices at which the 4 consecutive pieces can start
                (0..4).map(|i| { // Check that all the four belong to the current player
                    let col: usize = (x as i32 + (i - start) * dx).try_into().ok()?;
                    let row: usize = (y as i32 + (i - start) * dy).try_into().ok()?;
                    *self.grid.get(row)?.get(col)?
                }).all(|v| v == Some(self.turn))
            )
        )
    }

    fn check_column(&self, input: String) -> Result<Position, ColError> {
        let x = match input.parse() {
            Ok(i) if 1 <= i && i <= 7 => Ok(i - 1),
            Ok(i) => Err(ColError::WrongColNb(i)),
            Err(_) => Err(ColError::Invalid(input)),
        }?;
        (0..ROW).rev()
            .flat_map(|y| // Create an iterator of free positions
                if self.grid[y][x].is_none() { Some(Position { x, y }) } else { None }
            ).next() // Take the first free position
            .ok_or(ColError::FullCol(x)) // Return an error if there were no free positions
    }

    fn check_full(&self) -> bool {
        self.nb_turn == NB_TURNS
    }

    fn fill_grid(&mut self, player: PlayerNb, pos: Position) {
        self.grid[pos.y][pos.x] = Some(player);
    }

    fn display_grid(&self) {
        println!("\n  1   2   3   4   5   6   7  ");
        println!("|---+---+---+---+---+---+---|");
        for row in self.grid.iter() {
            print!("|");
            for val in row.iter() {
                match val {
                    None => print!("   |"),
                    Some(p) => print!(" {} |", if *p == PlayerNb::P1 { "O" } else { "X" })
                }
            }
            println!("\n|---+---+---+---+---+---+---|");
        }
        println!();
    }

    fn process_computer_turn(&self) -> Position {
        unimplemented!();
    }

    fn process_user_turn(&self) -> Result <Position, Box<dyn Error>> {
        println!("{:?}, it's your turn.\nPlease choose a column.\n", self.turn);
        io::stdout().flush()?;
        let pos: Position;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match self.check_column(input[..].trim().to_string()) {
                Err(e) => println!("{}\nPlease try again.\n", e),
                Ok(p) => { pos = p; break}
            }
        }
        Ok(pos)
    }

    pub fn run(config: Config) -> Result <(), Box<dyn Error>> {
        let mut game_master = GameMaster::new(config);
        println!("\nHere the game begins !\n");
        loop {
            game_master.display_grid();
            let pos: Position = if
            (game_master.turn == PlayerNb::P1 && game_master.p1.kind == PlayerKind::Computer) ||
                (game_master.turn == PlayerNb::P2 && game_master.p2.kind == PlayerKind::Computer) {
                game_master.process_computer_turn()
            } else {
                game_master.process_user_turn()?
            };
            game_master.fill_grid(game_master.turn, pos);
            game_master.nb_turn += 1;
            if game_master.check_success(pos) {
                game_master.display_grid();
                println!("Congrats {:?}, you won !\n", game_master.turn);
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

#[derive(Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::game_master::{GameMaster, ROW, COL, Position};
    use crate::game_config::{Player, PlayerNb::{self, P1, P2}, PlayerKind};

    const A: Option<PlayerNb> = Some(P1);
    const B: Option<PlayerNb> = Some(P2);
    const O: Option<PlayerNb> = None;

    fn assert_success_3_2(grid: [[Option<PlayerNb>; COL]; ROW]) {
        assert_eq!(true, make_grid(grid).check_success(Position { x: 3, y: 2 }));
    }

    fn assert_no_success_3_2(grid: [[Option<PlayerNb>; COL]; ROW]) {
        assert_eq!(false, make_grid(grid).check_success(Position { x: 3, y: 2 }));
    }

    fn make_grid(grid: [[Option<PlayerNb>; COL]; ROW]) -> GameMaster {
        assert_eq!(grid.len(), ROW);
        assert_eq!(grid[0].len(), COL);
        GameMaster {
            grid,
            p1: Player { nb: P1, kind: PlayerKind::User },
            p2: Player { nb: P2, kind: PlayerKind::User },
            turn: P1,
            nb_turn: 0,
        }
    }

    #[test]
    fn test_check_empty_grid() {
        assert_no_success_3_2([
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_vertical() {
        assert_success_3_2([
            [O, O, O, O, O, O, O],
            [O, O, O, O, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_vertical_top() {
        assert_success_3_2([
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, B, O, O, O],
            [O, O, O, B, O, O, O],
        ]);
    }

    #[test]
    fn test_check_no_success_vertical_non_continuous() {
        assert_no_success_3_2([
            [O, O, O, O, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, B, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
            [O, O, O, A, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_horizontal() {
        assert_success_3_2([
            [O, O, O, A, O, O, O],
            [O, O, O, B, O, O, O],
            [O, A, A, A, A, O, O],
            [O, B, B, A, A, O, O],
            [O, A, B, B, B, O, O],
            [O, B, B, B, A, O, O],
        ]);
    }

    #[test]
    fn test_check_no_success_horizontal_missing() {
        assert_no_success_3_2([
            [O, O, O, A, O, O, O],
            [O, O, O, B, O, O, O],
            [O, B, A, A, A, O, O],
            [O, B, B, A, A, O, O],
            [O, A, B, B, B, O, O],
            [O, B, B, B, A, O, O],
        ]);
    }

    #[test]
    fn test_check_success_diagonal1() {
        assert_success_3_2([
            [O, O, O, A, O, O, O],
            [O, O, O, B, O, O, O],
            [O, B, A, A, A, O, O],
            [O, B, B, A, A, O, O],
            [O, A, B, B, B, A, O],
            [O, B, B, B, A, A, A],
        ]);
    }

    #[test]
    fn test_check_success_diagonal2() {
        assert_success_3_2([
            [O, O, O, A, O, O, O],
            [O, O, O, B, A, O, O],
            [O, B, O, A, A, O, O],
            [O, B, A, A, A, O, O],
            [O, A, B, B, B, O, O],
            [O, B, B, B, A, O, O],
        ]);
    }

    #[test]
    fn test_check_column() {
        let grid = make_grid([
            [O, O, O, A, O, O, O],
            [O, O, O, B, A, O, O],
            [O, O, O, A, A, O, O],
            [O, B, A, A, A, O, O],
            [O, A, B, B, B, O, O],
            [O, B, B, B, A, O, O],
        ]);
        assert_eq!(
            Ok(Position { x: 1, y: 2 }),
            grid.check_column("2".into())
        );
    }
}