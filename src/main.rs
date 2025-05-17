mod util;

use std::{collections::HashSet, error::Error, io::stdout};

use ratatui::{buffer::Buffer, crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}}, layout::Rect, prelude::CrosstermBackend, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame, Terminal};
use util::select_random_word;

struct App {
    selected_word: String,
    guessed_letters: HashSet<char>,
    strikes: u8,
    run: bool,
}

impl App {
    pub fn init() -> Self {
        Self {
            selected_word: select_random_word(),
            guessed_letters: HashSet::new(),
            strikes: 0,
            run: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while self.run {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;

            if let Event::Key(event) = event::read()? {
                self.handle_keypress(event);
            }
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_keypress(&mut self, event: KeyEvent) {
        if event.kind == KeyEventKind::Press {
            match event.code {
                KeyCode::Esc => {
                    self.quit();
                },
                _ => {},
            }
        }
    }

    pub fn quit(&mut self) {
        self.run = false;
    }
}

impl Widget for &App {
    fn render(self, layout: Rect, buffer: &mut Buffer) {
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let mut app = App::init();
    let run_result = app.run(&mut terminal);

    ratatui::restore();

    run_result
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyModifiers;

    use super::*;

    #[test]
    fn inits_app() {
        let app = App::init();

        assert!(!app.selected_word.is_empty());
        assert!(app.guessed_letters.is_empty());
        assert_eq!(app.strikes, 0);
        assert!(app.run);
    }

    #[test]
    fn quits_app() {
        let mut app = App::init();

        app.handle_keypress(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)
        );

        assert!(!app.run);
    }

    #[test]
    fn runs_app() {
        let app = App::init();

        assert!(app.run);
    }
}
