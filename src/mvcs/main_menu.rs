use std::io::Result;
use ratatui::{
    self,
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::*
};
use super::base::{Action, Model, View, Controller};


const MAX_LINES: usize = 10;

pub struct MainMenuModel{
    // Main View
    pub messages: Vec<String>,
    pub counter: usize,
}
impl Model for MainMenuModel {
    fn reset(&mut self) -> () {
        self.messages.clear();
        self.counter = 0;
    }
}
impl Default for MainMenuModel{
    fn default() -> Self {
        return Self{
            messages: Vec::with_capacity(16),
            counter: 0,
        };
    }
}


pub struct MainMenuView{}
impl View<MainMenuModel> for MainMenuView{
    fn render(&self, frame: &mut Frame, model: &MainMenuModel) -> Result<()> {
        let last_elems = if model.messages.len() <= MAX_LINES {
            model.messages.to_vec()
        } else {
            model.messages[model.messages.len()-MAX_LINES..].to_vec()
        };
        let text: Vec<Line<'_>> = last_elems.iter().map(|s| Span::styled(s, Style::default().fg(Color::LightGreen)).into()).collect();
        let block: Block = Block::bordered().title(format!("TimeTracker (messages={}). Press Esc to exit.", model.messages.len()));
        let widget = Paragraph::new(text).block(block);
        frame.render_widget(widget, frame.size(),);
        Ok(())
    }
}


pub struct MainMenuController{}
impl Controller<MainMenuModel> for MainMenuController{
    fn handle_event(&self, event: &Event, model: &mut MainMenuModel) -> Result<Action> {
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(Action::Terminate),
                    KeyCode::Delete => return Ok(Action::Terminate),
                    KeyCode::Enter => return Ok(Action::GotoTrackerInfo),
                    KeyCode::Char('n') => {
                        model.messages.push(format!("Message {}", model.counter));
                        model.counter += 1;
                        return Ok(Action::GotoTrackerEdit)
                    },
                    _ => {},
                }
            }
        }
        return Ok(Action::DoNothing)
    }
}