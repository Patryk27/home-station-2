use std::collections::HashMap;
use std::path::Path;

use crate::{Display, TextureLoader};
use crate::vendor::{ImageId, ImageMap};

pub struct TextureController<'a> {
    display: &'a Display,
    image_map: ImageMap,
    images: HashMap<String, ImageId>,
}

impl<'a> TextureController<'a> {
    pub fn new(display: &'a Display, image_map: ImageMap) -> Self {
        Self {
            display,
            image_map,
            images: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, assets: &Path) {
        for (texture_name, texture) in TextureLoader::load_dir(self.display, assets) {
            let texture_id = self.image_map.insert(texture);

            trace!("Texture [{}] is now available as [{:?}]", texture_name, texture_id);

            self.images.insert(texture_name, texture_id);
        }
    }

    pub fn get(&self, name: &str) -> Option<ImageId> {
        self.images
            .get(name)
            .map(ToOwned::to_owned)
    }

    pub fn get_ex(&self, name: &[&str]) -> Option<ImageId> {
        self.get(
            &name.join(":"),
        )
    }

    pub fn image_map(&self) -> &ImageMap {
        &self.image_map
    }
}