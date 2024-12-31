use ui_base::types::{UiRenderPipe, UiState};
use ui_generic::traits::UiPageInterface;

use super::{main_frame, user_data::UserData};

pub struct ChatUi {}

impl Default for ChatUi {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatUi {
    pub fn new() -> Self {
        Self {}
    }
}

impl UiPageInterface<UserData<'_>> for ChatUi {
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        pipe: &mut UiRenderPipe<UserData>,
        ui_state: &mut UiState,
    ) {
        main_frame::render(ui, pipe, ui_state)
    }
}
