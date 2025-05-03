use macroquad::prelude::*;
use std::collections::HashMap;

// A combined texture cache and image viewer that handles preloading
// and displaying images with no flickering

// The texture cache stores preloaded textures
struct TextureCache {
    textures: HashMap<String, (Texture2D, Vec<u8>)>,
}

impl TextureCache {
    fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    // Preload a texture and store it in the cache
    async fn preload(&mut self, path: &str) {
        if !self.textures.contains_key(path) {
            let texture = load_texture(path).await.unwrap();
            texture.set_filter(FilterMode::Linear);
            
            let tex_width = texture.width() as usize;
            let tex_height = texture.height() as usize;
            
            let image = load_image(path).await.unwrap();
            let pixels = image.bytes;
            
            let mut mask = vec![0; (tex_width * tex_height + 7) / 8];
            for y in 0..tex_height {
                for x in 0..tex_width {
                    let idx = (y * tex_width + x) * 4;
                    let alpha = pixels[idx + 3];
                    let mask_byte_idx = (y * tex_width + x) / 8;
                    let bit_offset = (y * tex_width + x) % 8;
                    
                    if alpha > 0 {
                        mask[mask_byte_idx] |= 1 << (7 - bit_offset);
                    } else {
                        mask[mask_byte_idx] &= !(1 << (7 - bit_offset));
                    }
                }
            }
            
            self.textures.insert(path.to_string(), (texture, mask));
        }
    }

    // Get a preloaded texture
    fn get(&self, path: &str) -> Option<(Texture2D, Vec<u8>)> {
        self.textures.get(path).map(|(texture, mask)| (texture.clone(), mask.clone()))
    }
}

// The image viewer that uses the texture cache
pub struct ImageViewer {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    cache: TextureCache,
    image_paths: Vec<String>,
    current_index: usize,
}

impl ImageViewer {
    // Create a new ImageViewer and preload all images
    pub async fn new(
        image_paths: &[&str],
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
        let mut cache = TextureCache::new();
        
        // Preload all images at startup
        for path in image_paths {
            cache.preload(path).await;
        }
        
        // Store image paths as Strings
        let paths = image_paths.iter().map(|&s| s.to_string()).collect();
        
        Self {
            x,
            y,
            width,
            height,
            cache,
            image_paths: paths,
            current_index: 0,
        }
    }
    
    // Draw the current image
    pub fn draw(&self) {
        // Only draw if we have images
        if self.image_paths.is_empty() {
            return;
        }
        
        // Get the current image path
        let current_path = &self.image_paths[self.current_index];
        
        // Get the texture from cache
        if let Some((texture, _)) = self.cache.get(current_path) {
            draw_texture_ex(
                &texture,
                self.x,
                self.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.width, self.height)),
                    ..Default::default()
                },
            );
        }
    }
    
    // Move to the next image
    pub fn next_image(&mut self) {
        if self.image_paths.len() <= 1 {
            return;
        }
        
        // Move to the next image
        self.current_index = (self.current_index + 1) % self.image_paths.len();
    }
    
    // Get position as Vec2
    pub fn position(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
    
    // Get size as Vec2
    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }
    
    // Show a specific image by index
    pub fn show_image_by_index(&mut self, index: usize) -> bool {
        if self.image_paths.is_empty() || index >= self.image_paths.len() {
            return false;
        }
        
        self.current_index = index;
        true
    }
    
    // Show a specific image by filename
    pub fn show_image_by_filename(&mut self, filename: &str) -> bool {
        if let Some(index) = self.image_paths.iter().position(|path| path == filename) {
            self.current_index = index;
            return true;
        }
        false
    }
    
    // Get the current image index
    pub fn current_index(&self) -> usize {
        self.current_index
    }
    
    // Get the total number of images
    pub fn image_count(&self) -> usize {
        self.image_paths.len()
    }
    
    // Get the current image filename
    pub fn current_filename(&self) -> Option<&str> {
        if self.image_paths.is_empty() {
            return None;
        }
        Some(&self.image_paths[self.current_index])
    }
    
    // Add a new image to the cache (for dynamic loading)
    pub async fn add_image(&mut self, path: &str) -> bool {
        // Check if image is already in the cache
        if self.image_paths.iter().any(|p| p == path) {
            return false;
        }
        
        // Preload the new image
        self.cache.preload(path).await;
        
        // Add the path to our list
        self.image_paths.push(path.to_string());
        
        true
    }
}