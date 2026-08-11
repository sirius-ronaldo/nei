use std::io::{self, Stdout};

use crossterm::cursor::{Hide, SetCursorStyle, Show};
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

pub struct TerminalGuard {
    pub stdout: Stdout,
}

impl TerminalGuard {
    pub fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let mut guard = Self { stdout };

        if let Err(error) = execute!(
            &mut guard.stdout,
            EnterAlternateScreen,
            Clear(ClearType::All),
            Hide,
            SetCursorStyle::SteadyBlock,
        ) {
            // O raw mode já foi habilitado; restaure-o se a configuração falhar.
            let _ = terminal::disable_raw_mode();
            return Err(error);
        }

        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // A restauração é de melhor esforço, pois Drop não pode retornar erro de I/O.
        let _ = execute!(
            self.stdout,
            Show,
            SetCursorStyle::DefaultUserShape,
            LeaveAlternateScreen
        );
        let _ = terminal::disable_raw_mode();
    }
}
