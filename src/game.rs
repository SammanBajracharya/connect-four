use crossterm::event::KeyCode;
use crossterm::style;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, ExecutableCommand, QueueableCommand};

use colored::*;

use strip_ansi::strip_ansi;

use crate::board::{Board, Piece};
use std::io::{self, Stdout, Write};

enum Action {
    Quit,

    Left,
    Right,
    Push,
}

pub struct Game {
    stdout: Stdout,
    board: Board,
    current_player: Piece,
    game_over: bool,
    size: (u16, u16),
}

impl Game {
    pub fn new() -> anyhow::Result<Self> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        stdout
            .execute(EnterAlternateScreen)?
            .execute(cursor::Hide)?
            .execute(terminal::Clear(terminal::ClearType::All))?;

        Ok(Game {
            stdout,
            board: Board::new(),
            current_player: Piece::Red,
            game_over: false,
            size: terminal::size()?,
        })
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        const BOARD_WIDTH: usize = 25;
        const BOARD_HEIGHT: usize = 8;

        let start_x = ((self.size.0 as i32 - BOARD_WIDTH as i32) / 2).max(0) as u16;
        let mut start_y = ((self.size.1 as i32 - BOARD_HEIGHT as i32) / 2).max(0) as u16;

        let mut message = format!("{}", "Connect 4".cyan());

        self.centred_print(message, BOARD_WIDTH, start_x, &mut start_y)?;
        start_y += 1;

        let board_str = self.board.get_board();
        self.centred_print(board_str, BOARD_WIDTH, start_x, &mut start_y)?;
        start_y += 1;

        message = self.get_game_status_str();
        self.centred_print(message, BOARD_WIDTH, start_x, &mut start_y)?;
        start_y += 1;

        message = if self.game_over { "Game Over".to_string() } else { "Game Running".to_string() };
        self.centred_print(message, BOARD_WIDTH, start_x, &mut start_y)?;

        self.stdout.flush()?;
        Ok(())
    }

    fn get_game_status_str(&self) -> String {
        let current_player_str = match self.current_player {
            Piece::Red => "Red".red().to_string(),
            Piece::Blue => "Blue".blue().to_string(),
            Piece::None => String::new(),
        };

        let on_going_game_str = match self.current_player {
            Piece::None => "Press any key to start!".to_string(),
            _ => format!("It's {}'s turn!", current_player_str),
        };

        let game_over_str = match self.current_player {
            Piece::Red => format!("{} wins!", current_player_str),
            Piece::Blue => format!("{} wins!", current_player_str),
            Piece::None => String::new(),
        };

        match self.game_over {
            true => game_over_str,
            false => on_going_game_str,
        }
    }

    fn centred_print(&mut self, strings: String, board_width: usize, x: u16, y: &mut u16) -> anyhow::Result<()> {
        self.stdout
            .execute(cursor::MoveTo(x, *y))?;
        let line_y = *y;

        for (i, string) in strings.lines().enumerate() {
            let line_y = line_y + i as u16;
            let stripped_string = strip_ansi(string);
            // NO IDEA WHATS GOING ON BUT IT WORKS
            let text_len = stripped_string.len().min(board_width);
            let center_x = x + (board_width / 2) as u16 - (text_len / 2) as u16;

            self.stdout
                .queue(cursor::MoveTo(0, line_y))?
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;

            self.stdout
                .queue(cursor::MoveTo(center_x, line_y))?
                .queue(style::Print(string))?;

            *y += 1;
        }
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;
            if let Some(action) = self.handle_event(event::read()?)? {
                match action {
                    Action::Quit => break,
                    Action::Left => {
                        self.board.cx = self.board.cx.saturating_sub(1);
                        self.board.highlights = Some(self.board.cx);
                    },
                    Action::Right => {
                        if self.board.cx < 6 { self.board.cx += 1; }
                        self.board.highlights = Some(self.board.cx);
                    },
                    Action::Push => {
                        self.board.drop_piece(self.board.cx, self.current_player);
                        self.game_over =
                            self.board.check_for_win(self.board.cx) ||
                            self.board.is_board_full();
                        if self.game_over { break; }
                        else { self.change_player(); }
                    },
                }
            }
        }

        Ok(())
    }

    fn change_player(&mut self) {
        match self.current_player {
            Piece::Red | Piece::None => self.current_player = Piece::Blue,
            Piece::Blue => self.current_player = Piece::Red,
        }
    }

    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        if matches!(ev, event::Event::Resize(_, _)) {
            self.size = terminal::size()?;
        }

        let action = match ev {
            event::Event::Key(event) => match event.code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('h') | KeyCode::Left => Some(Action::Left),
                KeyCode::Char('l') | KeyCode::Right => Some(Action::Right),
                KeyCode::Enter => Some(Action::Push),
                _ => None,
            }
            _ => None,
        };

        Ok(action)
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        self.stdout
            .execute(LeaveAlternateScreen)?
            .execute(cursor::Show)?;
        disable_raw_mode()?;

        Ok(())
    }
}
