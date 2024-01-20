use crate::{WIDTH, HEIGHT, na, Vector2, util, game::Game};

use pixels_primitives;

const GRID_SIZE: u32 = 12;
const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

pub fn render_view(game: &Game, screen: &mut [u8]) {
    let cam_plane = Vector2::new(-game.player.dir.y, game.player.dir.x);

    // floor and ceiling
    draw_rect(screen, 0, 0, WIDTH as usize, HEIGHT as usize/2, &[0x44, 0x55, 0xDD, 0xFF]);
    draw_rect(screen, 0, HEIGHT as usize/2, WIDTH as usize, HEIGHT as usize, &[0x55, 0x55, 0x55, 0xFF]);

    for w in 0..WIDTH {
        let raycast_result = util::raycast(&game, game.player.pos, game.player.dir + (cam_plane * (w as f64 / WIDTH as f64 * 2.0 - 1.0)), 100.0);
        if let Some((cell, distance, side)) = raycast_result {

            let h = HEIGHT as f64;
            let lineheight = h / distance;
            let mut draw_start = -lineheight / 2.0 + h / 2.0;
            if draw_start < 0.0 { draw_start = 0.0 };
            let mut draw_end = lineheight / 2.0 + h / 2.0;
            if draw_end >= h { draw_end= h - 1.0 };

            let mut color = get_col(game.map[cell]);
            if side == util::RaycastSide::Y {
                color[0] /= 2;
                color[1] /= 2;
                color[2] /= 2;
            }
            draw_line(screen, Vector2::new(w as f64, draw_start), Vector2::new(w as f64, draw_end), &color);
        }
    }
}

pub fn render_map(game: &Game, screen: &mut [u8]) {

}

// Draws the map on to the screen
fn draw_map(screen: &mut [u8], game: &Game) {

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

fn get_col(c: u8) -> [u8;4] {
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