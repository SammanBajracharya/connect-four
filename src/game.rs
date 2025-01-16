use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, ExecutableCommand, QueueableCommand};

use colored::*;

use strip_ansi::strip_ansi;
use unicode_segmentation::UnicodeSegmentation;

use crate::board::{Board, Piece};
use std::io::{self, Stdout, Write};

enum Action {
    Quit,

    Left,
    Right,
    Push,

    ChangeGameState(GameState),
}

#[derive(PartialEq)]
enum GameState {
    Start,
    Playing,
    GameOver { winner: Option<Piece> },
}

pub struct Game {
    stdout: Stdout,
    board: Board,
    current_player: Piece,
    game_state: GameState,
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
            game_state: GameState::Start,
            size: terminal::size()?,
        })
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        const BOARD_HEIGHT: usize = 8;

        let mut start_y = ((self.size.1 as i32 - BOARD_HEIGHT as i32) / 2).max(0) as u16;

        let mut message = format!("{}", "Connect 4".cyan());

        self.centred_print(message, &mut start_y)?;

        message = self.get_game_status_str();
        self.centred_print(message, &mut start_y)?;
        start_y += 1;

        let board_str = self.board.get_board();
        self.centred_print(board_str, &mut start_y)?;
        start_y += 1;

        if self.game_state == GameState::Playing {
            message = format!(
                "Select Column: {}\n{}",
                (self.board.cx + 1).to_string().yellow(),
                "Press enter to confirm.",
            );
            self.centred_print(message, &mut start_y)?;

            start_y += 1;
        }

        message = format!("{}", "Press Ctrl+C key at any time.".dimmed());
        self.centred_print(message, &mut start_y)?;

        self.stdout.flush()?;
        Ok(())
    }

    fn get_game_status_str(&self) -> String {
        let current_player_str = match self.current_player {
            Piece::Red => "Red".red().to_string(),
            Piece::Blue => "Blue".blue().to_string(),
            Piece::None => String::new(),
        };

        match self.game_state {
            GameState::Start => "Press any key to start!".to_string(),
            GameState::Playing => format!("It's {}'s turn!", current_player_str),
            GameState::GameOver { winner } => {
                match winner {
                    Some(Piece::Red) => format!("{} wins!", "Red".red()),
                    Some(Piece::Blue) => format!("{} wins!", "Blue".blue()),
                    Some(Piece::None) | None => "It's a draw!".to_string(),
                }
            }
        }
    }

    fn centred_print(&mut self, strings: String, y: &mut u16) -> anyhow::Result<()> {
        let line_y = *y;
        for (i, string) in strings.lines().enumerate() {
            let line_y = line_y + i as u16;
            let text_len = strip_ansi(string).graphemes(true).count();
            let center_x = (self.size.0 - text_len as u16) / 2;

            self.stdout
                .queue(cursor::MoveTo(0, line_y))?
                .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

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
                        let game_over =
                            self.board.check_for_win(self.board.cx) ||
                            self.board.is_board_full();
                        if !game_over { self.change_player(); }
                        else { self.game_state = GameState::GameOver { winner: Some(self.current_player) } }
                    },
                    Action::ChangeGameState(GameState::Start) => {
                        self.board.reset();
                        self.current_player = Piece::Red;
                        self.game_state = GameState::Start;
                    },
                    Action::ChangeGameState(GameState::Playing) => {
                        self.board.highlights = Some(self.board.cx);
                        self.game_state = GameState::Playing;
                    },
                    Action::ChangeGameState(state) => self.game_state = state,
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        if matches!(ev, event::Event::Resize(_, _)) {
            self.size = terminal::size()?;
        }

        if let event::Event::Key(event) = ev {
            if let (KeyModifiers::CONTROL, KeyCode::Char('c')) = (event.modifiers, event.code) { return Ok(Some(Action::Quit)) }
        };

        let action = match self.game_state {
            GameState::Start => {
                if let event::Event::Key(_) = ev {
                    Some(Action::ChangeGameState(GameState::Playing))
                } else {
                    None
                }
            }
            GameState::Playing => self.handle_playing_event(ev)?,
            GameState::GameOver { .. } => {
                if let event::Event::Key(_) = ev {
                    Some(Action::ChangeGameState(GameState::Start))
                } else {
                    None
                }
            }
        };

        Ok(action)
    }

    fn handle_playing_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        let action = match ev {
            event::Event::Key(event) => match event.code {
                KeyCode::Char('h') | KeyCode::Left => Some(Action::Left),
                KeyCode::Char('l') | KeyCode::Right => Some(Action::Right),
                KeyCode::Enter => Some(Action::Push),
                _ => None,
            }
            _ => None,
        };

        Ok(action)
    }

    fn change_player(&mut self) {
        match self.current_player {
            Piece::Red | Piece::None => self.current_player = Piece::Blue,
            Piece::Blue => self.current_player = Piece::Red,
        }
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        self.stdout
            .execute(LeaveAlternateScreen)?
            .execute(cursor::Show)?;
        disable_raw_mode()?;

        Ok(())
    }
}
