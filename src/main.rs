use hello_raylib::character::Character;
use raylib::{ffi::Vector2, prelude::*};

const W_SIZE: f32 = 640.0;
const H_SIZE: f32 = 480.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(W_SIZE as i32, H_SIZE as i32)
        .title("Hello, World")
        .build();

    let mut character = Character::new("resources/actors.png", (1.0, 0.0), &mut rl, &thread);

    rl.set_target_fps(60);

    let square_pos: Vector2 = Vector2 {
        x: W_SIZE / 2.0,
        y: H_SIZE / 2.0,
    };
    let square_size: Vector2 = Vector2 { x: 100.0, y: 100.0 };

    while !rl.window_should_close() {

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            character.frame_rec.y = 0.0;

            dbg!(&character.pos);
            character.pos.y = if character.pos.y + character.height + character.speed >= H_SIZE
            {
                H_SIZE - character.height
            } else {
                character.pos.y + character.speed
            };
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            character.frame_rec.y = 3.0 * character.height;

            dbg!(&character.pos);
            character.pos.y = if character.pos.y - character.speed <= 0.0 {
                0.0
            } else {
                character.pos.y - character.speed
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            character.frame_rec.y = 2.0 * character.height;

            dbg!(&character.pos);
            character.pos.x =
                if character.pos.x + character.width + character.speed >= W_SIZE {
                    W_SIZE - character.width
                } else {
                    character.pos.x + character.speed
                }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            character.frame_rec.y = 1.0 * character.height;

            dbg!(&character.pos);
            character.pos.x = if character.pos.x - character.speed <= 0.0 {
                0.0
            } else {
                character.pos.x - character.speed
            }
        }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_rectangle_v(square_pos, square_size, Color::WHITE);
        d.draw_texture_rec(
            &character.texture,
            character.frame_rec,
            character.pos,
            Color::WHITE,
        );
    }
}
