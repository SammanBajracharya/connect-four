use colored::Colorize;

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
