use color_eyre::Result;
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::component::Component;
use crate::{action::Action, config::Config};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
enum ActiveHeaderElement {
    Kind,
    Endianness,
    Encoding,
    #[default]
    Input,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,

    active_header_element: ActiveHeaderElement,
    input: String,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => todo!(),
            Action::Render => todo!(),
            Action::Resize(_, _) => todo!(),
            Action::Suspend => todo!(),
            Action::Resume => todo!(),
            Action::Quit => todo!(),
            Action::ClearScreen => todo!(),
            Action::Error(_) => todo!(),
            Action::Help => todo!(),
            Action::ScrollDown => todo!(),
            Action::ScrollUp => todo!(),
            Action::SelectRight => todo!(),
            Action::SelectLeft => todo!(),
            Action::CharacterInput(character_input) => todo!(),
            _ => todo!(),
        }

        // match action {
        //     Action::Tick => {
        //         tracing::info!("Home: Tick");
        //         // trace_dbg!("Home: Tick");
        //         // add any logic here that should run on every tick
        //     }
        //     Action::Render => {
        //         // add any logic here that should run on every render
        //     }
        //     _ => {}
        // }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // frame.render_widget(Block::default().borders(Borders::ALL).title("Home"), area);
        // Paragraph::new("hello world"), area);
        Ok(())
    }
}
