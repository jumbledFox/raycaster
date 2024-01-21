use std::{time::Instant, f64::consts::PI};

use game::{Game, player::Player};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::{WindowBuilder, WindowButtons},
    dpi::{LogicalSize, PhysicalPosition, LogicalPosition}, platform::windows::WindowBuilderExtWindows,
};
use winit_input_helper::WinitInputHelper;

use error_iter::ErrorIter as _;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use pixels_primitives;

extern crate nalgebra as na;
use na::Vector2;

use lerp::Lerp;

use rand::Rng;

pub mod renderer;
pub mod util;
pub mod game;

const WIDTH : u32 = 480;
const HEIGHT: u32 = WIDTH/2;//324;
const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;
const GRID_SIZE: u32 = 12;
const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

const MAP_WIDTH : usize = 48;
const MAP_HEIGHT: usize = 27;
static map: [usize; MAP_WIDTH*MAP_HEIGHT] = [
    7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,
    7,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,7,0,0,0,0,0,0,7,0,0,0,0,0,0,0,7,2,
    7,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,0,7,0,7,0,7,7,7,0,0,0,0,0,0,0,7,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,7,7,0,7,7,0,0,0,0,0,0,7,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,3,0,0,0,0,0,0,5,5,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,6,7,8,0,0,0,0,3,0,0,0,0,0,0,5,0,0,5,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    7,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,2,
    7,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,2,
    7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,
];

#[derive(Debug, PartialEq)]
enum CursorMode {Free, Locked}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Raycasting :3")
            // .with_title("Raycasting")
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

    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::RedrawRequested => {
                    renderer::render_view(&mut g, pixels.frame_mut());
                    //draw(pixels.frame_mut(), &player_pos, &player_dir, &cam_plane, &mouse_pos, &hit_pos, &check_points);

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
            if !window.has_focus() {
                window.set_cursor_grab(winit::window::CursorGrabMode::None);
                window.set_cursor_visible(true);
                cursor_mode = CursorMode::Free;
            }
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
            if cursor_mode == CursorMode::Locked {
                window.set_cursor_position(LogicalPosition::new(window.inner_size().width/2, window.inner_size().height/2));
            }
            // if let Some(p) = input.cursor() {
            //     mouse_pos = match pixels.window_pos_to_pixel(p) {
            //         Ok(p_pos)  => Vector2::new(p_pos.0 as f64, p_pos.1 as f64),
            //         Err(p_pos) => Vector2::new(p_pos.0 as f64, p_pos.1 as f64),
            //     };
            // }

            if input.mouse_pressed(0) && cursor_mode == CursorMode::Locked {
                if let Some((cell, ..)) = util::raycast(&g, g.player.pos, g.player.dir, 4.5) {
                    g.map[cell] = 0;
                }
            }
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
                    g.map[c] = 1;
                }
            }

            // if input.key_held(KeyCode::KeyQ) { g.player.head_height += 8.0 * deltatime; }
            // if input.key_held(KeyCode::KeyE) { g.player.head_height -= 8.0 * deltatime; }
            if input.key_held(KeyCode::Space) && !g.player.jumping { g.player.jumping = true; }

            // TODO: Make headbob better
            // if g.player.vel.magnitude() < 0.5 { g.player.head_bob_amount = g.player.head_bob_amount.lerp(0.0, (deltatime * 1.0).min(1.0)); }
            // println!("{:?} {:?}", g.player.head_bob_amount, g.player.vel.magnitude());
            if g.player.jumping {
                g.player.jump_amount += deltatime * 5.0;
                // g.player.head_bob_amount = g.player.head_bob_amount.lerp(0.0, deltatime * 10.0);
                g.player.head_height = g.player.jump_amount.cos() * 35.0;
                if g.player.jump_amount > PI/2.0 {
                    g.player.jumping = false;
                    g.player.jump_amount = -PI/2.0;
                }
            }

            let mut mov = Vector2::new(0.0, 0.0);
            if input.key_held(KeyCode::KeyW) { mov.y += 1.0; }
            if input.key_held(KeyCode::KeyA) { mov.x -= 1.0; }
            if input.key_held(KeyCode::KeyS) { mov.y -= 1.0; }
            if input.key_held(KeyCode::KeyD) { mov.x += 1.0; }
            if mov.magnitude() != 0.0 { mov = mov.normalize(); }
            if input.key_held(KeyCode::ControlLeft) { mov *= 0.5; }
            // if g.player.jumping { mov *= 0.5; }
            
            g.player.step(mov * 8.0, deltatime);
            // g.player.pos.x = g.player.pos.x.rem_euclid(g.map_width as f64);
            // g.player.pos.y = g.player.pos.y.rem_euclid(g.map_height as f64);
            // if let Some(c) = g.map.get_mut(g.player.pos.y as usize * g.map_width + g.player.pos.x as usize) {
            //     *c = 0;
            // }
            // Only rotate if the mouse is locked
            if cursor_mode == CursorMode::Locked {
                let mut r: f64 = 0.0;
                if input.key_held(KeyCode::ArrowLeft)  { r -= 1.0; }
                if input.key_held(KeyCode::ArrowRight) { r += 1.0; }
                r += input.mouse_diff().0 as f64 / 10.0;
                // window.set_cursor_position(LogicalPosition::new(WIDTH, HEIGHT));
                r *= 0.035;
                g.player.dir = na::Rotation2::new(r) * g.player.dir;
                g.player.pitch = (g.player.pitch + input.mouse_diff().1 as f64 / 2.0).clamp(-150.0, 150.0);
            }
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

fn draw(screen: &mut [u8], player_pos: &Vector2<f64>, player_dir: &Vector2<f64>, cam_plane: &Vector2<f64>,
    mouse_pos: &Vector2<f64>, hit_pos: &Option<Vector2<f64>>, check_points: &Vec<Vector2<f64>>) {
    // Clear screen
    //screen.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF].repeat(screen.len()/4));

    // Draw grid
    // This works for now, even though it's a little slow, so I turn a blind eye and pretend it's fast
    for x in 0..WIDTH/GRID_SIZE {
        let x_pos = (x*GRID_SIZE).into();
        pixels_primitives::line(screen, WIDTH as i32, x_pos, 0.0, x_pos, HEIGHT.into(),
            &[0x22, 0x22, 0x22, 0xFF]);
    }
    for y in 0..HEIGHT/GRID_SIZE {
        let y_pos = (y*GRID_SIZE).into();
        pixels_primitives::line(screen, WIDTH as i32, 0.0, y_pos, WIDTH.into(), y_pos,
            &[0x22, 0x22, 0x22, 0xFF]);
    }
    // Draw map
    for (i, m) in map.iter().enumerate() {
        if *m == 0 { continue; }
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH;
        draw_rect(screen,
            x*GRID_SIZE as usize,                      y*GRID_SIZE as usize,
            x*GRID_SIZE as usize + GRID_SIZE as usize, y*GRID_SIZE as usize + GRID_SIZE as usize,
            &get_col(*m));
    }
    // Draw line from player to mouse
    draw_line(screen,
        player_pos * GRID_SIZE_F64, *mouse_pos,
        &[0x77, 0x00, 0x00, 0xFF]);
    // Draw ray
    if let Some(hit_pos) = hit_pos {
        // Draw hit point
        pixels_primitives::circle(screen, WIDTH as i32,
            hit_pos.x * GRID_SIZE_F64, hit_pos.y * GRID_SIZE_F64, 5.0, 1.0, &[0xAA, 0xAA, 0xAA, 0xFF]);
        // Draw line
        draw_line(screen,
            hit_pos * GRID_SIZE_F64, player_pos * GRID_SIZE_F64,
            &[0x77, 0x77, 0x77, 0xFF]);
    }
    for c in check_points {
        pixels_primitives::circle_filled(screen, WIDTH as i32,
            c.x * GRID_SIZE_F64, c.y * GRID_SIZE_F64, 2.0, &[0x92, 0x92, 0x92, 0xFF]);
    }

    // Draw player
    pixels_primitives::circle(screen, WIDTH as i32,
        player_pos.x * GRID_SIZE_F64, player_pos.y * GRID_SIZE_F64, 5.0, 1.0, &[0x00, 0xFF, 0x00, 0xFF]);
    // Draw mouse
    pixels_primitives::circle(screen, WIDTH as i32,
        mouse_pos.x, mouse_pos.y, 5.0, 1.0, &[0xFF, 0x00, 0x00, 0xFF]);
    // Draw direction and cam plane
    draw_line(screen,
        player_pos * GRID_SIZE_F64,
        (player_pos + player_dir) * GRID_SIZE_F64,
        &[0xFF, 0x00, 0xFF, 0xFF]);
    draw_line(screen,
        (player_pos + player_dir - cam_plane) * GRID_SIZE_F64,
        (player_pos + player_dir + cam_plane) * GRID_SIZE_F64,
        &[0x00, 0xFF, 0xFF, 0xFF]);
}

// A neater way of invoking pixels_primitves functions
fn draw_line(screen: &mut [u8], pos_a: Vector2<f64>, pos_b: Vector2<f64>, col: &[u8; 4]) {
    pixels_primitives::line(screen, WIDTH as i32, pos_a.x, pos_a.y, pos_b.x, pos_b.y, col);
}

// My own draw_rect function, doesn't do bounds checking but like,, just don't be stupid?? 
fn draw_rect(screen: &mut [u8], x_0: usize, y_0: usize, x_1: usize, y_1: usize, col: &[u8; 4]) {
    for y in y_0..y_1 {
        screen[(x_0+(y)*WIDTH as usize) * 4..(x_1+(y)*WIDTH as usize) * 4].copy_from_slice(&col.repeat(x_1-x_0));
    }
}

fn get_col(c: usize) -> [u8;4] {
    match c {
        2 => [0xFF, 0x00, 0x00, 0xFF],
        3 => [0xFF, 0xAA, 0x00, 0xFF],
        4 => [0xFF, 0xFF, 0x00, 0xFF],
        5 => [0x00, 0xFF, 0x00, 0xFF],
        6 => [0x00, 0xFF, 0xFF, 0xFF],
        7 => [0x00, 0x00, 0xFF, 0xFF],
        8 => [0xFF, 0x00, 0xFF, 0xFF],
        _ => [0xFF, 0xFF, 0xFF, 0xFF],
    }
}