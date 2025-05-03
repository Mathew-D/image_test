/*
By: Your Name Here
Date: 2025-05-03
Program Details: Image viewer with centralized texture preloading and name-based image selection
*/
mod objects {
    pub mod images_obj;
    pub mod txt_buttons;
    pub mod label;
    pub mod image_preload;
}

use macroquad::prelude::*;
use objects::images_obj::ImageObject;
use objects::txt_buttons::TextButton;
use objects::label::Label;
use objects::image_preload::TextureManager;

#[macroquad::main("image_test")]
async fn main() {
    // Image paths to use - using descriptive names
    let image_paths = ["assets/image1.png", "assets/image2.png"];
    
    // Create friendly names for our images
    let image_names = ["First Image", "Second Image"];
    
    // Create and initialize the texture manager
    let mut texture_manager = TextureManager::new();
    
    // Preload all textures at startup
    texture_manager.preload_all(&image_paths).await;
    
    // Add friendly names mapping
    texture_manager.add_friendly_names(&image_names, &image_paths);
    
    // Create first image object with the first texture
    let (texture1, mask1) = texture_manager.get(&image_paths[0]).unwrap();
    
    let mut image1 = ImageObject::new(
        image_paths[0],
        200.0,  // width
        150.0,  // height
        screen_width() / 2.0 - 250.0,  // left side
        screen_height() / 2.0 - 100.0, // centered vertically
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    // Replace the loaded texture with our preloaded one
    image1.set_preloaded_texture(texture1, mask1);
    
    // Create a second ImageObject
    let (texture2, mask2) = texture_manager.get(&image_paths[1]).unwrap();
    
    let mut image2 = ImageObject::new(
        image_paths[0], // Start with same image but we'll override it
        200.0,  // width
        150.0,  // height
        screen_width() / 2.0 + 50.0,   // right side
        screen_height() / 2.0 - 100.0, // centered vertically
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    // Set the preloaded texture for the second image
    image2.set_preloaded_texture(texture2, mask2);
    
    // Create buttons for image selection
    // First row of buttons for Image 1
    let mut img1_buttons = Vec::new();
    for (i, name) in image_names.iter().enumerate() {
        let mut btn = TextButton::new(
            screen_width() / 2.0 - 250.0 + (i as f32 * 100.0),  // horizontal spread below image1
            screen_height() / 2.0 + 80.0, // below the first image
            90.0,   // width
            40.0,   // height
            *name,  // Dereference the &str to str
            BLUE,   // normal color
            GREEN,  // hover color
            14      // font size
        );
        
        btn.with_round(10.0)
           .with_border(WHITE, 2.0)
           .with_text_color(WHITE)
           .with_hover_text_color(YELLOW);
        
        img1_buttons.push(btn);
    }
    
    // Second row of buttons for Image 2
    let mut img2_buttons = Vec::new();
    for (i, name) in image_names.iter().enumerate() {
        let mut btn = TextButton::new(
            screen_width() / 2.0 + 50.0 + (i as f32 * 100.0),  // horizontal spread below image2
            screen_height() / 2.0 + 80.0, // below the second image
            90.0,   // width
            40.0,   // height
            *name,  // Dereference the &str to str
            PURPLE, // normal color
            PINK,   // hover color
            14      // font size
        );
        
        btn.with_round(10.0)
           .with_border(WHITE, 2.0)
           .with_text_color(WHITE)
           .with_hover_text_color(YELLOW);
        
        img2_buttons.push(btn);
    }
    
    // Create button for clearing/restoring images
    let mut clear_button = TextButton::new(
        screen_width() / 2.0 - 60.0,  // centered
        screen_height() / 2.0 + 160.0, // below other buttons
        120.0,  // width
        50.0,   // height
        "Clear Images",
        DARKPURPLE, // normal color
        PURPLE,     // hover color
        20          // font size
    );
    
    clear_button.with_round(10.0)
                .with_border(WHITE, 2.0)
                .with_text_color(WHITE)
                .with_hover_text_color(YELLOW);
    
    // Create a label for instructions
    let mut instructions_label = Label::new(
        "Click the buttons to change images by name", 
        screen_width() / 2.0 - 200.0,  // x (centered)
        50.0,                          // y (top of screen)
        24                            // font size
    );
    
    // Apply styling to the label
    instructions_label.with_colors(WHITE, Some(DARKGRAY))
                      .with_round(8.0)
                      .with_border(GRAY, 1.5)
                      .with_fixed_size(400.0, 40.0)
                      .with_alignment(objects::label::TextAlign::Center);
    
    // Add a label to show available textures
    let mut textures_label = Label::new(
        format!("Available textures: {}", texture_manager.get_friendly_names().join(", ")),
        screen_width() / 2.0 - 200.0,  // x (centered)
        screen_height() - 50.0,        // y (bottom of screen)
        16                             // font size
    );
    
    textures_label.with_colors(WHITE, Some(DARKGREEN))
                  .with_round(8.0)
                  .with_border(GREEN, 1.5)
                  .with_fixed_size(400.0, 30.0)
                  .with_alignment(objects::label::TextAlign::Center);
    
    // Track currently displayed images by name
    let mut image1_name = image_names[0].to_string();
    let mut image2_name = image_names[1].to_string();
    
    loop {
        clear_background(LIGHTGRAY);
        
        // Draw both images
        image1.draw();
        image2.draw();
        
        // Draw the instructions label
        instructions_label.draw();
        
        // Draw the textures info label
        textures_label.draw();
        
        // Update button text based on current image state
        if image1.is_empty() && image2.is_empty() {
            clear_button.set_text("Restore");
        } else {
            clear_button.set_text("Clear");
        }
        
        // Draw labels indicating which image is showing
        let label_text1 = if !image1.is_empty() { 
            format!("Showing: {}", image1_name) 
        } else { 
            "Cleared".to_string() 
        };
        
        let label_text2 = if !image2.is_empty() { 
            format!("Showing: {}", image2_name) 
        } else { 
            "Cleared".to_string() 
        };
        
        // Draw small labels above each image
        draw_text(&label_text1, image1.pos().x, image1.pos().y - 10.0, 20.0, WHITE);
        draw_text(&label_text2, image2.pos().x, image2.pos().y - 10.0, 20.0, WHITE);
        
        // Handle image 1 buttons
        for (i, button) in img1_buttons.iter_mut().enumerate() {
            if button.click() {
                // Update the image name
                image1_name = image_names[i].to_string();
                
                // Get and apply the texture using the name-based lookup
                if let Some((texture, mask)) = texture_manager.get_by_name(&image1_name) {
                    image1.set_preloaded_texture(texture, mask);
                }
            }
        }
        
        // Handle image 2 buttons
        for (i, button) in img2_buttons.iter_mut().enumerate() {
            if button.click() {
                // Update the image name
                image2_name = image_names[i].to_string();
                
                // Get and apply the texture using the name-based lookup
                if let Some((texture, mask)) = texture_manager.get_by_name(&image2_name) {
                    image2.set_preloaded_texture(texture, mask);
                }
            }
        }
        
        // Handle clear/restore button
        if clear_button.click() {
            if !image1.is_empty() || !image2.is_empty() {
                // Clear both images
                if let Some((empty_texture, empty_mask)) = texture_manager.get_empty_texture() {
                    image1.set_preloaded_texture(empty_texture.clone(), empty_mask.clone());
                    image2.set_preloaded_texture(empty_texture, empty_mask);
                }
            } else {
                // Restore the images
                if let Some((texture, mask)) = texture_manager.get_by_name(&image1_name) {
                    image1.set_preloaded_texture(texture, mask);
                }
                
                if let Some((texture, mask)) = texture_manager.get_by_name(&image2_name) {
                    image2.set_preloaded_texture(texture, mask);
                }
            }
        }

        next_frame().await;
    }
}
