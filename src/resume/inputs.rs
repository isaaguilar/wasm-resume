use super::{ChangeMenu, CurrentSelection, GoToMenu, Language, SelectionMarker, actions};
use crate::AppState;
use crate::DisplayLanguage;
use crate::resume::ActiveLink;
use crate::resume::open_link;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentSelection::default());

    // app.add_systems(
    //     Update,
    //     (keyboard_menu_selection_system,).run_if(in_state(AppState::Menu)),
    // );

    // app.add_systems(
    //     Update,
    //     keyboard_selection.run_if(input_just_pressed(KeyCode::Enter).and(in_state(AppState::Menu))),
    // );
}

// fn keyboard_menu_selection_system(
//     time: Res<Time>,
//     mut interaction_rate_limit: ResMut<InteractionRateLimit>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     gamepads: Query<&Gamepad>,
//     dialog_message: ResMut<ActiveMenu>,
//     mut current_selection: ResMut<CurrentSelection>,
// ) {
//     let (_right, _left, _, up, down, _pause) = match gamepads.iter().next() {
//         Some(gamepad) => {
//             let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
//             let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();

//             (
//                 left_stick_x > 0.075,  //right
//                 left_stick_x < -0.075, //left
//                 gamepad.any_just_pressed([
//                     GamepadButton::North,
//                     GamepadButton::South,
//                     GamepadButton::East,
//                     GamepadButton::West,
//                 ]),
//                 left_stick_y > 0.075, //up
//                 left_stick_y < -0.75, //down
//                 gamepad.just_pressed(GamepadButton::Start),
//             )
//         }
//         None => (false, false, false, false, false, false),
//     };
//     let up_key_pressed = up || keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
//     let down_key_pressed = down || keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

//     let dialog = match &dialog_message.opt {
//         Some(d) => d,
//         None => {
//             return;
//         }
//     };

//     let choices = match &dialog.choices {
//         Some(choices) => {
//             if choices.is_empty() {
//                 warn!("No choices available");
//                 return;
//             }
//             choices
//         }
//         None => {
//             warn!("No choice options");
//             return;
//         }
//     };

//     interaction_rate_limit.0.tick(time.delta());
//     if interaction_rate_limit.0.finished() || interaction_rate_limit.0.just_finished() {
//         if up_key_pressed || down_key_pressed {
//             interaction_rate_limit.0.reset();
//         }

//         let index = if let Some(choice) = &current_selection.0 {
//             match choices.iter().enumerate().find(|(_, c)| c.id == choice.id) {
//                 Some((index, _)) => index,
//                 None => 0,
//             }
//         } else {
//             0
//         };

//         let new_index = if up_key_pressed {
//             if index <= 0 {
//                 0
//             } else {
//                 index - 1
//             }
//         } else if down_key_pressed {
//             if index >= choices.len() - 1 {
//                 choices.len() - 1
//             } else {
//                 index + 1
//             }
//         } else {
//             index
//         };

//         for (choice_index, choice) in choices.iter().enumerate() {
//             if new_index == choice_index {
//                 current_selection.0 = Some(choice.clone());
//             }
//         }
//     }
// }

// fn keyboard_selection(
//     change_menu: EventWriter<ChangeMenu>,
//     current_selection: ResMut<CurrentSelection>,
//     next_app_state: ResMut<NextState<AppState>>,
//     display_language: ResMut<DisplayLanguage>,
//     // assets: Res<CustomAssets>,
// ) {
//     // let svg = &assets.svgfile;
//     // info!(?svg);
//     info!(?current_selection);
//     let Some(choice) = &current_selection.0 else {
//         return;
//     };

//     match &choice.choice.action {
//         Some(action) => match action.as_str() {
//             "start_game" => {
//                 actions::start_game(next_app_state);
//             }
//             "show_credits" => {
//                 actions::show_credits(next_app_state);
//             }
//             "english" | "spanish" => {
//                 actions::language_selection(display_language, action);
//             }
//             _ => {}
//         },
//         None => {}
//     }

//     match &choice.choice.next_id {
//         Some(id) => {
//             actions::menu_selection(change_menu, id);
//         }
//         None => {}
//     }
// }

// -- Mouse Inputs via Observers --
pub fn mouse_over(
    trigger: Trigger<Pointer<Over>>,
    mut button: Query<&SelectionMarker>,
    mut current_selection: ResMut<CurrentSelection>,
    mut active_link: ResMut<ActiveLink>,
) {
    if let Ok(selection) = button.get_mut(trigger.target) {
        current_selection.0 = Some(selection.0.clone());
        active_link.0 = selection.0.choice.link.clone();
    };
}

pub fn mouse_move(
    trigger: Trigger<Pointer<Move>>,
    mut button: Query<&SelectionMarker>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    if let Ok(selection) = button.get_mut(trigger.target) {
        current_selection.0 = Some(selection.0.clone());
    };
}

// Mouse click observers
pub fn click_link(_: Trigger<Pointer<Click>>, active_link: Res<ActiveLink>) {
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
        actions::menu_selection(change_menu, go_to_menu.0.clone());
    }
}
