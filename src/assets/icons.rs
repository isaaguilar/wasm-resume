use bevy::ecs::resource::Resource;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]

pub struct IconAssets {
    #[asset(path = "notepad.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub notepad: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 8, rows = 1))]
    pub notepad_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "links.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub links: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 1))]
    pub links_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "window.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub window: Handle<Image>,

    #[asset(path = "window2.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub window2: Handle<Image>,

    #[asset(path = "close.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub close: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 14, tile_size_y = 14, columns = 2, rows = 1))]
    pub close_layout: Handle<TextureAtlasLayout>,
}
