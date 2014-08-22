#![feature(globs, macro_rules)]

extern crate debug;
extern crate piston;
extern crate sdl2_game_window;
extern crate gl;
extern crate hgl;
extern crate image;
extern crate libc;
extern crate cgmath;

use sdl2_game_window::GameWindowSDL2 as Window;
use piston::input;
use piston::cam;
use piston::{
    AssetStore,
    GameIterator,
    GameIteratorSettings,
    GameWindow,
    GameWindowSettings,
    Input,
    Render,
    Update,
};

use array::*;
use texture::Texture;

pub mod array;
pub mod cube;
pub mod shader;
pub mod texture;
pub mod vecmath;

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_3,
        GameWindowSettings {
            title: "Hematite".to_string(),
            size: [854, 480],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let asset_store = AssetStore::from_folder("../assets");

    // Load texture.
    let texture = asset_store.path("texture.png").unwrap();
    let texture = Texture::from_path(&texture).unwrap();

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let shader = shader::Shader::new();

    shader.set_projection(cam::CameraPerspective {
        fov: 70.0,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: {
            let (w, h) = window.get_size();
            (w as f32) / (h as f32)
        }
    }.projection());

    let mut camera = cam::Camera::new(0.5, 0.5, 4.0);
    let mut fps_controller_settings = cam::FPSControllerSettings::default();
    fps_controller_settings.speed_horizontal = 8.0;
    fps_controller_settings.speed_vertical = 4.0;
    let mut fps_controller = cam::FPSController::new(fps_controller_settings);
    camera.set_yaw_pitch(fps_controller.yaw, fps_controller.pitch);

    let mut capture_cursor = false;
    println!("Press C to capture mouse");

    let buffer = shader::Buffer::new();
    let mut events = GameIterator::new(&mut window, &game_iter_settings);
    for e in events {
        match e {
            Render(_args) => {
                let mut tri = vec![];
                for face in cube::FaceIterator::new() {
                    let xyz = face.vertices([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
                    let [u0, v1, u1, v0] = [0.0, 0.75, 0.25, 1.0];
                    let uv = [
                        [u1, v0],
                        [u0, v0],
                        [u0, v1],
                        [u1, v1]
                    ];
                    let v = [
                        (xyz[0], uv[0], [1.0, 0.0, 0.0]),
                        (xyz[1], uv[1], [0.0, 1.0, 0.0]),
                        (xyz[2], uv[2], [0.0, 0.0, 1.0]),
                        (xyz[3], uv[3], [1.0, 0.0, 1.0])
                    ].map(|(xyz, uv, rgb)| shader::Vertex { xyz: xyz, uv: uv, rgb: rgb });

                    // Split the clockwise quad into two clockwise triangles.
                    tri.push([v[0], v[1], v[2]]);
                    tri.push([v[2], v[3], v[0]]);
                }
                buffer.load_data(tri.as_slice());

                shader.set_view(camera.orthogonal());
                shader.bind();
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                shader.render(&buffer);
            }
            Input(input::KeyPress { key: input::keyboard::C }) => {
                println!("Turned cursor capture {}", if capture_cursor { "off" } else { "on" });
                capture_cursor = !capture_cursor;

                events.game_window.capture_cursor(capture_cursor);
            }
            Input(input::MouseRelativeMove { .. }) => {
                if !capture_cursor {
                    // Don't send the mouse event to the FPS controller.
                    continue;
                }
            }
            _ => {}
        }

        // Camera controller.
        match e {
            Input(ref args) => fps_controller.input(args, &mut camera),
            Update(piston::UpdateArgs { dt, .. }) => fps_controller.update(dt, &mut camera),
            _ => {}
        }
    }
}
