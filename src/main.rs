use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::ascii::AsciiExt;
use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug)]
struct Sudoku {
    board: [Tile; 81],
    count: isize,
}
#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
enum Tile {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Empty,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::Empty => write!(f, " "),
            Tile::One => write!(f, "1"),
            Tile::Two => write!(f, "2"),
            Tile::Three => write!(f, "3"),
            Tile::Four => write!(f, "4"),
            Tile::Five => write!(f, "5"),
            Tile::Six => write!(f, "6"),
            Tile::Seven => write!(f, "7"),
            Tile::Eight => write!(f, "8"),
            Tile::Nine => write!(f, "9"),
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..9 {
            write!(f, "\n--------------------------------------\n")?;
            for j in 0..9 {
                write!(f, "| {} ", self.board[(i * 9) + j])?;
            }
            write!(f, "|")?;
        }
        write!(f, "\n--------------------------------------\n")?;
        return Ok(());
    }
}

impl Sudoku {
    fn find_empty(&self) -> Option<usize> {
        for i in 0..self.board.len() {
            if self.board[i] == Tile::Empty {
                return Some(i);
            };
        }
        return None;
    }

    fn place_tile(&mut self, tile: Tile, i: usize) -> Result<usize, &'static str> {
        if self.board[i] == Tile::Empty {
            self.board[i] = tile;
            return Ok(i);
        } else {
            return Err("Tile already occupied!");
        }
    }

    fn remove_tile(&mut self, i: usize) -> Result<Tile, &'static str> {
        if self.board[i] != Tile::Empty {
            let removed = self.board[i];
            self.board[i] = Tile::Empty;
            return Ok(removed);
        } else {
            return Err("Tile is already empty");
        }
    }
    fn check_legal(&self, tile: Tile, i: usize) -> bool {
        let row: usize = i / 9;
        for j in 0..9 {
            if self.board[j + (9 * row)] == tile {
                return false;
            }
        }
        let column = i % 9;

        for j in 0..9 {
            if self.board[column + (j * 9)] == tile {
                return false;
            }
        }
        let square_row = (row / 3) * 3;
        let square_column = (column / 3) * 3;
        for j in square_row..square_row + 3 {
            for k in square_column..square_column + 3 {
                if tile == self.board[(j * 9) + k] {
                    return false;
                }
            }
        }
        return true;
    }

    fn gen_legal_moves(&self, i: usize) -> Option<Vec<Tile>> {
        let mut legal_moves: Vec<Tile> = Vec::new();
        for tile in Tile::iter() {
            if self.check_legal(tile, i) {
                legal_moves.push(tile);
            }
        }
        if legal_moves.len() > 0 {
            return Some(legal_moves);
        } else {
            return None;
        }
    }
    fn solve(&mut self) -> bool {
        let empty_tile = match self.find_empty() {
            Some(num) => num,
            None => return true,
        };

        let legal_moves = match self.gen_legal_moves(empty_tile) {
            Some(moves) => moves,
            None => return false,
        };
        for tile in legal_moves {
            self.place_tile(tile, empty_tile).unwrap();
            if self.solve() {
                return true;
            }
            self.remove_tile(empty_tile).unwrap();
        }
        return false;
    }

    fn fill_board(&mut self) -> bool {
        let empty_tile = match self.find_empty() {
            Some(tile) => tile,
            None => return true,
        };
        let mut legal_moves = match self.gen_legal_moves(empty_tile) {
            Some(legal_moves) => legal_moves,
            None => return false,
        };
        legal_moves.shuffle(&mut thread_rng());
        for tile in legal_moves {
            self.place_tile(tile, empty_tile).unwrap();
            if self.fill_board() {
                return true;
            }
            self.remove_tile(empty_tile).unwrap();
        }
        return false;
    }
    fn is_full(&self) -> bool{
        for tile in self.board{
            if tile == Tile::Empty{
                return false;
            }
        }
        return true;
    }

    fn check_board(&mut self) -> bool{
      let empty_tile = match self.find_empty() {
            Some(num) => num,
            None => return true,
        };

        let legal_moves = match self.gen_legal_moves(empty_tile) {
            Some(moves) => moves,
            None => return false,
        };
        for tile in legal_moves {
            self.place_tile(tile, empty_tile).unwrap();
            if self.is_full(){
                self.count += 1;
                break
            }
            if self.check_board() {
                return true;
            }
            self.remove_tile(empty_tile).unwrap();
        }
        return false;
    }

    fn gen_board(&mut self, mode: Mode) {
        let attempts = match mode {
            Mode::Easy => 300,
            Mode::Medium => 500,
            Mode::Hard => 700,
        };
        self.board = [Tile::Empty ; 81];
        self.fill_board();
        for _ in 0..attempts{
            let mut rng = rand::thread_rng();
            let cord = rng.gen_range(0..81);
            if self.board[cord] != Tile::Empty{
                let deleted = self.remove_tile(cord).unwrap();
                let backup_board = self.board.clone();
                self.count = 0;
                self.check_board();
                self.board = backup_board;
                if self.count != 1{
                    self.place_tile(deleted, cord).unwrap();
                }

            }
        }
        
    }
}

fn main() {
    loop{
    let mut sudoku1 = Sudoku {
        board: [Tile::Empty; 81],
        count: 0
    };
    let mut s = String::new();
    println!("Please enter desired difficulty(Easy/Medium/Hard): ");
    match std::io::stdin()
    .read_line(&mut s){
        Ok(s) => s,
        Err(err) =>{
            println!("{}",err);
            break;
        }
    };
    let diffucilty = match s.to_lowercase().trim(){
        "easy" => Mode::Easy,
        "medium" => Mode::Medium,
        "hard" => Mode::Hard,
        _ => {
            println!("Couldn't parse the string. restarting!");
            return;
        }
    };
    sudoku1.gen_board(diffucilty);
    println!("{}", sudoku1);
    println!("Type s or Solve to see the answer!");
    let mut line = String::new();
     match std::io::stdin()
    .read_line(&mut line){
       Ok(line) => line,
       Err(err) => {
        println!("{}",err);
        break;
       }
    };
    if line.to_lowercase().trim() == "s" || line.to_lowercase().trim() == "solve"{
    sudoku1.solve();
    println!("{}", sudoku1);
    }

    }
    
}
