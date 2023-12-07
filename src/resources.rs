use macroquad::prelude::*;
use once_cell::sync::OnceCell;

//shamelessly ripped off
pub static RESOURCES: OnceCell<Resources> = OnceCell::new();

pub struct Resources {
    pub house_texture: Texture2D,
    pub santa_texture: Texture2D,
}

pub async fn init_resources() {
    let resources = Resources::new().await;
    match RESOURCES.set(resources) {
        Ok(_) => println!("Resources init successfull"),
        Err(_) => panic!("Failed to load Resources"),
    };
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            house_texture: load_texture("res/blue-house-with-smoke-1.png").await.unwrap(),
            santa_texture: load_texture("res/santa.png").await.unwrap(),
        }
    }
}