use ratatui::layout::Constraint;

use crate::component::Component;

pub mod fps;
pub mod home;
pub mod home_old;
pub mod sized_component;

trait ConstrainedComponent: Component {
    fn constraint(&self) -> Constraint;
}
