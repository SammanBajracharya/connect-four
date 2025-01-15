use crossterm::event::KeyCode;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, ExecutableCommand};

use crate::board::{Board, Piece};
use std::io::{self, Stdout, Write};

enum Action {
    Quit
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
        // TODO: Draw Screen
        const BOARD_WIDTH: u32 = 23;
        const BOARD_HEIGHT: u32 = 8;

        let start_x = ((self.size.0 as i32 - BOARD_WIDTH as i32) / 2).max(0) as u16;
        let start_y = ((self.size.1 as i32 - BOARD_HEIGHT as i32) / 2).max(0) as u16;

        self.board.print(&mut self.stdout, start_x, start_y)?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // TODO: Main Game Lopo
        loop {
            self.draw()?;
            if let Some(action) = self.handle_event(event::read()?)? {
                match action {
                    Action::Quit => break,
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, ev: event::Event) -> anyhow::Result<Option<Action>> {
        if matches!(ev, event::Event::Resize(_, _)) {
            self.size = terminal::size()?;
        }

        let action = match ev {
            event::Event::Key(event) => match(event.code) {
                KeyCode::Char('q') => Some(Action::Quit),
                _ => None,
            }
            _ => None,
        };

        Ok(action)
    }

    fn cleanup(&mut self) -> anyhow::Result<()> {
        self.stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
