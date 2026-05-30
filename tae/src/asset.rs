use raylib::prelude::*;
use tae_core::FileStore;

/// Load a Raylib texture from a path inside the FileStore.
/// Returns None if the file is missing or can't be decoded.
pub fn load_texture(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    store: &FileStore,
    path: &str,
) -> Option<Texture2D> {
    let bytes = store.get(path)?;
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");
    let file_type = format!(".{ext}");
    let image = Image::load_image_from_mem(&file_type, bytes).ok()?;
    rl.load_texture_from_image(thread, &image).ok()
}
