use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;

use gfx_device_gl::Resources;
use piston_window::*;

pub type TextureHandle = Arc<Texture<Resources>>;

/// A HashMap-based texture store.
pub struct TextureStore {
    textures: HashMap<String, TextureHandle>,
}

impl TextureStore {
    /// Get the texture from its filename (e.g. "my_texture.png").
    pub fn get<S: Into<String>>(&self, key: S) -> TextureHandle {
        let key = key.into();
        self.textures
            .get(&key)
            .expect(&format!("TextureStore : no texture found with key \"{}\"", key))
            .clone()
    }

    /// Create a new TextureStore instance and load all the *.png images in the
    /// given assets folder.
    /// Each texture will be stored with their filename set as their unique key.
    pub fn new(window: &mut PistonWindow, assets: &Path) -> Self {
        let paths = fs::read_dir(assets)
                        .ok()
                        .expect("could not enumerate the content of the provided assets folder");

        let mut store = HashMap::new();

        for filename in paths {
            let filepath = filename.unwrap().path();
            if !filepath.is_file() {
                continue;
            }
            if let Some(ext) = filepath.extension() {
                if ext == "png" {
                    let texture = Texture::from_path(&mut window.factory,
                                                     &filepath,
                                                     Flip::None,
                                                     &TextureSettings::new())
                                      .ok()
                                      .expect(&format!("failed to load the texture : {}",
                                                       filepath.display()));

                    store.insert(filepath.file_name()
                                         .unwrap()
                                         .to_str()
                                         .unwrap()
                                         .to_string(),
                                 Arc::new(texture));
                }
            }
        }

        TextureStore { textures: store }
    }
}
