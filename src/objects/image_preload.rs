/*
Made by: Your Name Here
Date: 2025-05-03
Program Details: Central texture manager for preloading and sharing textures

To use this:
1. In the mod objects section add:
    pub mod image_preload;
    
2. Add the following use command:
    use objects::image_preload::TextureManager;

3. Create and initialize a TextureManager:
    let mut texture_manager = TextureManager::new();
    
4. Preload your textures at startup:
    // Preload a list of textures
    texture_manager.preload_all(&["assets/image1.png", "assets/image2.png"]).await;
    
    // Or preload individual textures
    texture_manager.preload("assets/image3.png").await;
    
5. Get preloaded textures for use with ImageObject:
    // Since all textures are preloaded, you can directly pass the result of get_preload() 
    // to set_preload() without intermediate variables:
    image_obj.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());
    
    // The unwrap() is safe because we know the texture was preloaded

Note: For clearing images, use the clear() method directly on the ImageObject:
    image_obj.clear();
*/
use macroquad::texture::Texture2D;
use std::collections::HashMap;
use crate::objects::images_obj::set_texture_main;

/// A central texture manager to preload and share textures
/// This reduces memory usage and prevents flickering when switching images
pub struct TextureManager {
    textures: HashMap<String, (Texture2D, Vec<u8>)>,
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
    
    /// Preload a texture by its file path
    pub async fn preload(&mut self, path: &str) {
        if !self.textures.contains_key(path) {
            let (texture, mask) = set_texture_main(path).await;
            self.textures.insert(path.to_string(), (texture, mask));
        }
    }
    
    /// Preload multiple textures at once
    pub async fn preload_all(&mut self, paths: &[&str]) {
        for path in paths {
            self.preload(path).await;
        }
    }
    
    /// Get a preloaded texture for use in an ImageObject
    pub fn get_preload(&self, path: &str) -> Option<(Texture2D, Vec<u8>, String)> {
        self.textures.get(path).map(|(texture, mask)| 
            (texture.clone(), mask.clone(), path.to_string())
        )
    }
}