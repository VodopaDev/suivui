pub mod base {
    use std::io::{Result, Stdout};
    use ratatui::prelude::CrosstermBackend;
    use ratatui::{Frame, Terminal};
    use ratatui::crossterm::event::Event;

    #[derive(PartialEq, Clone)]
    pub enum Action {
        GotoMainMenu,
        GotoTrackerInfo,
        GotoTrackerEdit,
        Terminate,
        DoNothing,
    }

    pub trait Model {
        fn reset(&mut self) -> ();
    }

    pub trait View<M: Model> {
        fn render(&self, frame: &mut Frame, model: &M) -> Result<()>;
    }

    pub trait Controller<M: Model> {
        fn handle_event(&self, event: &Event, model: &mut M) -> Result<Action>;
    }

    pub trait ModelViewController {
        fn run(&mut self, event: &Event, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Action;
    }

    pub struct MVC<M: Model> {
        pub model: M,
        pub view: Box<dyn View<M>>,
        pub controller: Box<dyn Controller<M>>,
    }
    impl<M: Model> ModelViewController for MVC<M>{
        fn run(&mut self, event: &Event, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Action {
            if terminal.draw(|f| self.view.render(f, &self.model).unwrap()).is_err() {return Action::Terminate;}
            return self.controller.handle_event(event, &mut self.model).unwrap_or(Action::Terminate);
        }
    }
}

pub mod main_menu;
pub mod tracker_edit;
pub mod tracker_info;