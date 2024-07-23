use std::io::Result;
use ratatui::{
    self,
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::*
};
use super::base::{Action, Model, View, Controller};

pub struct TrackerEditModel{
    cursor_position: usize,
    name: String
}
impl Model for TrackerEditModel {
    fn reset(&mut self) {
        self.name.clear();
        self.cursor_position = 0;
    }
}
impl Default for TrackerEditModel{
    fn default() -> Self {
        return Self{cursor_position: 0, name: String::with_capacity(16)};
    }
}


pub struct TrackerEditView{}
impl View<TrackerEditModel> for TrackerEditView{
    fn render(&self, frame: &mut Frame, model: &TrackerEditModel) -> Result<()> {
        let widget = Paragraph::new(model.name.clone())
            .block(Block::bordered().title("Press Enter to save, Esc to exit"));
        frame.render_widget(widget, frame.size(),);
        Ok(())
    }
}


pub struct TrackerEditController{}
impl Controller<TrackerEditModel> for TrackerEditController{
    fn handle_event(&self, event: &Event, model: &mut TrackerEditModel) -> Result<Action> {
        //if event::poll(std::time::Duration::from_millis(50))? {
        //    if let Event::Key(key) = event::read()? {
        if let Event::Key(key) = event  {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    // Exit
                    KeyCode::Enter => {
                        // TODO: save line
                        model.reset();
                        return Ok(Action::GotoMainMenu)
                    },
                    KeyCode::Esc => {
                        model.reset();
                        return Ok(Action::GotoMainMenu)
                    },

                    // Cursor move
                    KeyCode::Left => model.cursor_position = if model.cursor_position > 0 { model.cursor_position - 1 } else { 0 },
                    KeyCode::Right => model.cursor_position = if model.cursor_position < model.name.len() { model.cursor_position + 1 } else { 0 },
                    KeyCode::Home => model.cursor_position = 0,
                    KeyCode::End => model.cursor_position = model.name.len(),

                    // String manipulation
                    KeyCode::Backspace => {
                        if model.cursor_position > 0 {
                            let mut new_name = String::with_capacity(model.name.len()-1);
                            new_name.push_str(&model.name[0..model.cursor_position-1]);
                            new_name.push_str(&model.name[model.cursor_position..]);
                            model.cursor_position -= 1;
                            model.name = new_name;
                        }
                    },
                    KeyCode::Delete => {
                        if model.cursor_position < model.name.len() {
                            let mut new_name = String::with_capacity(model.name.len()-1);
                            new_name.push_str(&model.name[0..model.cursor_position]);
                            new_name.push_str(&model.name[model.cursor_position+1..]);
                            model.name = new_name;
                        }
                    },
                    KeyCode::Char(c) => {
                        let mut new_name = String::with_capacity(model.name.len()+1);
                        new_name.push_str(&model.name[0..model.cursor_position]);
                        new_name.push(c);
                        new_name.push_str(&model.name[model.cursor_position..]);
                        model.name = new_name;
                        model.cursor_position += 1;
                    },

                    // Default (do nothing)
                    _ => {},
                }
            }
        }
        return Ok(Action::DoNothing)
    }
}