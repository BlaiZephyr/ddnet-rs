pub mod ingame_menu;

use api::GRAPHICS;
use ui_generic::traits::UiPageInterface;

pub use api_ui::ui_impl::*;
pub use api_ui_game::render::*;

#[no_mangle]
fn mod_ui_new() -> Box<dyn UiPageInterface<()>> {
    Box::new(ingame_menu::page::IngameMenu::new(
        &GRAPHICS.with(|g| (*g).clone()),
    ))
}
