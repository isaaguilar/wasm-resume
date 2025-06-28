use super::ChangeMenu;
use crate::AppState;
use crate::DisplayLanguage;
use bevy::prelude::*;

pub fn start_game(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::Resume);
}

pub fn show_credits(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::Loading);
}

pub fn language_selection(
    mut display_language: ResMut<DisplayLanguage>,
    language: impl Into<String>,
) {
    display_language.0 = language.into();
}

pub fn menu_selection(mut change_menu: EventWriter<ChangeMenu>, menu: impl Into<String>) {
    change_menu.write(ChangeMenu::new(menu.into()));
}
