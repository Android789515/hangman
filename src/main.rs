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

    pub fn get_masked_word(&self) -> String {
        self.selected_word.chars()
            .map(|letter| {
                if self.guessed_letters.contains(&letter) {
                    letter
                } else {
                    '_'
                }
            })
            .collect::<String>()
    }

    fn add_guess(&mut self, guess: char) -> bool {
        self.guessed_letters.insert(guess)
    }

    fn strike(&mut self) {
        self.strikes += 1;
    }

    fn handle_guess(& mut self, guess: char) {
        let is_wrong = !self.selected_word.contains(guess);

        if is_wrong {
            self.strike();
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
                KeyCode::Char(key) => {
                    let new_guess = self.add_guess(key);

                    if new_guess {
                        self.handle_guess(key);
                    }
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
    fn masks_selected_word() {
        let app = App::init();

        let masked_word = (0..app.get_masked_word().len())
            .map(|_| '_')
            .collect::<String>();

        assert_eq!(app.get_masked_word(), masked_word);
    }

    #[test]
    fn strikes_on_incorrect_guess() {
        let mut app = App::init();

        let incorrect_letter = ('a'..='z').find(|&letter| {
            !app.selected_word.contains(letter)
        }).unwrap();

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char(incorrect_letter), KeyModifiers::NONE)
        );

        assert_eq!(app.strikes, 1);

        let incorrect_letter = ('a'..='z').rfind(|&letter| {
            !app.selected_word.contains(letter)
        }).unwrap();

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char(incorrect_letter), KeyModifiers::NONE)
        );

        assert_eq!(app.strikes, 2);
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
    fn does_not_add_duplicate_guess() {
        let mut app = App::init();

        let letter = 'p';

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char(letter), KeyModifiers::NONE)
        );

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char(letter), KeyModifiers::NONE)
        );

        assert!(app.guessed_letters.len() == 1);
    }

    #[test]
    fn adds_guess() {
        let mut app = App::init();

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)
        );

        assert!(app.guessed_letters.contains(&'c'));
    }

    #[test]
    fn runs_app() {
        let app = App::init();

        assert!(app.run);
    }
}
