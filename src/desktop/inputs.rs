use super::{ChangeMenu, CurrentSelection, GoToMenu, Language, SelectionMarker, actions};
use crate::AppState;
use crate::DisplayLanguage;
use crate::desktop::ActiveLink;
use crate::desktop::open_link;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentSelection::default());
}

// -- Mouse Inputs via Observers --
pub fn mouse_over(
    trigger: Trigger<Pointer<Over>>,
    mut button: Query<(&SelectionMarker, &mut ImageNode)>,
    mut current_selection: ResMut<CurrentSelection>,
    mut active_link: ResMut<ActiveLink>,
) {
    if let Ok((selection, mut sprite)) = button.get_mut(trigger.target) {
        // current_selection.0 = Some(selection.0.clone());
        if let Some(link) = &selection.0 {
            active_link.0 = link.link.clone();
        }
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = atlas.index + 1;
        }
    };
}

pub fn mouse_out_generic(trigger: Trigger<Pointer<Out>>, mut button: Query<&mut ImageNode>) {
    if let Ok(mut sprite) = button.get_mut(trigger.target) {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = atlas.index - 1;
        }
    };
}

pub fn mouse_over_generic(trigger: Trigger<Pointer<Over>>, mut button: Query<&mut ImageNode>) {
    if let Ok(mut sprite) = button.get_mut(trigger.target) {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = atlas.index + 1;
        }
    };
}

pub fn mouse_out(
    trigger: Trigger<Pointer<Out>>,
    mut button: Query<(&SelectionMarker, &mut ImageNode)>,
    mut current_selection: ResMut<CurrentSelection>,
    mut active_link: ResMut<ActiveLink>,
) {
    if let Ok((selection, mut sprite)) = button.get_mut(trigger.target) {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = atlas.index - 1;
        }
    };
}

pub fn mouse_move(
    trigger: Trigger<Pointer<Move>>,
    mut button: Query<(&SelectionMarker, &mut ImageNode)>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    if let Ok((selection, mut sprite)) = button.get_mut(trigger.target) {
        // current_selection.0 = Some(selection.0.clone());
        // if let Some(atlas) = sprite.texture_atlas.as_mut() {
        //     atlas.index = 1;
        // }
    };
}

// Mouse click observers
pub fn click_link(_: Trigger<Pointer<Click>>, active_link: Res<ActiveLink>) {
    info!(?active_link);
    if let Some(link) = &active_link.0 {
        open_link(link);
    }
}

pub fn click_start_game(_: Trigger<Pointer<Click>>, next_app_state: ResMut<NextState<AppState>>) {
    actions::start_game(next_app_state);
}

pub fn click_show_credits(_: Trigger<Pointer<Click>>, next_app_state: ResMut<NextState<AppState>>) {
    actions::show_credits(next_app_state);
}

pub fn click_language_selection(
    trigger: Trigger<Pointer<Click>>,
    display_language: ResMut<DisplayLanguage>,
    language_query: Query<&Language>,
) {
    if let Ok(language) = language_query.get(trigger.target) {
        actions::language_selection(display_language, language.0.clone());
    }
}
pub fn click_menu_selection(
    trigger: Trigger<Pointer<Click>>,
    change_menu: EventWriter<ChangeMenu>,
    go_to_menu_query: Query<&GoToMenu>,
) {
    if let Ok(go_to_menu) = go_to_menu_query.get(trigger.target) {
        info!("Going to {}", go_to_menu.0);
        actions::menu_selection(change_menu, go_to_menu.0.clone());
    }
}
