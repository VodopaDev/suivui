mod mvcs;
use mvcs::base::{Action, ModelViewController, MVC};
use mvcs::main_menu::*;
use mvcs::tracker_edit::*;
use mvcs::tracker_info::*;

use std::io::{self, stdout};
use std::process::exit;
use ratatui::{
    self,
    crossterm::{
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*
};



fn main<>() -> io::Result<()> {
    let mut main_menu_mvc = MVC::<MainMenuModel>{
        model: MainMenuModel::default(),
        view: Box::new(MainMenuView{}),
        controller: Box::new(MainMenuController{}),
    };

    let mut tracker_edit_mvc = MVC::<TrackerEditModel>{
        model: TrackerEditModel::default(),
        view: Box::new(TrackerEditView{}),
        controller: Box::new(TrackerEditController{}),
    };

    let mut tracker_info_mvc = MVC::<TrackerInfoModel>{
        model: TrackerInfoModel::default(),
        view: Box::new(TrackerInfoView{}),
        controller: Box::new(TrackerInfoController{}),
    };
    
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut current_mvc: &mut dyn ModelViewController = &mut main_menu_mvc;
    let mut next_action: Action = Action::GotoMainMenu;

    while next_action != Action::Terminate {
        match next_action {
            Action::GotoMainMenu => current_mvc = &mut main_menu_mvc,
            Action::GotoTrackerInfo => current_mvc = &mut tracker_info_mvc,
            Action::GotoTrackerEdit => current_mvc = &mut tracker_edit_mvc,
            Action::Terminate => exit(0),
            Action::DoNothing => {},
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            let e = event::read()?;
            next_action = current_mvc.run(&e, &mut terminal);
        } else {
            next_action = Action::DoNothing;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}