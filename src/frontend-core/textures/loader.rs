use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use glium::texture::RawImage2d;

use crate::Display;
use crate::vendor::{Texture2d, TextureName};

pub struct TextureLoader;

impl TextureLoader {
    pub fn name(base: &Path, texture_dir: &Path, texture_file: &Path) -> String {
        let mut name: Vec<_> = texture_dir
            .strip_prefix(base)
            .unwrap()
            .components()
            .filter_map(|component| match component {
                Component::Normal(str) => {
                    Some(str.to_string_lossy().to_string())
                }

                _ => {
                    None
                }
            })
            .collect();

        name.push(
            texture_file
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        );

        name.join(":")
    }

    pub fn load_dir(display: &Display, path: &Path) -> HashMap<TextureName, Texture2d> {
        debug!("Loading textures from directory: {}", path.display());

        let mut textures = HashMap::new();

        let mut pending_dirs = vec![
            PathBuf::from(path),
        ];

        while let Some(dir) = pending_dirs.pop() {
            for entry in fs::read_dir(&dir).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let entry_meta = fs::metadata(&entry_path).unwrap();

                if entry_meta.is_dir() {
                    pending_dirs.push(entry.path());
                    continue;
                }

                if let Some(texture) = Self::load(display, &entry_path) {
                    let texture_name = Self::name(path, &dir, &entry_path);

                    textures.insert(texture_name, texture);
                }
            }
        }

        textures
    }

    pub fn load(display: &Display, texture: &Path) -> Option<Texture2d> {
        if texture.extension() == Some(OsStr::new("png")) {
            Some(Self::load_png(display, texture))
        } else {
            None
        }
    }

    fn load_png(display: &Display, texture: &Path) -> Texture2d {
        let image = image::open(texture)
            .unwrap()
            .to_rgba();

        let image_dim = image.dimensions();
        let image_data = image.into_raw();

        let texture = RawImage2d::from_raw_rgba_reversed(
            &image_data, image_dim,
        );

        Texture2d::new(&display.0, texture).unwrap()
    }
}