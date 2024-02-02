use std::time::Instant;

use game::Game;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::{WindowBuilder, WindowButtons},
    dpi::{LogicalSize, PhysicalPosition, LogicalPosition},// platform::windows::WindowBuilderExtWindows,
};
use winit_input_helper::WinitInputHelper;

use error_iter::ErrorIter as _;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};

extern crate nalgebra as na;
use na::Vector2;

pub mod renderer;
pub mod util;
pub mod game;

const WIDTH : u32 = 480;
const HEIGHT: u32 = WIDTH/2;//324;

const WIDTH_USIZE: usize = WIDTH as usize;
const HEIGHT_USIZE: usize = HEIGHT as usize;

const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;

#[derive(Debug, PartialEq)]
enum CursorMode {Free, Locked}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        //let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Raycasting :3")
            .with_title("Raycasting")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut cursor_mode = CursorMode::Free;

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut deltatime: f64 = 0.0;
    let mut lasttime = Instant::now();

    let mut g = Game::new();
    let mut render_map = false;

    let mut power = 0.0;

    let mut fov = 2.0;
    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::RedrawRequested => {
                    renderer::render_view(pixels.frame_mut(), &mut g, fov);
                    if render_map {
                        renderer::render_map(pixels.frame_mut(), &mut g, 10);
                    }

                    if let Err(err) = pixels.render() {
                        return log_error("pixels.render", err, &control_flow);
                    }
                }
                _ => ()
            }
        }
        if input.update(&event) {
            deltatime = lasttime.elapsed().as_secs_f64();
            lasttime = Instant::now();

            // Exiting
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                control_flow.exit();
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    return log_error("pixels.render", err, &control_flow);
                }
            }

            // Cursor modes
            // Unlock if window loses focus
            if !window.has_focus() && cursor_mode == CursorMode::Locked {
                window.set_cursor_grab(winit::window::CursorGrabMode::None);
                window.set_cursor_visible(true);
                cursor_mode = CursorMode::Free;
            }
            // Switch cursor modes when user presses Tab
            if input.key_pressed(KeyCode::Tab) {
                cursor_mode = match cursor_mode {
                    CursorMode::Free   => {
                        window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
                        window.set_cursor_visible(false);
                        CursorMode::Locked
                    }
                    CursorMode::Locked => {
                        window.set_cursor_grab(winit::window::CursorGrabMode::None);
                        window.set_cursor_visible(true);
                        CursorMode::Free
                    }
                }
            }
            // If the cursor should be locked, move it to the middle every frame
            if cursor_mode == CursorMode::Locked {
                window.set_cursor_position(LogicalPosition::new(window.inner_size().width/2, window.inner_size().height/2));
            }

            // Light tests
            if input.key_pressed(KeyCode::KeyR) {
                g.lights.push(game::light::Light::new(g.player.pos, power));
            }
            if input.key_pressed(KeyCode::KeyT) {
                g.lights.clear();
            }
            if input.key_pressed_os(KeyCode::KeyF) {
                power += 1.0;
                println!("POWER: {power:?}");
            }
            if input.key_pressed_os(KeyCode::KeyG) {
                power -= 1.0;
                println!("POWER: {power:?}");
            }

            // Breaking blocks
            if input.mouse_pressed(0) && cursor_mode == CursorMode::Locked {
                if let Some((cell, ..)) = util::raycast(&g, g.player.pos, g.player.dir, 4.5) {
                    if !(cell % g.map_width == 0 || cell % g.map_width == g.map_width-1 ||
                         cell / g.map_width == 0 || cell / g.map_width == g.map_height-1) {
                        g.map[cell] = 0;
                    }
                }
            }
            // Placing blocks
            // This is a bit wonky lol
            if input.mouse_pressed(1) && cursor_mode == CursorMode::Locked {
                if let Some((cell, pos, real_dist, side)) = util::raycast(&g, g.player.pos, g.player.dir, 4.5) {
                    // g.map[cell] = 0;
                    let mut c: usize = 0;
                    if side == util::RaycastSide::X {
                        c = pos.y as usize * g.map_width + (pos.x as usize).saturating_add_signed(-g.player.dir.x.signum() as isize);
                    } else {
                        c = (pos.y as usize).saturating_add_signed(-g.player.dir.y.signum() as isize) * g.map_width + pos.x as usize;
                    }
                    if !(c % g.map_width == 0 || c % g.map_width == g.map_width-1 ||
                         c / g.map_width == 0 || c / g.map_width == g.map_height-1) {
                       g.map[c] = 1;
                   }
                }
            }

            // Player controls            
            if input.key_pressed(KeyCode::KeyC) { render_map = !render_map; }
            if input.key_held(KeyCode::KeyQ) { fov = (fov - deltatime).max(0.01); }
            if input.key_held(KeyCode::KeyE) { fov += deltatime; }

            let mut mov = Vector2::new(0.0, 0.0);
            if input.key_held(KeyCode::KeyW) { mov.y += 1.0; }
            if input.key_held(KeyCode::KeyA) { mov.x -= 1.0; }
            if input.key_held(KeyCode::KeyS) { mov.y -= 1.0; }
            if input.key_held(KeyCode::KeyD) { mov.x += 1.0; }
            if mov.magnitude() != 0.0 { mov = mov.normalize(); }
            if input.key_held(KeyCode::ControlLeft) { mov *= 0.5; }

            g.player.step(mov * 8.0, deltatime);

            // Only rotate if the mouse is locked
            if cursor_mode == CursorMode::Locked {
                let mut r: f64 = 0.0;
                if input.key_held(KeyCode::ArrowLeft)  { r -= 1.0; }
                if input.key_held(KeyCode::ArrowRight) { r += 1.0; }
                r += input.mouse_diff().0 as f64 / 10.0;
                // window.set_cursor_position(LogicalPosition::new(WIDTH, HEIGHT));
                r *= 0.035;
                g.player.dir = na::Rotation2::new(r) * g.player.dir;
                if input.key_held(KeyCode::ArrowUp)   { g.player.pitch += 1.0; }
                if input.key_held(KeyCode::ArrowDown) { g.player.pitch -= 1.0; }

                g.player.pitch = (g.player.pitch + input.mouse_diff().1 as f64 / 2.0).clamp(-150.0, 150.0);
            }

            // Redraw
            window.request_redraw();
        }
    }).unwrap();
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E, control_flow: &EventLoopWindowTarget<()>) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
    control_flow.exit();
}
