use crate::AppState;
use crate::DialogDisplay;
use crate::DisplayLanguage;
use crate::RESOLUTION_X;
use crate::assets::lexi::resume::{Choice, ResumeData};
use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;

mod actions;
mod inputs;
pub mod layouts;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveMenu::default());
        app.insert_resource(ActiveLink::default());
        app.add_event::<ChangeMenu>();
        app.add_event::<InitSelection>();

        app.add_systems(OnEnter(AppState::Resume), menu_setup)
            .add_systems(
                Update,
                move_choice_marker.run_if(in_state(AppState::Resume)),
            )
            .add_systems(
                Update,
                change_menu.run_if(on_event::<ChangeMenu>.and(in_state(AppState::Resume))),
            )
            .add_systems(
                Update,
                init_selection_markers
                    .run_if(on_event::<InitSelection>.and(in_state(AppState::Resume))),
            )
            .add_systems(OnExit(AppState::Resume), leave_menu);

        app.add_plugins(inputs::plugin);
    }
}

#[derive(Resource, Default, Clone)]
pub struct ActiveMenu {
    pub opt: Option<ResumeData>,
}

impl ActiveMenu {
    pub fn reset(&mut self) {
        *self = Self { ..default() };
    }
}

#[derive(Resource, Default, Clone)]
pub struct ActiveLink(Option<String>);

#[derive(Component)]
struct SelectionMarker(Choice);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
struct CurrentSelection(Option<Choice>);

impl Default for CurrentSelection {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Event)]
pub struct ChangeMenu(pub String);

impl ChangeMenu {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Event)]
struct InitSelection;

#[derive(Component)]
struct Language(String);

impl Language {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Component)]
struct GoToMenu(String);

impl GoToMenu {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

fn menu_setup(
    mut commands: Commands,
    mut bg: ResMut<ClearColor>,
    display_language: ResMut<DisplayLanguage>,
) {
    info!("Menu");
    info!(language = display_language.0);
    bg.0 = Color::srgb(0.2, 0.2, 0.2);
    commands.spawn((StateScoped(AppState::Resume), Camera2d::default()));

    commands.send_event(ChangeMenu::new("home"));
    return;
}

fn change_menu(
    mut changes: EventReader<ChangeMenu>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_language: ResMut<DisplayLanguage>,
    mut dialog_message: ResMut<ActiveMenu>,
    menu_data: Res<Assets<ResumeData>>,
    dialog_display_query: Query<(Entity, &DialogDisplay), With<DialogDisplay>>,
    hud: Res<Hud>,
) {
    info!("Spawning menu");
    let hud_entity = hud.0;

    let Some(event) = changes.read().next() else {
        return;
    };

    let menu_id = &event.0;

    dialog_message.opt = menu_data
        .into_inner()
        .iter()
        .filter(|(_, data)| data.id == menu_id.clone())
        .map(|(_, data)| data.clone())
        .next();

    let dialog = match &dialog_message.opt {
        Some(d) => d,
        None => {
            // for (entity, _) in dialog_display_query.iter() {
            //     commands.entity(entity).despawn();
            // }
            return;
        }
    };

    for (entity, dialog_display) in dialog_display_query.iter() {
        if dialog_display.0 != dialog.id {
            commands.entity(entity).despawn();
        } else {
            return;
        }
    }

    let padding_x = RESOLUTION_X / 32.0;
    commands.entity(hud_entity).with_children(|parent| {
        parent
            .spawn((
                StateScoped(AppState::Resume),
                DialogDisplay(dialog.id.clone()),
                layouts::menu_layout(RESOLUTION_X - 2. * padding_x),
            ))
            .with_children(|p| {
                let text = dialog.lex.from_language(&display_language.0);

                p.spawn(layouts::header_layout(&text));

                if let Some(images) = &dialog.image_reel {
                    if let Some(image) = images.first() {
                        p.spawn(layouts::image_reel_layout(asset_server.load(image)));
                    }
                }

                match &dialog.choices {
                    Some(choices) => {
                        for (_index, choice) in choices.iter().enumerate() {
                            let text = format!(
                                "  {}",
                                choice.choice.lex.from_language(&display_language.0)
                            );

                            let mut button =
                                p.spawn((layouts::button_layout(&text, choice.clone()),));
                            button.observe(inputs::mouse_move);
                            button.observe(inputs::mouse_over);

                            match &choice.choice.link {
                                Some(_) => {
                                    button.observe(inputs::click_link);
                                }
                                None => {}
                            }

                            match &choice.choice.action {
                                Some(action) => match action.as_str() {
                                    "start_game" => {
                                        button.observe(inputs::click_start_game);
                                    }
                                    "show_credits" => {
                                        button.observe(inputs::click_show_credits);
                                    }
                                    "english" | "spanish" => {
                                        button
                                            .insert(Language::new(action))
                                            .observe(inputs::click_language_selection);
                                    }
                                    _ => {}
                                },
                                None => {}
                            }

                            match &choice.choice.next_id {
                                Some(id) => {
                                    button
                                        .insert(GoToMenu::new(id))
                                        .observe(inputs::click_menu_selection);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            });
    });
    commands.send_event(InitSelection);
    return;
}

fn init_selection_markers(
    _: EventReader<InitSelection>,
    button: Query<&SelectionMarker>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    if let Some(next) = button.iter().next() {
        current_selection.0 = Some(next.0.clone());
    };
}

fn move_choice_marker(
    display_language: ResMut<DisplayLanguage>,
    dialog_message: Res<ActiveMenu>,
    current_selection: Res<CurrentSelection>,
    // mut button: Query<&mut BackgroundColor>,
    mut selections: Query<(&mut TextSpan, &ChildOf), With<layouts::MenuOption>>,
) {
    let Some(current_choice) = &current_selection.0 else {
        return;
    };

    let dialog = match &dialog_message.opt {
        Some(d) => d,
        None => {
            return;
        }
    };

    let choices = match &dialog.choices {
        Some(choices) => {
            if choices.is_empty() {
                return;
            }
            choices
        }
        None => return,
    };

    for (idx, choice) in choices.iter().enumerate() {
        let text = choice.choice.lex.from_language(&display_language.0);

        if current_choice.id == choice.id.clone() {
            for (text_idx, (mut text_span, _parent)) in selections.iter_mut().enumerate() {
                if idx == text_idx {
                    *text_span = TextSpan::new(format!("> {}", text.clone()));
                    // if let Ok(mut bg) = button.get_mut(parent.0) {
                    //     bg.0 = DARK_ORCHID.into();
                    // }
                }
            }
        } else {
            for (text_idx, (mut text_span, _parent)) in selections.iter_mut().enumerate() {
                if idx == text_idx {
                    *text_span = TextSpan::new(format!("  {}", text.clone()));
                    // if let Ok(mut bg) = button.get_mut(parent.0) {
                    //     bg.0 = ORCHID.into();
                    // }
                }
            }
        }
    }
}

fn leave_menu(mut dialog_message: ResMut<ActiveMenu>) {
    info!("left menu");
    dialog_message.reset();
}

#[cfg(target_arch = "wasm32")]
fn open_link(link: &str) {
    info!("Opening link");
    if let Some(window) = web_sys::window() {
        let _ = window.open_with_url(link);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn open_link(_: &str) {
    warn!("External links can only open in wasm builds");
}
