use colored::Colorize;

#[derive(Copy, Clone)]
pub enum Piece {
    None,
    Red,
    Blue,
}

pub struct Board {
    board: [[Piece; 6]; 7],
    highlights: [[bool; 6]; 7],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Piece::None; 6]; 7],
            highlights: [[false; 6]; 7],
        }
    }

    pub fn print(&self) {
        println!(" ╭ {} ╮", "1  2  3  4  5  6  7");
        for y in 0..6 {
            print!(" |");
            for x in 0..7 {
                match self.board[x][y] {
                    Piece::None => print!("{}", " ◯ ".dimmed()),
                    Piece::Red => {
                        match self.highlights[x][y] {
                            true => print!("╏{}╏", "◈".red()),
                            false => print!("{}", " ◉ ".red()),
                        }
                    },
                    Piece::Blue => {
                        match self.highlights[x][y] {
                            true => print!("╏{}╏", "◈".blue()),
                            false => print!("{}", " ◉ ".blue()),
                        }
                    }
                }
            }
            println!("| ");
        }
        println!(" ╰─────────────────────╯");
    }

    pub fn drop_piece(&mut self, x: usize, piece: Piece) {
        for y in (0..6).rev() {
            match self.board[x][y] {
                Piece::None => {
                    self.board[x][y] = piece;
                    break;
                },
                _ => continue,
            }
        }
    }

    pub fn highlight_col(&mut self, x: usize) {
        for y in 0..6 {
            self.highlights[x][y] = true;
        }
    }
}
