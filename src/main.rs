/*
By: Your Name Here
Date: 2025-05-03
Program Details: Image viewer with filename-based image switching
*/
mod objects {
    pub mod images_obj;
    pub mod txt_buttons;
    pub mod label;
}

use macroquad::prelude::*;
use objects::images_obj::ImageObject;
use objects::txt_buttons::TextButton;
use objects::label::Label;

#[macroquad::main("image_test")]
async fn main() {
    // Image paths to use
    let image_paths = ["assets/image1.png", "assets/image2.png"];
    
    // Create a single ImageObject
    let mut image = ImageObject::new(
        image_paths[0],
        300.0,  // width
        200.0,  // height
        screen_width() / 2.0 - 150.0,  // x (centered)
        screen_height() / 2.0 - 100.0, // y (centered)
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    // Preload all textures before entering the main loop
    image.preload_textures(&image_paths).await;
    
    // Create buttons for different images
    let mut image1_button = TextButton::new(
        screen_width() / 2.0 - 150.0,  // x (left side)
        screen_height() / 2.0 + 150.0, // y (below the image)
        120.0,  // width
        50.0,   // height
        "Show Image 1",
        BLUE,   // normal color
        GREEN,  // hover color
        20      // font size
    );
    
    let mut image2_button = TextButton::new(
        screen_width() / 2.0 + 30.0,   // x (right side)
        screen_height() / 2.0 + 150.0, // y (below the image)
        120.0,  // width
        50.0,   // height
        "Show Image 2",
        BLUE,   // normal color
        GREEN,  // hover color
        20      // font size
    );
    
    let mut clear_button = TextButton::new(
        screen_width() / 2.0 - 60.0,   // x (center)
        screen_height() / 2.0 + 210.0, // y (below the other buttons)
        120.0,  // width
        50.0,   // height
        "Clear Image",
        PURPLE, // normal color
        PINK,   // hover color
        20      // font size
    );
    
    // Apply button styling
    for button in [&mut image1_button, &mut image2_button, &mut clear_button].iter_mut() {
        button.with_round(10.0)
              .with_border(WHITE, 2.0)
              .with_text_color(WHITE)
              .with_hover_text_color(YELLOW);
    }
    
    // Create a label for instructions
    let mut instructions_label = Label::new(
        "Click a button to display an image", 
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
    
    // Create a label to show the current image info
    let mut info_label = Label::new(
        "", 
        screen_width() / 2.0 - 200.0,  // x (centered)
        screen_height() / 2.0 + 280.0, // y (below buttons)
        18                            // font size
    );
    
    // Apply styling to the info label
    info_label.with_colors(WHITE, Some(DARKBLUE))
              .with_round(8.0)
              .with_border(BLUE, 1.5)
              .with_fixed_size(400.0, 60.0)
              .with_alignment(objects::label::TextAlign::Center);
    
    // Create a label to list available textures
    let mut preloaded_label = Label::new(
        "Preloaded textures:", 
        screen_width() / 2.0 - 200.0,  // x (centered)
        screen_height() / 2.0 + 350.0, // y (bottom of screen)
        16                           // font size
    );
    
    // Apply styling to the preloaded textures label
    preloaded_label.with_colors(WHITE, Some(DARKGREEN))
                   .with_round(8.0)
                   .with_border(GREEN, 1.5)
                   .with_fixed_size(400.0, 100.0)
                   .with_alignment(objects::label::TextAlign::Center);
                   
    // Set the preloaded textures text once at startup
    let preloaded_names = image.get_preloaded_texture_names();
    let preloaded_text = format!("Preloaded textures:\n{}", preloaded_names.join("\n"));
    preloaded_label.set_text(&preloaded_text);
    
    loop {
        clear_background(LIGHTGRAY);
        
        // Draw the current image
        image.draw();
        
        // Draw the instructions label
        instructions_label.draw();
        
        // Update the current image info text
        let current_path = image.get_current_texture_path().unwrap_or_else(|| "Unknown".to_string());
        let info_text = if image.is_empty() {
            "Current Image: None (cleared)".to_string()
        } else {
            format!("Current Image: {}", current_path)
        };
        info_label.set_text(&info_text);
        info_label.draw();
        
        // Draw the preloaded textures label
        preloaded_label.draw();
        
        // Handle button clicks
        if image1_button.click() {
            // Switch to image 1 by filename
            image.switch_texture_by_name("assets/image1.png");
        }
        
        if image2_button.click() {
            // Switch to image 2 by filename
            image.switch_texture_by_name("assets/image2.png");
        }
        
        if clear_button.click() {
            // Clear the image
            image.clear_image();
        }

        next_frame().await;
    }
}
