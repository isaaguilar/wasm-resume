use super::{ChangeMenu, CurrentSelection, GoToMenu, SelectionMarker, actions};
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

    mut active_link: ResMut<ActiveLink>,
) {
    if let Ok((_, mut sprite)) = button.get_mut(trigger.target) {
        active_link.0 = None;
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = atlas.index - 1;
        }
    };
}

// Mouse click observers
pub fn click_link(_: Trigger<Pointer<Click>>, active_link: Res<ActiveLink>) {
    if let Some(link) = &active_link.0 {
        open_link(link);
    }
}

pub fn click_menu_selection(
    trigger: Trigger<Pointer<Click>>,
    change_menu: EventWriter<ChangeMenu>,
    go_to_menu_query: Query<&GoToMenu>,
) {
    if let Ok(go_to_menu) = go_to_menu_query.get(trigger.target) {
        actions::menu_selection(change_menu, go_to_menu.0.clone());
    }
}
