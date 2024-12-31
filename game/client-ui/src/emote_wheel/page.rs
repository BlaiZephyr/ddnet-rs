use ui_base::types::{UiRenderPipe, UiState};
use ui_generic::traits::UiPageInterface;

use super::{main_frame, user_data::UserData};

pub struct EmoteWheelUi {}

impl Default for EmoteWheelUi {
    fn default() -> Self {
        Self::new()
    }
}

impl EmoteWheelUi {
    pub fn new() -> Self {
        Self {}
    }
}

impl UiPageInterface<UserData<'_>> for EmoteWheelUi {
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        pipe: &mut UiRenderPipe<UserData>,
        ui_state: &mut UiState,
    ) {
        main_frame::render(ui, pipe, ui_state)
    }
}
