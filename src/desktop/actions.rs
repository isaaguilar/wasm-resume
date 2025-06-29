use super::ChangeMenu;
use bevy::prelude::*;

pub fn menu_selection(mut change_menu: EventWriter<ChangeMenu>, menu: impl Into<String>) {
    change_menu.write(ChangeMenu::new(menu.into()));
}
