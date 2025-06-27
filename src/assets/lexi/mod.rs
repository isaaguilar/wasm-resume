use bevy::prelude::*;
use std::collections::HashMap;

use crate::AppState;

pub mod resume;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(resume::plugin);
    app.add_systems(Update, preload.run_if(in_state(AppState::Preload)));
}

#[derive(Resource, Debug, Default)]
#[allow(dead_code)]
pub struct LexiCollection<A: Asset + TypePath>(pub Vec<Handle<A>>);

impl<A: Asset + TypePath> LexiCollection<A> {
    fn new(asset_server: &Res<AssetServer>, files: Vec<impl Into<String>>) -> Self
    where
        A: Asset + TypePath,
    {
        let handles = files
            .into_iter()
            .map(|file| asset_server.load(&file.into()))
            .collect::<Vec<_>>();

        Self(handles)
    }
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Default, Clone)]
pub struct Lexicon {
    pub translations: HashMap<String, String>,
}

impl Lexicon {
    pub fn from_language(&self, language: impl Into<String>) -> String {
        self.translations
            .get(&language.into())
            .cloned()
            .unwrap_or(String::new())
    }
}

fn preload(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Loading);
}
