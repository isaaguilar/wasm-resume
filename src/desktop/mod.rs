use crate::AppState;
use crate::DialogDisplay;
use crate::DisplayLanguage;
use crate::RESOLUTION_X;
use crate::assets::icons::IconAssets;
use crate::assets::lexi::desktop::Icon;
use crate::assets::lexi::desktop::Link;
use crate::util::handles::BODY_FONT;
// use crate::assets::lexi::resume::{Choice, ResumeData};
use crate::RESOLUTION_Y;
use crate::assets::lexi::desktop::DesktopData;
use bevy::color::palettes::css::BLACK;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy_aspect_ratio_mask::Hud;

mod actions;
mod inputs;
mod layouts;

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
                change_desktop.run_if(on_event::<ChangeMenu>.and(in_state(AppState::Resume))),
            )
            .add_systems(OnExit(AppState::Resume), leave_menu);

        app.add_plugins(inputs::plugin);
    }
}

#[derive(Resource, Default, Clone)]
pub struct ActiveMenu {
    pub opt: Option<DesktopData>,
}

impl ActiveMenu {
    pub fn reset(&mut self) {
        *self = Self { ..default() };
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ActiveLink(Option<String>);

#[derive(Component)]
struct SelectionMarker(Option<Link>);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
struct CurrentSelection(Option<Icon>);

impl Default for CurrentSelection {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Event)]
struct ChangeMenu(String);

impl ChangeMenu {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Event)]
struct InitSelection;

#[derive(Component)]
struct GoToMenu(String);

impl GoToMenu {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

fn menu_setup(mut commands: Commands, mut bg: ResMut<ClearColor>) {
    info!("Menu");

    bg.0 = Color::srgb(0.2, 0.2, 0.2);
    commands.spawn((StateScoped(AppState::Resume), Camera2d::default()));

    commands.send_event(ChangeMenu::new("home"));
    return;
}

fn change_desktop(
    mut changes: EventReader<ChangeMenu>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<IconAssets>,
    display_language: ResMut<DisplayLanguage>,
    mut dialog_message: ResMut<ActiveMenu>,
    desktop_data: Res<Assets<DesktopData>>,
    dialog_display_query: Query<(Entity, &DialogDisplay), With<DialogDisplay>>,
    hud: Res<Hud>,
) {
    let hud_entity = hud.0;

    let Some(event) = changes.read().next() else {
        return;
    };

    let menu_id = &event.0;

    dialog_message.opt = desktop_data
        .into_inner()
        .iter()
        .filter(|(_, data)| data.id == menu_id.clone())
        .map(|(_, data)| data.clone())
        .next();

    let dialog = match &dialog_message.opt {
        Some(d) => d,
        None => {
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
            .with_children(|parent| {
                let text = dialog.lex.from_language(&display_language.0);

                if let Some(show_window) = &dialog.window {
                    if *show_window {
                        let window_image = if dialog.window_image.is_some() {
                            assets.window2.clone()
                        } else {
                            assets.window.clone()
                        };

                        let mut window = parent.spawn((
                            Name::new(format!("Button {}", text)),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(112. - padding_x),
                                top: Val::Px(70.),
                                width: Val::Px(800.),
                                height: Val::Px(500.),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            Pickable::default(),
                            ImageNode {
                                image: window_image,

                                ..default()
                            },
                        ));

                        if let Some(image) = &dialog.image {
                            window.with_children(|parent| {
                                parent.spawn((
                                    Name::new(format!("Note")),
                                    Node {
                                        position_type: PositionType::Relative,
                                        left: Val::Px(60.),
                                        top: Val::Px(80.),
                                        width: Val::Px(400.),
                                        height: Val::Px(300.),
                                        ..default()
                                    },
                                    children![(
                                        Pickable::IGNORE,
                                        ImageNode {
                                            image: asset_server.load(image),
                                            ..default()
                                        }
                                    )],
                                ));
                            });
                        }

                        match &dialog.note {
                            Some(note) => {
                                let font_color = match &note.lex.style {
                                    Some(color_string) => match color_string.as_str() {
                                        "black" => TextColor(BLACK.into()),
                                        "white" => TextColor(WHITE.into()),
                                        _ => TextColor(WHITE.into()),
                                    },
                                    None => TextColor(WHITE.into()),
                                };

                                let text =
                                    format!("{}", note.lex.from_language(&display_language.0));

                                window.with_children(|parent| {
                                    parent.spawn((
                                        // BackgroundColor(DARK_ORCHID.into()),
                                        Name::new(format!("Note")),
                                        Node {
                                            position_type: PositionType::Relative,
                                            left: Val::Px(note.position[0]),
                                            top: Val::Px(note.position[1]),
                                            width: Val::Px(700.),
                                            height: Val::Px(500.),
                                            ..default()
                                        },
                                        Text::default(),
                                        TextLayout::default().with_justify(JustifyText::Left),
                                        children![(
                                            font_color,
                                            TextFont::from_font(BODY_FONT)
                                                .with_font_size(RESOLUTION_Y * 6. / 8. / 36.)
                                                .with_line_height(
                                                    bevy::text::LineHeight::RelativeToFont(1.5)
                                                ),
                                            Pickable::IGNORE,
                                            TextSpan::new(format!("{}", text)),
                                        )],
                                    ));
                                });
                            }
                            None => {}
                        }

                        let x_placement_adjustment = if dialog.window_image.is_some() {
                            10.
                        } else {
                            0.
                        };

                        let mut button = parent.spawn((
                            Name::new(format!("Close")),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(112. - padding_x + 800. - 34.),
                                top: Val::Px(74. + x_placement_adjustment),
                                width: Val::Px(24.),
                                height: Val::Px(24.),

                                ..default()
                            },
                            Pickable::default(),
                            ImageNode {
                                image: assets.close.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: assets.close_layout.clone(),
                                    index: 0,
                                }),

                                ..default()
                            },
                        ));

                        button.observe(inputs::mouse_over_generic);
                        button.observe(inputs::mouse_out_generic);

                        match &dialog.next_id {
                            Some(id) => {
                                button
                                    .insert(GoToMenu::new(id))
                                    .observe(inputs::click_menu_selection);
                            }
                            None => {}
                        }

                        parent.spawn((
                            Name::new(format!("Close")),
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(140.),
                                top: Val::Px(76. + x_placement_adjustment),
                                width: Val::Px(800.),
                                height: Val::Px(24.),

                                ..default()
                            },
                            Pickable::IGNORE,
                            TextFont::from_font(BODY_FONT)
                                .with_font_size(RESOLUTION_Y * 6. / 8. / 36.)
                                .with_line_height(bevy::text::LineHeight::RelativeToFont(1.5)),
                            Text::new(text),
                        ));
                    }
                } else {
                    parent.spawn(layouts::header_layout(&text));
                }

                match &dialog.icons {
                    Some(icons) => {
                        for (_index, icon) in icons.iter().enumerate() {
                            let font_color = match &icon.lex.style {
                                Some(color_string) => match color_string.as_str() {
                                    "black" => TextColor(BLACK.into()),
                                    "white" => TextColor(WHITE.into()),
                                    _ => TextColor(WHITE.into()),
                                },
                                None => TextColor(WHITE.into()),
                            };

                            let text = format!("  {}", icon.lex.from_language(&display_language.0));

                            let (image, layout) =
                                (assets.notepad.clone(), assets.notepad_layout.clone());

                            let mut button = parent.spawn((
                                // BackgroundColor(DARK_ORCHID.into()),
                                layouts::MenuOption,
                                Name::new(format!("Button {}", text)),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(icon.position[0]),
                                    top: Val::Px(icon.position[1]),
                                    width: Val::Px(icon.icon.size[0]),
                                    height: Val::Px(icon.icon.size[1]),

                                    ..default()
                                },
                                Pickable::default(),
                                SelectionMarker(None),
                                ImageNode {
                                    image: image,
                                    texture_atlas: Some(TextureAtlas {
                                        layout: layout,
                                        index: icon.icon.index,
                                    }),
                                    ..default()
                                },
                                children![(
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(-1.7 * icon.icon.size[0]),
                                        top: Val::Px(icon.icon.size[1]),
                                        width: Val::Px(icon.icon.size[0] * 4.),
                                        height: Val::Px(icon.icon.size[1]),
                                        ..default()
                                    },
                                    Text::default(),
                                    TextLayout::default().with_justify(JustifyText::Center),
                                    children![(
                                        font_color,
                                        TextFont::from_font(BODY_FONT)
                                            .with_font_size(RESOLUTION_Y * 6. / 8. / 36.)
                                            .with_line_height(
                                                bevy::text::LineHeight::RelativeToFont(1.5)
                                            ),
                                        Pickable::IGNORE,
                                        TextSpan::new(format!("{}", text)),
                                    )]
                                ),],
                            ));

                            button.observe(inputs::mouse_over);
                            button.observe(inputs::mouse_out);

                            match &icon.link {
                                Some(_) => {
                                    button.observe(inputs::click_link);
                                }
                                None => {}
                            }

                            match &icon.next_id {
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

                match &dialog.links {
                    Some(links) => {
                        for (_index, link) in links.iter().enumerate() {
                            let font_color = match &link.lex.style {
                                Some(color_string) => match color_string.as_str() {
                                    "black" => TextColor(BLACK.into()),
                                    "white" => TextColor(WHITE.into()),
                                    _ => TextColor(WHITE.into()),
                                },
                                None => TextColor(WHITE.into()),
                            };

                            let text = format!("{}", link.lex.from_language(&display_language.0));

                            let (image, layout) =
                                (assets.links.clone(), assets.links_layout.clone());

                            let mut button = parent.spawn((
                                // BackgroundColor(DARK_ORCHID.into()),
                                layouts::MenuOption,
                                Name::new(format!("Button {}", text)),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(link.position[0]),
                                    top: Val::Px(link.position[1]),
                                    width: Val::Px(link.icon.size[0]),
                                    height: Val::Px(link.icon.size[1]),

                                    ..default()
                                },
                                Pickable::default(),
                                SelectionMarker(Some(link.clone())),
                                ImageNode {
                                    image: image,
                                    texture_atlas: Some(TextureAtlas {
                                        layout: layout,
                                        index: link.icon.index,
                                    }),
                                    ..default()
                                },
                                children![(
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(-1.7 * link.icon.size[0]),
                                        top: Val::Px(link.icon.size[1]),
                                        width: Val::Px(700.),
                                        height: Val::Px(120.),
                                        ..default()
                                    },
                                    Text::default(),
                                    TextLayout::default().with_justify(JustifyText::Left),
                                    children![(
                                        font_color,
                                        TextFont::from_font(BODY_FONT)
                                            .with_font_size(RESOLUTION_Y * 6. / 8. / 36.)
                                            .with_line_height(
                                                bevy::text::LineHeight::RelativeToFont(1.5)
                                            ),
                                        Pickable::IGNORE,
                                        TextSpan::new(format!("{}", text)),
                                    )]
                                ),],
                            ));

                            button.observe(inputs::mouse_over);
                            button.observe(inputs::mouse_out);

                            match &link.link {
                                Some(_) => {
                                    button.observe(inputs::click_link);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            });
    });

    return;
}

fn leave_menu(mut dialog_message: ResMut<ActiveMenu>) {
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
