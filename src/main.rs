use std::time::Instant;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget},
    keyboard::KeyCode,
    window::WindowBuilder,
    dpi::{LogicalSize, PhysicalPosition},
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
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,
    1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,1,0,1,0,1,1,1,0,0,0,0,0,0,0,1,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,
    1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
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
    let mut player_dir = [1.0, 0.0];
    //let mut cam_plane: [f64; 2] = [0.0, -1.0];
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];
    let mut check_points: Vec<[f64; 2]> = Vec::with_capacity(10);
    let mut hit_pos: Option<[f64; 2]> = None;

    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::RedrawRequested => {
                    draw(pixels.frame_mut(), &player_pos, &mouse_pos, &hit_pos, &check_points);
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

            if let Some(p) = input.cursor() {
                mouse_pos = match pixels.window_pos_to_pixel(p) {
                    Ok(p_pos)  => [p_pos.0 as f64, p_pos.1 as f64],
                    Err(p_pos) => [p_pos.0 as f64, p_pos.1 as f64],
                };
            }

            if input.key_held(KeyCode::KeyW) { player_pos[1] -= deltatime * 10.0; }
            if input.key_held(KeyCode::KeyA) { player_pos[0] -= deltatime * 10.0; }
            if input.key_held(KeyCode::KeyS) { player_pos[1] += deltatime * 10.0; }
            if input.key_held(KeyCode::KeyD) { player_pos[0] += deltatime * 10.0; }

            // Player diretion = 
            player_dir = [(mouse_pos[0]/GRID_SIZE as f64)-player_pos[0], (mouse_pos[1]/GRID_SIZE as f64)-player_pos[1]];
            let len = f64::sqrt(player_dir[0].powi(2) + player_dir[1].powi(2));
            //player_dir = [player_dir[0] / len / 5.0, player_dir[1] / len / 5.0];
            player_dir = [player_dir[0] / len, player_dir[1] / len];
            // Calculate ray
            let ray_dir: [f64; 2] = player_dir;

            let mut ray_pos = player_pos;
            check_points.clear();
            hit_pos = None;
            //hit_pos = [ray_pos[0] + ray_dir[0], ray_pos[1] + ray_dir[1]];
            for i in 0..1000 {
                ray_pos = [ray_pos[0] + ray_dir[0], ray_pos[1] + ray_dir[1]];
                check_points.push(ray_pos);
                if ray_pos[0] < 0.0 || ray_pos[0].ceil() > MAP_WIDTH  as f64 || 
                   ray_pos[1] < 0.0 || ray_pos[1].ceil() > MAP_HEIGHT as f64 {
                    continue;
                }
                let map_x = ray_pos[0].floor() as usize;
                let map_y = ray_pos[1].floor() as usize;
                if map[map_x + map_y * MAP_WIDTH] != 0 {
                    println!("dist: {:?}", f64::sqrt((ray_pos[0]-player_pos[0]).powi(2)+(ray_pos[1]-player_pos[1]).powi(2)));
                    // hit_pos = [
                    //     map_x as f64,
                    //     map_y as f64,
                    // ];
                    hit_pos = Some(ray_pos);
                    break;
                }
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

fn draw(screen: &mut [u8], player_pos: &[f64; 2], mouse_pos: &[f64; 2], hit_pos: &Option<[f64; 2]>, check_points: &Vec<[f64; 2]>) {
    // Clear screen
    screen.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF].repeat(screen.len()/4));

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
    let col: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
    for (i, m) in map.iter().enumerate() {
        if *m == 0 { continue; }
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH;
        draw_rect(screen, WIDTH as usize,
            x*GRID_SIZE as usize,                      y*GRID_SIZE as usize,
            x*GRID_SIZE as usize + GRID_SIZE as usize, y*GRID_SIZE as usize + GRID_SIZE as usize,
            &col);
        // This code also works but has lots of floats, my version is better, albeit without bounds checking.. haha
        // pixels_primitives::square_filled(screen, WIDTH as i32,
        //     (x as u32*GRID_SIZE)as f64+(GRID_SIZE as f64/2.0),
        //     (y as u32*GRID_SIZE)as f64+(GRID_SIZE as f64/2.0),
        //     GRID_SIZE.into(), &[0x00, 0x00, 0xFF, 0xFF]);
    }
    for i in 0..map.len() {
        if map[i] == 0 { continue; }
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH % MAP_HEIGHT;
    }
    // Draw player
    pixels_primitives::circle(screen, WIDTH as i32,
        player_pos[0] * GRID_SIZE as f64, player_pos[1] * GRID_SIZE as f64, 5.0, 1.0, &[0x00, 0xFF, 0x00, 0xFF]);
    // Draw mouse
    pixels_primitives::circle(screen, WIDTH as i32,
        mouse_pos[0], mouse_pos[1], 5.0, 1.0, &[0xFF, 0x00, 0x00, 0xFF]);
    // Draw line
    // pixels_primitives::line(screen, WIDTH as i32,
    //     player_pos[0] * GRID_SIZE as f64, player_pos[1] * GRID_SIZE as f64,
    //     mouse_pos[0], mouse_pos[1],
    //     &[0x77, 0x77, 0x77, 0xFF]);
    if let Some(hit_pos) = hit_pos {
        // Draw hit point
        pixels_primitives::circle(screen, WIDTH as i32,
            hit_pos[0] * GRID_SIZE as f64, hit_pos[1] * GRID_SIZE as f64, 5.0, 1.0, &[0xAA, 0xAA, 0xAA, 0xFF]);
        // Draw line
        pixels_primitives::line(screen, WIDTH as i32,
            hit_pos[0] * GRID_SIZE as f64, hit_pos[1] * GRID_SIZE as f64,
            player_pos[0] * GRID_SIZE as f64, player_pos[1] * GRID_SIZE as f64,
            &[0x77, 0x77, 0x77, 0xFF]);
    }
    for c in check_points {
        pixels_primitives::circle_filled(screen, WIDTH as i32,
            c[0] * GRID_SIZE as f64, c[1] * GRID_SIZE as f64, 2.0, &[0x92, 0x92, 0x92, 0xFF]);
    }

}

fn draw_rect(screen: &mut [u8], width: usize, x_0: usize, y_0: usize, x_1: usize, y_1: usize, col: &[u8; 4]) {
    for y in y_0..y_1 {
        screen[(x_0+(y)*width) * 4..(x_1+(y)*width) * 4].copy_from_slice(&col.repeat(x_1-x_0));
    }
}
