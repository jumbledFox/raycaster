use std::{f64::consts::PI, ops::Add};

use crate::{WIDTH, HEIGHT, WIDTH_USIZE, HEIGHT_USIZE, na, Vector2, util::{self, RaycastSide}, game::{Game, player}, ASPECT_RATIO};

use na::coordinates::X;
use pixels_primitives;

const GRID_SIZE: u32 = 12;
const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

pub fn render_view(screen: &mut [u8], game: &mut Game, fov: f64) {
    game.player.cam_plane = Vector2::new(-game.player.dir.y, game.player.dir.x) * fov;

    // floor and ceiling
    let middle = (((HEIGHT/2) as f64 - game.player.pitch) as usize).min(HEIGHT as usize-1);
    draw_rect(screen, 0, 0,      WIDTH as usize, middle,            &[ 26,  28,  44, 0xFF]);
    draw_rect(screen, 0, middle, WIDTH as usize, HEIGHT as usize,   &[ 51,  60,  87, 0xFF]);

    // TODO: make it so no-matter the aspect ratio, the map is always cubes
    // for i in 0..100 {
    for w in 0..WIDTH {
        // if w % 3 != 0 {continue;}

        let ray_direction = game.player.dir + (game.player.cam_plane * (w as f64 / WIDTH as f64 * 2.0 - 1.0));
        let raycast_result = util::raycast(&game, game.player.pos, ray_direction, 500.0);
        if let Some((cell, hit_pos, distance, side)) = raycast_result {
            // Calculating heights
            let head_height = (game.player.head_bob_amount.sin() / distance) * 10.0;

            // let h = HEIGHT as f64;
            // let lineheight = (h / distance);// * (1.0/ASPECT_RATIO);
            // let mut draw_start = -lineheight / 2.0 + h / 2.0 + head_height - game.player.pitch;
            // if draw_start < 0.0 { draw_start = 0.0 };
            // let mut draw_end = lineheight / 2.0 + h / 2.0 + head_height - game.player.pitch;
            // if draw_end > h { draw_end = h };

            let h = HEIGHT as isize;
            let lineheight = (h as f64 / (distance*fov)) as isize;// * (1.0/ASPECT_RATIO);
            let draw_start = -lineheight / 2 + h / 2 + (head_height - game.player.pitch) as isize;
            // if draw_start < 0 { draw_start = 0 };
            let draw_end   =  lineheight / 2 + h / 2 + (head_height - game.player.pitch) as isize;
            // if draw_end > h { draw_end = h };

            // Texture shiz
            // How far along the texture is
            let along = match side {
                util::RaycastSide::X => hit_pos.y,
                util::RaycastSide::Y => hit_pos.x,
            }.rem_euclid(1.0);
            //if w != WIDTH / 2 {println!("{:?}", along)}
            //let mut color = game.texture[(along * game.texture_size.0 as f64) as usize + game.texture_size.0*5];
            //println!("{:?}", along);

            // Color stuff
            let mut color = get_col(game.map[cell]);
            if side == util::RaycastSide::Y {
                color[0] = (color[0] as f32 * 0.7) as u8;
                color[1] = (color[1] as f32 * 0.7) as u8;
                color[2] = (color[2] as f32 * 0.7) as u8;
            }
            // let real_dist = distance / ray_direction.angle(&game.player.dir).cos();

            let mut light_level = 0.0;
            for light in &game.lights {
                // calculate distance from light to hit position and add it to the light level
                // nalgebra::distance(hit_pos, light.pos);
            }

            // color[0] = (color[0] as f64 / (real_dist.max(1.0) / 3.0).max(1.0)) as u8;
            // color[1] = (color[1] as f64 / (real_dist.max(1.0) / 3.0).max(1.0)) as u8;
            // color[2] = (color[2] as f64 / (real_dist.max(1.0) / 3.0).max(1.0)) as u8;

            // draw_line(screen, Vector2::new(w as f64, draw_start), Vector2::new(w as f64, draw_end), &color);
            draw_slice(screen, game, w as usize, along, draw_start, draw_end, &color);

            if w == WIDTH / 2 { game.player.mid_ray_dist = distance }
        }
    }
    // Draw 'crosshair'
    draw_rect(screen, (WIDTH/2 - 2) as usize, (HEIGHT/2 - 2) as usize, (WIDTH/2 + 2) as usize, (HEIGHT/2 + 2) as usize, &[0xFF, 0xAA, 0x00, 0xFF]);
}

// TODO:
// Draws a slice of a raycast
fn draw_slice(screen: &mut [u8], game: &Game, w: usize, along: f64, draw_start: isize, draw_end: isize, col: &[u8; 4]) {
    // TODO: this shit

    let horizontal = (along * game.texture_size.0 as f64) as usize;

    let mut texture_indexes: Vec<usize> = Vec::with_capacity(game.texture_size.1);
    for h in 0..game.texture_size.1 {
        texture_indexes.push(h*game.texture_size.0 + horizontal);
    }
    for s in draw_start.clamp(0, HEIGHT as isize) as usize..draw_end.clamp(0, HEIGHT as isize) as usize {
        // let travelled = (((s-draw_start) as f32 / (draw_end-draw_start) as f32) * game.texture_size.0 as f32) as usize;
        //let travelled1 = (((s as isize-draw_start as usize) as f32 / (draw_end-draw_start) as f32) * game.texture_size.0 as f32) as usize;
        let travelled = (((s as isize -draw_start)*game.texture_size.0 as isize) / ((draw_end-draw_start))).clamp(0, HEIGHT as isize) as usize;

        let pos = 1*(w)+WIDTH_USIZE * s;

        let mut c = game.texture[texture_indexes[travelled]];
        c[0] = ((c[0] as f32 / 255.0)*(col[0] as f32)) as u8;
        c[1] = ((c[1] as f32 / 255.0)*(col[1] as f32)) as u8;
        c[2] = ((c[2] as f32 / 255.0)*(col[2] as f32)) as u8;
        screen[pos*4..pos*4+4].copy_from_slice(&c);
    }
}

// Draws the map on to the screen
pub fn render_map(screen: &mut [u8], game: &Game, cell_size: usize) {
    let render_offset_w = WIDTH_USIZE  / 2 - (game.map_width  * cell_size) / 2;
    let render_offset_h = HEIGHT_USIZE / 2 - (game.map_height * cell_size) / 2;
    let render_offset = Vector2::new(render_offset_w as f64, render_offset_h as f64);
    let map_size = Vector2::new((game.map_width * cell_size) as f64, (game.map_height * cell_size) as f64);

    for (i, &cell) in game.map.iter().enumerate() {
        if cell == 0 { continue; }
        let x = (i as usize % game.map_width) * cell_size + render_offset_w;
        let y = (i as usize / game.map_width) * cell_size + render_offset_h;
        draw_rect(screen, x, y, x+cell_size, y+cell_size, &get_col(cell));
    }

    pixels_primitives::circle_filled(screen, WIDTH as i32,
        game.player.pos.x.clamp(0.0, game.map_width  as f64) * cell_size as f64 + render_offset.x,
        game.player.pos.y.clamp(0.0, game.map_height as f64) * cell_size as f64 + render_offset.y,
        cell_size as f64 / 2.0, &[0x00, 0xFF, 0x00, 0xFF]);
    draw_line(screen,
         game.player.pos * cell_size as f64 + render_offset,
        (game.player.pos + game.player.dir * game.player.mid_ray_dist) * cell_size as f64 + render_offset,
        &[0xDD, 0xDD, 0xDD, 0xFF]);
}

// A neater way of invoking pixels_primitves functions
fn draw_line(screen: &mut [u8], pos_a: Vector2<f64>, pos_b: Vector2<f64>, col: &[u8; 4]) {
    pixels_primitives::line(screen, WIDTH as i32, pos_a.x, pos_a.y, pos_b.x, pos_b.y, col);
}

// My own draw_rect function, doesn't do bounds checking but like,, just don't be stupid?? 
fn draw_rect(screen: &mut [u8], x_0: usize, y_0: usize, x_1: usize, y_1: usize, col: &[u8; 4]) {
    if x_0 > WIDTH_USIZE || x_1 > WIDTH_USIZE || y_0 > HEIGHT_USIZE || y_1 > HEIGHT_USIZE { return; }
    for y in y_0..y_1 {
        screen[(x_0+(y)*WIDTH_USIZE) * 4..(x_1+(y)*WIDTH_USIZE) * 4].copy_from_slice(&col.repeat(x_1-x_0));
    }
}

fn get_col(c: u8) -> [u8;4] {
    match c { // Modified Sweetie-16
        2 => [177,  62,  83, 0xFF], // Red
        3 => [239, 125,  87, 0xFF], // Orange
        4 => [255, 205, 117, 0xFF], // Yellow
        5 => [ 56, 183, 100, 0xFF], // Green
        6 => [ 65, 166, 246, 0xFF], // Cyan
        7 => [ 59,  93, 201, 0xFF], // Blue
        8 => [149,  82, 165, 0xFF], // Purple
        _ => [244, 244, 244, 0xFF], // White
    }
}