use raylib::{ffi::Vector2, prelude::*};

const W_SIZE: f32 = 640.0;
const H_SIZE: f32 = 480.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(W_SIZE as i32, H_SIZE as i32)
        .title("Hello, World")
        .build();

    let wizard = Image::load_image("resources/actors.png").expect("Failed to load image into RAM");
    let _ = rl
        .load_texture(&thread, "resources/actors.png")
        .expect("Failed to load image into VRAM");
    let wizard_texture = rl
        .load_texture_from_image(&thread, &wizard)
        .expect("Failed to load texture from image");
    let mut wizard_pos: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let wizard_width = (wizard_texture.width / 12) as f32;
    let wizard_height = (wizard_texture.height / 8) as f32;

    let mut frame_rec: Rectangle = Rectangle { x: 1.0 * wizard_width, y: 0.0, width: wizard_width, height: wizard_height };

    rl.set_target_fps(60);

    let mov_length = 2.0;
    let mut square_pos: Vector2 = Vector2 {
        x: W_SIZE / 2.0,
        y: H_SIZE / 2.0,
    };
    let square_size: Vector2 = Vector2 { x: 100.0, y: 100.0 };

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            frame_rec.y = 0.0;

            dbg!(&wizard_pos);
            wizard_pos.y = if wizard_pos.y + wizard_height + mov_length >= H_SIZE {
                H_SIZE - wizard_height
            } else {
                wizard_pos.y + mov_length
            };
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            frame_rec.y = 3.0 * wizard_height;

            dbg!(&wizard_pos);
            wizard_pos.y = if wizard_pos.y - mov_length <= 0.0 {
                0.0
            } else {
                wizard_pos.y - mov_length
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            frame_rec.y = 2.0 * wizard_height;

            dbg!(&wizard_pos);
            wizard_pos.x = if wizard_pos.x + wizard_width + mov_length >= W_SIZE {
                W_SIZE - wizard_width
            } else {
                wizard_pos.x + mov_length
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            frame_rec.y = 1.0 * wizard_height;

            dbg!(&wizard_pos);
            wizard_pos.x = if wizard_pos.x - mov_length <= 0.0 {
                0.0
            } else {
                wizard_pos.x - mov_length
            }
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_rectangle_v(square_pos, square_size, Color::WHITE);
        d.draw_texture_rec(&wizard_texture, frame_rec, wizard_pos, Color::WHITE);
    }
}
