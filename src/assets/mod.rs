use crate::{AFTER_LOADING_STATE, AppState};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};

pub mod icons;
pub mod lexi;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(lexi::plugin);
    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AFTER_LOADING_STATE)
            .load_collection::<icons::IconAssets>(),
    );
}
