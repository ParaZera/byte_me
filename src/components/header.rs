use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, component::Component, config::Config};

#[derive(Default)]
pub struct HeaderState {}

#[derive(Default)]
pub struct Header {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,

    state: HeaderState,
    components: Vec<Box<dyn Component>>,
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for HeaderState {
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
            Action::Tick => {
                tracing::info!("Home: Tick");
                // trace_dbg!("Home: Tick");
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Block::default().borders(Borders::ALL).title("Home"), area);
        // Paragraph::new("hello world"), area);
        Ok(())
    }
}
