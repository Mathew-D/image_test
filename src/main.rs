/*
By: Your Name Here
Date: 2025-05-03
Program Details: Image viewer with centralized texture preloading and name-based image selection
*/
mod objects {
    pub mod image_preload;
    pub mod images_obj;
    pub mod label;
    pub mod txt_buttons;
}

use macroquad::prelude::*;
use objects::image_preload::TextureManager;
use objects::images_obj::ImageObject;
use objects::label::Label;
use objects::txt_buttons::TextButton;

#[macroquad::main("image_test")]
async fn main() {
     // Create and initialize the texture manager
    let mut texture_manager = TextureManager::new();

    // Preload all textures at startup
    texture_manager.preload_all(&["assets/image1.png", "assets/image2.png"]).await;

    // Create a single image object
    let mut image_obj = ImageObject::new(
        "assets/image1.png", 
        300.0, 
        200.0, 
        screen_width() / 2.0 - 150.0, 
        screen_height() / 2.0 - 150.0, 
        true, 
        1.0
    ).await;

    // Create three buttons: two for loading images and one for clearing
    // Button for loading image 1
    let mut image1_button = TextButton::new(
        screen_width() / 2.0 - 200.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Image 1", 
        BLUE, 
        GREEN, 
        20
    );
    image1_button.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Button for loading image 2
    let mut image2_button = TextButton::new(
        screen_width() / 2.0 - 60.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Image 2", 
        PURPLE, 
        PINK, 
        20
    );
    image2_button.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Button for clearing the image
    let mut clear_button = TextButton::new(
        screen_width() / 2.0 + 80.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Clear", 
        DARKPURPLE, 
        MAROON, 
        20
    );
    clear_button.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Create a label for instructions
    let mut instructions_label = Label::new(
        "Click a button to load or clear the image", 
        screen_width() / 2.0 - 200.0, 
        50.0, 
        24
    );

    instructions_label
        .with_colors(WHITE, Some(DARKGRAY))
        .with_round(8.0)
        .with_border(GRAY, 1.5)
        .with_alignment(objects::label::TextAlign::Center);

    loop {
        clear_background(LIGHTGRAY);

        // Draw the image
        image_obj.draw();

        // Draw the instructions label
        instructions_label.draw();

        // Handle image1 button
        if image1_button.click() {
            // Load image1 directly by passing the tuple
            image_obj.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());
        }

        // Handle image2 button
        if image2_button.click() {
            // Load image2 directly by passing the tuple
            image_obj.set_preload(texture_manager.get_preload("assets/image2.png").unwrap());
        }

        // Handle clear button
        if clear_button.click() {
            // Clear the image directly using the new clear() method
            image_obj.clear();
        }

        next_frame().await;
    }
}
