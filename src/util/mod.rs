use bevy::prelude::*;
pub mod handles;
use bevy::asset::load_internal_binary_asset;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::handles::*;
}

pub(super) fn plugin(app: &mut App) {
    load_internal_binary_asset!(
        app,
        handles::BODY_FONT,
        "../../assets/fonts/PressStart2P-vaV7.ttf",
        load_font_from_bytes
    );
    load_internal_binary_asset!(
        app,
        handles::SPLASH_FONT,
        "../../assets/fonts/PressStart2P-vaV7.ttf",
        load_font_from_bytes
    );
}

pub fn load_font_from_bytes(bytes: &[u8], _path: String) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}
