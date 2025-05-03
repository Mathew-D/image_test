/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a image object 
In the mod objects section add:
    pub mod images_obj;
    
Then add the following with the use commands:
use objects::images_obj::ImageObject;

Then to use this you would put the following above the loop: 
    let img = ImageObject::new(
        "assets/image_name.png",
        100.0,
        200.0,
        200.0,
        60.0,
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    ).await;

    // Or with custom stretch and zoom options:
    let img_custom = ImageObject::new(
        "assets/image_name.png",
        100.0,
        200.0,
        200.0,
        60.0,
        false,  // Disable stretching
        1.5,    // Set zoom to 150%
    ).await;

Then in side the loop you would use:
img.draw();
*/
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::collections::HashMap;

pub struct ImageObject {
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    transparency_mask: Vec<u8>, // Now storing raw transparency data (bitmask)
    stretch_enabled: bool, // Flag to control image stretching
    zoom_level: f32, // Zoom factor to scale the image
    // Add preloaded textures storage
    preloaded_textures: HashMap<String, (Texture2D, Vec<u8>)>,
}

impl ImageObject {
    // Constructor for ImageObject with asset path and x, y location
    pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32
    ) -> Self {
        let (texture, transparency_mask) = set_texture_main(asset_path).await;
        Self { 
            x, 
            y, 
            width, 
            height, 
            texture, 
            transparency_mask,
            stretch_enabled,
            zoom_level: zoom_level.max(0.1), // Ensure minimum zoom
            preloaded_textures: HashMap::new(),
        }
    }

    // Method to draw the image with current settings
    pub fn draw(&self) {
        // Get the size to use for drawing
        let (draw_width, draw_height) = if self.stretch_enabled {
            (self.width, self.height)
        } else {
            // Use original texture size when stretch is disabled
            (self.texture.width(), self.texture.height())
        };
        
        // Apply zoom factor
        let final_width = draw_width * self.zoom_level;
        let final_height = draw_height * self.zoom_level;
        
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(final_width, final_height)),
                ..Default::default()
            },
        );
    }

    // Accessors for image properties
    #[allow(unused)]
    pub fn pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
    #[allow(unused)]
    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }
    #[allow(unused)]
    pub fn texture_size(&self) -> Vec2 {
        vec2(self.texture.width(), self.texture.height())
    }
    #[allow(unused)]
    pub fn set_position(&mut self, pos: Vec2) {
        self.x = pos[0];
        self.y = pos[1];
    }

    // Get the transparency mask (bitmask)
    #[allow(unused)]
    pub fn get_mask(&self) -> Vec<u8> {
        return self.transparency_mask.clone();
    }
    #[allow(unused)]
    pub async fn set_texture(&mut self, texture_path: &str) {
        let (texture, transparency_mask) = set_texture_main(texture_path).await;
        self.texture = texture;
        self.transparency_mask = transparency_mask;
    }
    
    // Methods to toggle stretching
    #[allow(unused)]
    pub fn enable_stretch(&mut self) {
        self.stretch_enabled = true;
    }
    
    #[allow(unused)]
    pub fn disable_stretch(&mut self) {
        self.stretch_enabled = false;
    }
    
    #[allow(unused)]
    pub fn toggle_stretch(&mut self) {
        self.stretch_enabled = !self.stretch_enabled;
    }
    
    #[allow(unused)]
    pub fn is_stretch_enabled(&self) -> bool {
        self.stretch_enabled
    }
    
    #[allow(unused)]
    pub fn set_stretch(&mut self, enabled: bool) {
        self.stretch_enabled = enabled;
    }
    
    // Zoom methods
    #[allow(unused)]
    pub fn set_zoom(&mut self, zoom_level: f32) {
        self.zoom_level = zoom_level.max(0.1); // Prevent zoom from going too small
    }
    
    #[allow(unused)]
    pub fn zoom_in(&mut self, amount: f32) {
        self.zoom_level += amount;
        if self.zoom_level < 0.1 {
            self.zoom_level = 0.1; // Minimum zoom level
        }
    }
    
    #[allow(unused)]
    pub fn zoom_out(&mut self, amount: f32) {
        self.zoom_level -= amount;
        if self.zoom_level < 0.1 {
            self.zoom_level = 0.1; // Minimum zoom level
        }
    }
    
    #[allow(unused)]
    pub fn get_zoom_level(&self) -> f32 {
        self.zoom_level
    }
    
    #[allow(unused)]
    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
    }

    // New method to preload textures outside the main loop
    #[allow(unused)]
    pub async fn preload_textures(&mut self, texture_paths: &[&str]) {
        // Initialize the HashMap if needed
        if self.preloaded_textures.is_empty() {
            self.preloaded_textures = HashMap::new();
        }
        
        // Preload all specified textures
        for &path in texture_paths {
            if !self.preloaded_textures.contains_key(path) {
                let (texture, mask) = set_texture_main(path).await;
                self.preloaded_textures.insert(path.to_string(), (texture, mask));
            }
        }
    }
    
    // New method to switch to a preloaded texture instantly (no flickering)
    #[allow(unused)]
    pub fn switch_texture(&mut self, texture_path: &str) -> bool {
        if let Some((texture, mask)) = self.preloaded_textures.get(texture_path) {
            // Clone the texture and mask
            self.texture = texture.clone();
            self.transparency_mask = mask.clone();
            true
        } else {
            false
        }
    }
    
    // New method to clear the image (show nothing) without flickering
    #[allow(unused)]
    pub fn clear_image(&mut self) {
        // Create a 1x1 transparent pixel texture if none exists in the preloaded cache
        if !self.preloaded_textures.contains_key("__empty__") {
            // Create an empty 1x1 transparent texture
            let empty_texture = Texture2D::from_rgba8(1, 1, &[0, 0, 0, 0]);
            let empty_mask = vec![0]; // Single transparent pixel
            
            // Add it to the preloaded textures
            self.preloaded_textures.insert("__empty__".to_string(), (empty_texture, empty_mask));
        }
        
        // Switch to the empty texture
        if let Some((texture, mask)) = self.preloaded_textures.get("__empty__") {
            self.texture = texture.clone();
            self.transparency_mask = mask.clone();
        }
    }
    
    // Check if the image is currently cleared/empty
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.texture.width() == 1.0 && self.texture.height() == 1.0
    }
    
    // Check if collision should be performed (not empty)
    #[allow(unused)]
    pub fn is_collidable(&self) -> bool {
        !self.is_empty()
    }

    // Method to switch to a preloaded texture by filename
    #[allow(unused)]
    pub fn switch_texture_by_name(&mut self, texture_path: &str) -> bool {
        if let Some((texture, mask)) = self.preloaded_textures.get(texture_path) {
            // Clone the texture and mask
            self.texture = texture.clone();
            self.transparency_mask = mask.clone();
            true
        } else {
            false
        }
    }
    
    // Get the list of preloaded texture names
    #[allow(unused)]
    pub fn get_preloaded_texture_names(&self) -> Vec<String> {
        self.preloaded_textures.keys().cloned().collect()
    }
    
    // Check if a specific texture is preloaded
    #[allow(unused)]
    pub fn is_texture_preloaded(&self, texture_path: &str) -> bool {
        self.preloaded_textures.contains_key(texture_path)
    }
    
    // Get the current texture path (if it can be determined)
    #[allow(unused)]
    pub fn get_current_texture_path(&self) -> Option<String> {
        // Compare the current texture with preloaded textures to find a match
        for (path, (texture, _)) in &self.preloaded_textures {
            // Check if width, height and a few pixel values match as a heuristic
            if texture.width() == self.texture.width() && 
               texture.height() == self.texture.height() {
                // This is a simplification - in a real implementation you might
                // want to store the current path as a field when switching
                return Some(path.clone());
            }
        }
        None
    }
}

async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Vec<u8> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format

    let mut mask = vec![0; (width * height + 7) / 8]; // Create a bitmask with enough bytes

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            let mask_byte_idx = (y * width + x) / 8; // Find which byte this pixel belongs to
            let bit_offset = (y * width + x) % 8; // Find the bit offset inside the byte

            if alpha > 0 {
                // Set the bit to 1 (opaque pixel)
                mask[mask_byte_idx] |= 1 << (7 - bit_offset);
            } else {
                // Set the bit to 0 (transparent pixel)
                mask[mask_byte_idx] &= !(1 << (7 - bit_offset));
            }
        }
    }

    mask
}
pub async fn set_texture_main(texture_path: &str) -> (Texture2D, Vec<u8>) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Linear);
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;
    return (texture, transparency_mask);
}

