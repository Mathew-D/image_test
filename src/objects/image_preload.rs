/*
Made by: Your Name Here
Date: 2025-05-03
Program Details: Central texture manager for preloading and sharing textures
To use this:

In the mod objects section add:
    pub mod image_preload;
    
Then add the following with the use commands:
use objects::image_preload::TextureManager;

*/
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::collections::HashMap;
use crate::objects::images_obj::set_texture_main;

/// A central texture manager to preload and share textures
/// This reduces memory usage and prevents flickering when switching images
pub struct TextureManager {
    textures: HashMap<String, (Texture2D, Vec<u8>)>,
    friendly_names: HashMap<String, String>,
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            friendly_names: HashMap::new(),
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
        
        // Also create an empty texture
        self.create_empty_texture();
    }
    
    /// Associate friendly names with texture paths
    pub fn add_friendly_names(&mut self, names: &[&str], paths: &[&str]) {
        if names.len() != paths.len() {
            println!("Warning: Names and paths arrays must be the same length");
            return;
        }
        
        for i in 0..names.len() {
            self.friendly_names.insert(names[i].to_string(), paths[i].to_string());
        }
    }
    
    /// Get a texture by its file path (cloned for use)
    pub fn get(&self, path: &str) -> Option<(Texture2D, Vec<u8>)> {
        self.textures.get(path).map(|(texture, mask)| (texture.clone(), mask.clone()))
    }
    
    /// Get a texture with its filename by its file path
    pub fn get_with_filename(&self, path: &str) -> Option<(Texture2D, Vec<u8>, String)> {
        self.textures.get(path).map(|(texture, mask)| 
            (texture.clone(), mask.clone(), path.to_string())
        )
    }
    
    /// Get a texture by its friendly name (cloned for use)
    pub fn get_by_name(&self, name: &str) -> Option<(Texture2D, Vec<u8>)> {
        if let Some(path) = self.friendly_names.get(name) {
            self.get(path)
        } else {
            None
        }
    }
    
    /// Get a texture with its filename by its friendly name
    pub fn get_by_name_with_filename(&self, name: &str) -> Option<(Texture2D, Vec<u8>, String)> {
        if let Some(path) = self.friendly_names.get(name) {
            self.get_with_filename(path)
        } else {
            None
        }
    }
    
    /// Get a list of all preloaded texture paths
    pub fn get_texture_paths(&self) -> Vec<String> {
        self.textures.keys().cloned().collect()
    }
    
    /// Get a list of all friendly names
    pub fn get_friendly_names(&self) -> Vec<String> {
        self.friendly_names.keys().cloned().collect()
    }
    
    /// Check if a texture is preloaded by path
    pub fn is_texture_preloaded(&self, path: &str) -> bool {
        self.textures.contains_key(path)
    }
    
    /// Check if a friendly name is registered
    pub fn has_friendly_name(&self, name: &str) -> bool {
        self.friendly_names.contains_key(name)
    }
    
    /// Get the file path for a friendly name
    pub fn get_path_for_name(&self, name: &str) -> Option<String> {
        self.friendly_names.get(name).cloned()
    }
    
    /// Create an empty texture (1x1 transparent pixel)
    pub fn create_empty_texture(&mut self) {
        if !self.textures.contains_key("__empty__") {
            let empty_texture = Texture2D::from_rgba8(1, 1, &[0, 0, 0, 0]);
            let empty_mask = vec![0]; // Single transparent pixel
            self.textures.insert("__empty__".to_string(), (empty_texture, empty_mask));
        }
    }
    
    /// Get the empty texture
    pub fn get_empty_texture(&self) -> Option<(Texture2D, Vec<u8>)> {
        self.get("__empty__")
    }
}