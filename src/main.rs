#![allow(unused)]
mod blackjack;
use crate::blackjack::{Game, Deck, Card};
use std::time::Duration;
use std::thread;
use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    game: Game,
    exit: bool,
}

impl App {
    // runs application until quit
    pub fn run (&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('h') => self.hit(),
            KeyCode::Char('s') => self.stand(),
            KeyCode::Char('r') => if self.game.game_over() {
                self.game = Game::default();
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn hit(&mut self) {
        self.game.player_draw_card(0);
    }

    fn stand(&mut self) {
        self.game.player_stand()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Blackjack ".bold());
        let instructions: Line = if self.game.game_over() {
            Line::from(vec![
                " | Hit ".into(),
                " <H> ".blue().bold(),
                "| Stand ".into(),
                " <S> ".blue().bold(),
                "| Quit ".into(),
                "<Q> ".blue().bold(),
                "| ".into(),
                "Restart ".into(),
                "<R>".blue().bold(),
                " | ".into(),
            ])
        } else {
            Line::from(vec![
                " | Hit ".into(),
                " <H> ".blue().bold(),
                "| Stand ".into(),
                " <S> ".blue().bold(),
                "| Quit ".into(),
                "<Q> ".blue().bold(),
                "| ".into(),
            ])
        };


        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let dealer = self.game.dealer();
        let player = &self.game.players()[0];
        let card_strings: Vec<String> = player.hand().iter().map(|card| format!("{}{}", card.card_type(), card.suit())).collect();
        let cards_display = card_strings.join(", ");
        let hand_info = vec![Line::from(vec!["Hand: ".into(), cards_display.into()]),
                             Line::from(vec!["Value ".into(), self.game.players()[0].hand_value().to_string().yellow()])];
        let hand_text = Text::from(hand_info);
        let dealer_cards: Vec<String> = dealer.hand().iter().map(|card| format!("{}{}", card.card_type(), card.suit())).collect();
        let dealer_display = dealer_cards.join(", ");
        let dealer_info = vec![Line::from(vec!["Dealer: ".into(), dealer_display.into()]),
                               Line::from(vec!["Value ".into(), self.game.dealer().hand_value().to_string().yellow()])];
        let dealer_text = Text::from(dealer_info);

        let msg = if player.hand_value() > dealer.hand_value() && dealer.hand().len() >= 2 || player.has_blackjack(){
            format!("{} \n\n {} \n\n\n\n YOU WIN!!", hand_text, dealer_text)
        } else if self.game.players()[0].is_busted() || (dealer.hand().len() >= 2 && dealer.hand_value() >= player.hand_value() && !dealer.is_busted()) {
            format!("{} \n\n {} \n\n\n\n YOU LOSE!!!", hand_text, dealer_text)
        } else {
            format!("{} \n\n {}", hand_text, dealer_text)
        };
        Paragraph::new(msg)
            .centered()
            .block(block.clone())
            .render(area, buf);
    }
}
