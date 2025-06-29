use crate::{
    AppState,
    assets::lexi::{LexiCollection, Lexicon},
};
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<DesktopData>::new(&[".json"]));
    app.add_systems(OnEnter(AppState::Preload), preload);
}

fn preload(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading Resume");
    commands.insert_resource(LexiCollection::<DesktopData>::new(
        &asset_server,
        vec![
            "lexi/desktop/home.json",
            "lexi/desktop/work.json",
            "lexi/desktop/work/illumina.json",
            "lexi/desktop/work/tillster.json",
            "lexi/desktop/work/audit.json",
            "lexi/desktop/homedev.json",
            "lexi/desktop/homedev/picoparty.json",
            "lexi/desktop/homedev/pumpkinsound.json",
            "lexi/desktop/links.json",
        ],
    ));
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct DesktopData {
    pub id: String,
    pub lex: Lexicon,
    pub image: Option<String>,
    pub icons: Option<Vec<Icon>>,
    pub links: Option<Vec<Link>>,
    pub window: Option<bool>,
    pub window_image: Option<String>,
    pub next_id: Option<String>,
    pub note: Option<Note>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Note {
    pub lex: Lexicon,
    pub position: [f32; 2],
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Icon {
    pub icon: IconData,
    pub lex: Lexicon,
    pub next_id: Option<String>,
    pub position: [f32; 2],
    pub link: Option<String>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Link {
    pub icon: IconData,
    pub lex: Lexicon,
    pub position: [f32; 2],
    pub link: Option<String>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct IconData {
    pub size: [f32; 2],
    pub index: usize,
}
