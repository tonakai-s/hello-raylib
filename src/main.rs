use hello_raylib::character::Character;
use raylib::prelude::*;

const W_SIZE: f32 = 640.0;
const H_SIZE: f32 = 480.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(W_SIZE as i32, H_SIZE as i32)
        .title("Hello, World")
        .build();

    let mut character = Character::new("resources/actors.png", (1.0, 0.0), &mut rl, &thread);

    rl.set_target_fps(60);

    let square = Rectangle {
        x: W_SIZE / 2.0,
        y: H_SIZE / 2.0,
        width: 100.0,
        height: 100.0,
    };

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            character.frame_rec.y = 0.0;

            character.area.y = if character.area.y + character.area.height + character.speed >= H_SIZE
            {
                H_SIZE - character.area.height
            } else {
                character.area.y + character.speed
            };
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            character.frame_rec.y = 3.0 * character.area.height;

            // dbg!(&character.pos);
            character.area.y = if character.area.y - character.speed <= 0.0 {
                0.0
            } else {
                character.area.y - character.speed
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            character.frame_rec.y = 2.0 * character.area.height;

            // dbg!(&character.pos);
            character.area.x = if character.area.x + character.area.width + character.speed >= W_SIZE
            {
                W_SIZE - character.area.width
            } else {
                character.area.x + character.speed
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            character.frame_rec.y = 1.0 * character.area.height;

            // dbg!(&character.pos);
            character.area.x = if character.area.x - character.speed <= 0.0 {
                0.0
            } else {
                character.area.x - character.speed
            }
        }

        let collision = character.area.get_collision_rec(&square);
        if collision.is_some() && collision.unwrap().x == 0.0 {
            dbg!(&character.area);
            dbg!(&collision);
        }
        // if character_area.get_collision_rec(&square).is_some() {
        //     println!("Colliding!");
        // }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_rectangle_rec(square, Color::WHITE);
        d.draw_texture_rec(
            &character.texture,
            character.frame_rec,
            character.pos,
            Color::WHITE,
        );
    }
}
