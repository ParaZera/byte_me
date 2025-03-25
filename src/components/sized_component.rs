use crate::component::Component;

pub(crate) trait SizedComponent: Component {
    fn size(&self) -> (u16, u16);
}
