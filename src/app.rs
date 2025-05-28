use std::{collections::HashSet, error::Error};

use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout}, style::Stylize, symbols::border, text::Line, widgets::{Block, Paragraph}, DefaultTerminal, Frame};

use crate::figure::Figure;

use super::util::select_random_word;

#[derive(PartialEq)]
enum GameOverState {
    Win,
    Lose,
}

pub struct App {
    selected_word: String,
    guessed_letters: HashSet<char>,
    strikes: u8,
    grace: u8,
    run: bool,
    game_over_state: Option<GameOverState>,
}

impl App {
    pub fn init() -> Self {
        Self::init_with_grace(Self::MAX_GRACE)
    }

    pub fn init_with_grace(grace: u8) -> Self {
        Self {
            selected_word: select_random_word(),
            guessed_letters: HashSet::new(),
            strikes: 0,
            grace,
            run: true,
            game_over_state: None,
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

    pub const MAX_STRIKES: u8 = 6;
    pub const MAX_GRACE: u8 = 3;
    fn strike(&mut self) {
        let should_be_graceful = self.grace > 0 && rand::random_bool(1.0 / 2.0);

        if should_be_graceful {
            self.grace -= 1;
        } else {
            self.strikes += 1;
        }

        if self.strikes == Self::MAX_STRIKES {
            self.set_game_state(GameOverState::Lose);
        }
    }

    fn set_game_state(&mut self, state: GameOverState) {
        self.game_over_state = Some(state);
    }

    fn should_win(&self) -> bool {
        self.strikes < Self::MAX_STRIKES
        && self.get_masked_word() == self.selected_word
    }

    fn handle_guess(& mut self, guess: char) {
        let is_correct = self.selected_word.contains(guess);

        if is_correct {
            if self.should_win() {
                self.set_game_state(GameOverState::Win);
            }
        } else {
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
        let app_title = Line::from(" Hangman ".bold());

        let controls = Line::from(
            Vec::from([
                " <Esc> Quit ".into(),
            ])
        );

        let app_block = Block::bordered()
            .title(app_title.centered())
            .title_bottom(controls.centered().bold())
            .border_set(border::ROUNDED);

        frame.render_widget(app_block, frame.area());

        let [ figure_area, game_info_area ] = Layout::horizontal([
            Constraint::Length(50),
            Constraint::Fill(1),
        ])
            .horizontal_margin(6)
            .vertical_margin(5)
            .areas(frame.area());

        let figure = Paragraph::new(Figure::get_state(self.strikes as usize));

        frame.render_widget(figure, figure_area);

        if let Some(game_over_state) = &self.game_over_state {
            let text = match game_over_state {
                GameOverState::Win => String::from("You Won!"),
                GameOverState::Lose => String::from("You have lost!"),
            }
                + &format!("  The word was {}", self.selected_word);

            frame.render_widget(
                Paragraph::new(text),
                game_info_area,
            );
        } else {
            let [ word_area, guessed_letters_area ] = Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).areas(game_info_area);
    
            let word = Paragraph::new(self.get_masked_word());
            let guessed_letters = Paragraph::new(
                self.guessed_letters.iter().collect::<String>()
            );
    
            frame.render_widget(word, word_area);
            frame.render_widget(guessed_letters, guessed_letters_area);
        }
    }

    pub fn handle_keypress(&mut self, event: KeyEvent) {
        if event.kind == KeyEventKind::Press {
            match event.code {
                KeyCode::Esc => {
                    self.quit();
                },
                KeyCode::Char(key) if key.is_alphabetic() && self.game_over_state.is_none() => {
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
    fn only_add_letter_guess() {
        let mut app = App::init();

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char(';'), KeyModifiers::NONE)
        );

        assert!(app.guessed_letters.is_empty());
    }

    #[test]
    fn game_over_when_strikeout() {
        let mut app = App::init_with_grace(0);

        (0..App::MAX_STRIKES).for_each(|_| {
            app.strike();
        });

        assert_eq!(app.strikes, 6);
        assert!(app.game_over_state.is_some_and(|state| {
            state == GameOverState::Lose
        }), "Strikes were {}", app.strikes);
    }

    #[test]
    fn wins_when_all_letters_guessed() {
        let mut app = App::init();

        app.selected_word = String::from("the");

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)
        );
        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)
        );
        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)
        );

        assert!(app.game_over_state.is_some_and(|state| {
            state == GameOverState::Win
        }), "The word was {} and the guesses were {:?}", app.selected_word, app.guessed_letters);
    }

    #[test]
    fn stops_key_handling_on_game_over() {
        let mut app = App::init();

        app.game_over_state = Some(GameOverState::Win);

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)
        );

        assert!(app.guessed_letters.is_empty());

        let mut app = App::init();

        app.game_over_state = Some(GameOverState::Lose);

        app.handle_keypress(
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)
        );

        assert!(app.guessed_letters.is_empty());
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
        let mut app = App::init_with_grace(0);

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
    fn runs_app() {
        let app = App::init();

        assert!(app.run);
    }
}
