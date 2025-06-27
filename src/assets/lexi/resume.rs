use crate::{
    AppState,
    assets::lexi::{LexiCollection, Lexicon},
};
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<ResumeData>::new(&[".json"]));
    app.add_systems(OnEnter(AppState::Preload), preload);
}

fn preload(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading Resume");
    commands.insert_resource(LexiCollection::<ResumeData>::new(
        &asset_server,
        vec![
            "lexi/home.json",
            "lexi/work.json",
            "lexi/homedev.json",
            "lexi/homedev/pico-party.json",
            "lexi/homedev/pumpkin-sound.json",
            "lexi/links.json",
        ],
    ));
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct ResumeData {
    pub id: String,
    pub lex: Lexicon,
    pub image_reel: Option<Vec<String>>,
    pub choices: Option<Vec<Choice>>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Display {}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Choice {
    pub id: String,
    pub choice: ChoiceLex,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct ChoiceLex {
    pub lex: Lexicon,
    pub link: Option<String>,
    pub action: Option<String>,
    pub next_id: Option<String>,
}
