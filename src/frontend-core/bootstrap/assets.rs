use std::path::PathBuf;

pub fn locate_assets() -> PathBuf {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();

    debug!("Assets located at: {}", assets.display());

    assets
}