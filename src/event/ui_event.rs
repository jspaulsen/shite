use crate::ui::UiHandle;

#[derive(Clone, Debug, PartialEq)]
pub enum UiEvent {
    Click(UiHandle),
    ClickRelease(UiHandle),
    None,
}
