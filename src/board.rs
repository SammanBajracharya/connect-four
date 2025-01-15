use colored::Colorize;
use crossterm::{
    cursor,
    style::{self, Print},
    QueueableCommand,
};
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
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

    pub fn print(&self, stdout: &mut impl Write, start_x: u16, mut start_y: u16) -> anyhow::Result<()> {
        let text = "Connect 4".cyan();
        let width = 23;
        let text_length = "Connect 4".len();  // Use the length of the plain text for calculating padding
        let padding_left = (width - text_length) / 2;
        let padding_right = width - text_length - padding_left;

        stdout
            .queue(cursor::MoveTo(start_x, start_y))?
            .queue(style::Print(format!("{:>width$}", " ".repeat(padding_left) + &text.to_string() + &" ".repeat(padding_right))))?;
        start_y += 2;

        stdout
            .queue(cursor::MoveTo(start_x, start_y))?
            .queue(style::Print(format!(" ╭ {} ╮", "1  2  3  4  5  6  7")))?;
        start_y += 1;

        for y in 0..6 {
            stdout
                .queue(cursor::MoveTo(start_x, start_y))?
                .queue(style::Print(format!(" |")))?;
            for x in 0..7 {
                let piece_str = match self.board[x][y] {
                    Piece::None => " ◯ ".dimmed().to_string(),
                    Piece::Red => {
                        match self.highlights[x][y] {
                            true => format!("╏{}╏", "◈".red()),
                            false => format!("{}", " ◉ ".red()),
                        }
                    },
                    Piece::Blue => {
                        match self.highlights[x][y] {
                            true => format!("╏{}╏", "◈".blue()),
                            false => format!("{}", " ◉ ".blue()),
                        }
                    }
                };

                stdout.queue(Print(piece_str))?;
            }
            stdout.queue(Print("| "))?;
            start_y += 1;
        }
        stdout
            .queue(cursor::MoveTo(start_x, start_y))?
            .queue(style::Print(format!(" ╰─────────────────────╯ ")))?;

        Ok(())
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
        self.highlights = [[false; 6]; 7];
        for y in 0..6 {
            self.highlights[x][y] = true;
        }
    }

    pub fn check_for_win(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        let check_bound: [[(isize, isize); 2]; 4] = [
            [(-1, -1), (1, 1)], // Diagonal top-left to bottom-right
            [(1, -1), (-1, 1)], // Diagonal bottom-left to top-right
            [(1, 0), (-1, 0)],  // Horizontal
            [(0, 1), (0, -1)],  // Vertical
        ];

        for &directions in &check_bound {
            let mut count = 1;

            for &(dx, dy) in directions.iter() {
                let mut curr_x = x as isize;
                let mut curr_y = y as isize;

                if !(0..7).contains(&curr_x) || !(0..6).contains(&curr_y) { break; }

                if self.board[curr_x as usize][curr_y as usize] == piece { count += 1; }
                else { break; }

                curr_x += curr_x.saturating_add(dx);
                curr_y += curr_y.saturating_add(dy);
            }

            if count >= 4 { return true; }
        }

        false
    }
}
