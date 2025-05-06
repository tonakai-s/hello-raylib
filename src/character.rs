use raylib::{
    RaylibHandle, RaylibThread,
    ffi::Vector2,
    math::Rectangle,
    texture::{Image, Texture2D},
};

pub struct Character {
    pub width: f32,
    pub height: f32,
    pub pos: Vector2,
    pub frame_rec: Rectangle,
    pub speed: f32,
    pub texture: Texture2D,
}

impl Character {
    pub fn new(
        sprite_path: &'static str,
        rec: (f32, f32),
        rl_handle: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Self {
        let texture = Character::load(sprite_path, rl_handle, rl_thread);
        let width = (texture.width / 12) as f32;
        let height = (texture.height / 8) as f32;

        let pos = Vector2 { x: 0.0, y: 0.0 };
        let frame_rec = Rectangle {
            x: rec.0,
            y: rec.1,
            width,
            height,
        };

        Character {
            width,
            height,
            pos,
            frame_rec,
            speed: 2.0,
            texture,
        }
    }

    fn load(
        sprite_path: &'static str,
        rl_handle: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Texture2D {
        let img = Image::load_image(sprite_path).expect("Failed to load image into RAM");

        let _ = rl_handle
            .load_texture(rl_thread, "resources/actors.png")
            .expect("Failed to load image into VRAM");

        rl_handle
            .load_texture_from_image(rl_thread, &img)
            .expect("Failed to load texture from image")
    }
}
