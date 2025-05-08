/*
By: Your Name Here
Date: 2025-05-03
Program Details: Image viewer with centralized texture preloading and name-based image selection
*/
mod modules {
    pub mod image_preload;
    pub mod still_image;
    pub mod label;
    pub mod txt_buttons;
}

use macroquad::prelude::*;
use modules::image_preload::TextureManager;
use modules::still_image::StillImage;
use modules::label::Label;
use modules::txt_buttons::TextButton;

fn window_conf() -> Conf {
    Conf {
        window_title: "Image Test".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create and initialize the texture manager
    let mut texture_manager = TextureManager::new();

    // Preload all textures at startup
    texture_manager.preload_all(&["assets/image1.png", "assets/image2.png"]).await;

    // Create a single image object
    let mut img = StillImage::new(
        "", 
        300.0, 
        200.0, 
        screen_width() / 2.0 - 150.0, 
        screen_height() / 2.0 - 150.0, 
        true, 
        1.0
    ).await;

    // Create buttons: two for loading specific images, one for cycling through images, and one for clearing
    // Button for loading image 1
    let mut btn_image1 = TextButton::new(
        screen_width() / 2.0 - 250.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Image 1", 
        BLUE, 
        GREEN, 
        20
    );
    btn_image1.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Button for loading image 2
    let mut btn_image2 = TextButton::new(
        screen_width() / 2.0 - 120.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Image 2", 
        PURPLE, 
        PINK, 
        20
    );
    btn_image2.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Button for clearing the image
    let mut btn_clear = TextButton::new(
        screen_width() / 2.0 + 180.0, 
        screen_height() / 2.0 + 100.0, 
        120.0, 
        50.0, 
        "Clear", 
        DARKPURPLE, 
        MAROON, 
        20
    );
    btn_clear.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Create the Exit button
    let mut btn_exit = TextButton::new(
        screen_width() / 2.0 + 310.0, // Adjust position as needed
        screen_height() / 2.0 + 100.0,
        120.0,
        50.0,
        "Exit",
        RED,
        DARKGRAY,
        20
    );
    btn_exit.with_round(10.0).with_border(WHITE, 2.0).with_text_color(WHITE).with_hover_text_color(YELLOW);

    // Create a label for instructions
    let mut lbl_instructions = Label::new(
        "Click a button to load or clear the image", 
        screen_width() / 2.0 - 200.0, 
        50.0, 
        24
    );

    lbl_instructions
        .with_colors(WHITE, Some(DARKGRAY))
        .with_round(8.0)
        .with_border(GRAY, 1.5)
        .with_fixed_size(500.0, 40.0)
        .with_alignment(modules::label::TextAlign::Center);

    loop {
        clear_background(LIGHTGRAY);

        // Draw the image
        img.draw();

        // Draw the instructions label
        lbl_instructions.draw();

        // Handle image1 button
        if btn_image1.click() {
            // Using the safer pattern matching approach
            if let Some(preloaded) = texture_manager.get_preload("assets/image1.png") {
                img.set_preload(preloaded);

            } else {
                // Handle the case where the image doesn't exist
                println!("Warning: Image assets/image1.png not found in texture manager");
                // Optionally, you could try to load it dynamically
                // texture_manager.preload("assets/image1.png").await;
            }
        }

        // Handle image2 button
        if btn_image2.click() {
            // Direct unwrap approach - simpler but will panic if image is missing
            // This approach assumes the image was preloaded and is definitely available
            img.set_preload(texture_manager.get_preload("assets/image2.png").unwrap());
        
        }

        // Handle clear button
        if btn_clear.click() {
            // Clear the image directly using the new clear() method
            img.clear();
        }

        // Draw the Exit button and handle exit
        if btn_exit.click() {
            break; // Exit the main loop and close the app
        }

        next_frame().await;
    }
}
