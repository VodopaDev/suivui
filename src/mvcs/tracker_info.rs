use std::io::Result;
use ratatui::{
    self,
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::*
};
use super::base::{Action, Model, View, Controller};

pub struct TrackerInfoModel{
    name: String
}
impl Model for TrackerInfoModel {
    fn reset(&mut self) -> () {
        self.name.clear();
    }
}
impl Default for TrackerInfoModel{
    fn default() -> Self {
        return Self{name: "Test".to_string()};
    }
}


pub struct TrackerInfoView{}
impl View<TrackerInfoModel> for TrackerInfoView{
    fn render(&self, frame: &mut Frame, model: &TrackerInfoModel) -> Result<()> {
        let widget = Paragraph::new(format!("Name: {}", model.name))
            .block(Block::bordered().title("Press Esc to exit"));
        frame.render_widget(widget, frame.size(),);
        Ok(())
    }
}


pub struct TrackerInfoController{}
impl Controller<TrackerInfoModel> for TrackerInfoController{
    fn handle_event(&self, event: &Event, model: &mut TrackerInfoModel) -> Result<Action> {
        //if event::poll(std::time::Duration::from_millis(50))? {
        //    if let Event::Key(key) = event::read()? {
        if let Event::Key(key) = event  {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    // Return to main menue
                    KeyCode::Esc => return Ok(Action::GotoMainMenu),
                    // Default (do nothing)
                    _ => {},
                }
            }
        }
        return Ok(Action::DoNothing)
    }
}