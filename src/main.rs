use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::WindowBuilder,
    dpi::LogicalSize,
};
use winit_input_helper::WinitInputHelper;
use error_iter::ErrorIter as _;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use pixels_primitives;

const WIDTH : u32 = 576;
const HEIGHT: u32 = 324;
const GRID_SIZE: u32 = 12;

const MAP_WIDTH : usize = 48;
const MAP_HEIGHT: usize = 27;
static map: [usize; MAP_WIDTH*MAP_HEIGHT] = [
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,
    0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,
    0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
];

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Raycasting :3")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut deltatime: f64 = 0.0;
    let mut lasttime = Instant::now();

    let mut player_pos: [f64; 2] = [0.0, 0.0];
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::RedrawRequested => {
                    draw(pixels.frame_mut(), &player_pos, &mouse_pos);
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

            if let Some((x, y)) = input.cursor() {
                mouse_pos = [x.into(), y.into()];
            }

            if input.key_held(KeyCode::KeyW) { player_pos[1] -= deltatime * 10.0; }
            if input.key_held(KeyCode::KeyA) { player_pos[0] -= deltatime * 10.0; }
            if input.key_held(KeyCode::KeyS) { player_pos[1] += deltatime * 10.0; }
            if input.key_held(KeyCode::KeyD) { player_pos[0] += deltatime * 10.0; }

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

fn draw(screen: &mut [u8], player_pos: &[f64; 2], mouse_pos: &[f64; 2]) {
    for pix in screen.chunks_exact_mut(4) {
        pix.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF]);
    }
    // Draw grid
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
    // Slices of each line
    // let screen_slices: [[bool; WIDTH]; MAP_HEIGHT];
    // let col: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];

    // for i in 0..map.len() {
    //     if map[i] == 0 { continue; }
    //     let x = i % MAP_WIDTH;
    //     let y = i / MAP_WIDTH % MAP_HEIGHT;
    //     let screen_slc: [u8; 4*MAP_HEIGHT*GRID_SIZE as usize];
        
    //     for i in 0..GRID_SIZE {
        
    //     }

    //     for (i, pix) in screen.chunks_exact_mut(4).enumerate() {
    //         let y = i % WIDTH as usize;
    //     } 
    //     screen[i * 4 * GRID_SIZE as usize + y * (GRID_SIZE) as usize * MAP_WIDTH] = 0xFF;
    //     // pixels_primitives::square_filled(screen, WIDTH as i32,
    //     //     (x as u32*GRID_SIZE)as f64+(GRID_SIZE as f64/2.0),
    //     //     (y as u32*GRID_SIZE)as f64+(GRID_SIZE as f64/2.0),
    //     //     GRID_SIZE.into(), &[0x00, 0x00, 0xFF, 0xFF]);
    // }
    // Draw player
    pixels_primitives::circle(screen, WIDTH as i32,
        player_pos[0] * GRID_SIZE as f64, player_pos[1] * GRID_SIZE as f64, 5.0, 1.0, &[0x00, 0xFF, 0x00, 0xFF]);
    // Draw mouse
    pixels_primitives::circle(screen, WIDTH as i32,
        mouse_pos[0], mouse_pos[1], 5.0, 1.0, &[0xFF, 0x00, 0x00, 0xFF]);
    pixels_primitives::line(screen, WIDTH as i32,
        player_pos[0] * GRID_SIZE as f64, player_pos[1] * GRID_SIZE as f64,
        mouse_pos[0], mouse_pos[1],
        &[0xAA, 0xAA, 0xAA, 0xFF]);
    // for (i, pix) in screen.chunks_exact_mut(4).enumerate() {
    //     let mut color = if i % WIDTH as usize % GRID_SIZE == 0 || i / WIDTH as usize % GRID_SIZE == 0 {
    //         [0x22, 0x22, 0x22, 0xFF]
    //     } else {
    //         [0x55, 0x55, 0x55, 0xFF]
    //     };

    //     color = match map[(i / MAP_HEIGHT + i / GRID_SIZE) % map.len()] {
    //         1 => [0x00, 0x00, 0xFF, 0xFF],
    //         _ => color,
    //     };        
    //     pix.copy_from_slice(&color)
    // }
}