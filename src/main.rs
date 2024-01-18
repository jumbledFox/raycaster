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

extern crate nalgebra as na;
use na::Vector2;

const WIDTH : u32 = 576;
const HEIGHT: u32 = 324;
const GRID_SIZE: u32 = 12;
const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

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

    //let mut player_pos = Vector2::<f64>::new(0.0, 0.0);
    let mut player_pos = Vector2::new(1.0, 1.0);
    let mut player_dir = Vector2::new(1.0, 0.0);
    let mut cam_plane  = Vector2::new(0.0, -1.0);
    let mut mouse_pos  = Vector2::new(0.0, 0.0);
    let mut check_points: Vec<Vector2<f64>> = Vec::with_capacity(10);
    let mut hit_pos: Option<Vector2<f64>> = None;

    event_loop.run(move |event, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            match event {
                WindowEvent::RedrawRequested => {
                    draw(pixels.frame_mut(), &player_pos, &player_dir, &cam_plane, &mouse_pos, &hit_pos, &check_points);
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
                    Ok(p_pos)  => Vector2::new(p_pos.0 as f64, p_pos.1 as f64),
                    Err(p_pos) => Vector2::new(p_pos.0 as f64, p_pos.1 as f64),
                };
            }

            let mut mov = Vector2::new(0.0, 0.0);
            if input.key_held(KeyCode::KeyW) { mov.y -= 1.0; }
            if input.key_held(KeyCode::KeyA) { mov.x -= 1.0; }
            if input.key_held(KeyCode::KeyS) { mov.y += 1.0; }
            if input.key_held(KeyCode::KeyD) { mov.x += 1.0; }
            if mov.magnitude() != 0.0 { player_pos += mov.normalize() * deltatime * 10.0; }
            
            // if input.key_held(KeyCode::KeyW) {  player_pos[1] += deltatime * 10.0 * player_dir[1];
            //                                     player_pos[0] += deltatime * 10.0 * player_dir[0]; }
            // if input.key_held(KeyCode::KeyS) {  player_pos[1] -= deltatime * 10.0 * player_dir[1];
            //                                     player_pos[0] -= deltatime * 10.0 * player_dir[0]; }
            // if input.key_held(KeyCode::KeyA) {  player_pos[1] += deltatime * 10.0 * player_dir[0];
            //                                     player_pos[0] -= deltatime * 10.0 * player_dir[1]; }
            
            // ?? what is this comment lol
            // Player diretion = 
            //player_dir = [(mouse_pos[0]/GRID_SIZE as f64)-player_pos[0], (mouse_pos[1]/GRID_SIZE as f64)-player_pos[1]];
            player_dir = ((mouse_pos/GRID_SIZE as f64) - player_pos).normalize();
            // let len = f64::sqrt(player_dir[0].powi(2) + player_dir[1].powi(2));
            // //player_dir = [player_dir[0] / len / 5.0, player_dir[1] / len / 5.0];
            // player_dir = [player_dir[0] / len * 0.5, player_dir[1] / len * 0.5];

            //cam_plane = [player_dir[1], -player_dir[0]];
            cam_plane = Vector2::new(player_dir.y, -player_dir.x);

            struct Ray {
                pos: Vector2<f64>,
                dir: Vector2<f64>,
            }
            let mut ray = Ray { pos: player_pos, dir: player_dir };
            
            check_points.clear();
            hit_pos = None;
            
            // TODO: do algorithm, make classes
            for i in 0..100 {
                // Calculate how far the ray should move and move it.
                let mov = Vector2::new(1.0, ray.dir.y/ray.dir.x);
                let mov = Vector2::new(ray.dir.x/ray.dir.y, 1.0);
                ray.pos += mov;
                //ray_pos = [ray_pos[0] + mov[0], ray_pos[1] + mov[1]];
                //ray_pos = [ray_pos[0] + ray_dir[0], ray_pos[1] + ray_dir[1]];
                //ray_pos = [ray_pos[0] + ray_dir[0], ray_pos[1] + ray_dir[1]];


                check_points.push(ray.pos);
                if  ray.pos.x < 0.0 || ray.pos.x.ceil() > MAP_WIDTH  as f64 || 
                    ray.pos.y < 0.0 || ray.pos.y.ceil() > MAP_HEIGHT as f64 {
                    continue;
                }
                let map_x = ray.pos.x.floor() as usize;
                let map_y = ray.pos.y.floor() as usize;
                if map[map_x + map_y * MAP_WIDTH] != 0 {
                    //println!("dist: {:?}", f64::sqrt((ray_pos[0]-player_pos[0]).powi(2)+(ray_pos[1]-player_pos[1]).powi(2)));
                    hit_pos = Some(ray.pos);
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

fn draw(screen: &mut [u8], player_pos: &Vector2<f64>, player_dir: &Vector2<f64>, cam_plane: &Vector2<f64>,
    mouse_pos: &Vector2<f64>, hit_pos: &Option<Vector2<f64>>, check_points: &Vec<Vector2<f64>>) {
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
        draw_rect(screen,
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
    draw_line(screen,
        player_pos * GRID_SIZE_F64, *mouse_pos,
        &[0x77, 0x00, 0x00, 0xFF]);
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
