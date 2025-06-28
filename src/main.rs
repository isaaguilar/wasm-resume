use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};

mod assets;
mod desktop;
#[cfg(feature = "dev")]
mod dev_tools;
mod resume;
mod util;

const RESOLUTION_X: f32 = 1024.0;
const RESOLUTION_Y: f32 = 640.0;
const AFTER_LOADING_STATE: AppState = AppState::Resume;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
#[states(scoped_entities)]
pub enum AppState {
    #[default]
    Preload,
    Loading,
    Splash,
    Resume,
    Menu,
    Reset,
    LoadLevel,
    TransitionOut,
    ReadyCheck,
}

#[derive(Resource, Deref, DerefMut)]
pub struct DisplayLanguage(pub String);

impl DisplayLanguage {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Component)]
pub struct DialogDisplay(String);

fn main() {
    App::new()
        .insert_resource(DisplayLanguage::new("english"))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Aspect Ratio Mask".into(),
                        name: Some("Aspect Ratio Mask".into()),
                        resolution: (RESOLUTION_X, RESOLUTION_Y).into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_plugins(AspectRatioPlugin {
            resolution: Resolution {
                width: RESOLUTION_X,
                height: RESOLUTION_Y,
            },
            mask: AspectRatioMask {
                color: Color::srgb(0.02, 0.02, 0.02),
            },
        })
        .add_plugins((assets::plugin, desktop::Menu, resume::Menu, util::plugin))
        .run();
}
