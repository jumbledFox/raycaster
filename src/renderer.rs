use crate::{WIDTH, HEIGHT, na, Vector2, util, game::{Game, player}, ASPECT_RATIO};

use pixels_primitives;

const GRID_SIZE: u32 = 12;
const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

pub fn render_view(game: &mut Game, screen: &mut [u8]) {
    game.player.cam_plane = Vector2::new(-game.player.dir.y, game.player.dir.x);

    // floor and ceiling
    let middle = (((HEIGHT/2) as f64 - game.player.pitch) as usize).min(HEIGHT as usize-1);
    draw_rect(screen, 0, 0,      WIDTH as usize, middle,            &[ 26,  28,  44, 0xFF]);
    draw_rect(screen, 0, middle, WIDTH as usize, HEIGHT as usize,   &[ 51,  60,  87, 0xFF]);

    // TODO: make it so no-matter the aspect ratio, the map is always cubes
    for w in 0..WIDTH {
        let ray_direction = game.player.dir + (game.player.cam_plane * (w as f64 / WIDTH as f64 * 2.0 - 1.0));
        let raycast_result = util::raycast(&game, game.player.pos, ray_direction, 100.0);
        if let Some((cell, distance, side)) = raycast_result {

            let head_bob = (game.player.head_bob_amount.sin() * 5.0) / (distance / 5.0);
            let h = HEIGHT as f64;
            let lineheight = (h / distance);// * (1.0/ASPECT_RATIO);
            let mut draw_start = -lineheight / 2.0 + h / 2.0 + head_bob - game.player.pitch;
            if draw_start < 0.0 { draw_start = 0.0 };
            let mut draw_end = lineheight / 2.0 + h / 2.0 + head_bob - game.player.pitch;
            if draw_end > h { draw_end = h };

            let mut color = get_col(game.map[cell]);
            if side == util::RaycastSide::Y {
                color[0] = (color[0] as f32 * 0.7) as u8;
                color[1] = (color[1] as f32 * 0.7) as u8;
                color[2] = (color[2] as f32 * 0.7) as u8;
            }
            let real_dist = distance / ray_direction.angle(&game.player.dir).cos();
            color[0] = (color[0] as f64 / (real_dist.max(1.0) / 10.0).max(1.0)) as u8;
            color[1] = (color[1] as f64 / (real_dist.max(1.0) / 10.0).max(1.0)) as u8;
            color[2] = (color[2] as f64 / (real_dist.max(1.0) / 10.0).max(1.0)) as u8;
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
    // match c {
    //     2 => [0xFF, 0x00, 0x00, 0xFF], // Red
    //     3 => [0xFF, 0xAA, 0x00, 0xFF], // Orange
    //     4 => [0xFF, 0xFF, 0x00, 0xFF], // Yellow
    //     5 => [0x00, 0xFF, 0x00, 0xFF], // Green
    //     6 => [0x00, 0xFF, 0xFF, 0xFF], // Cyan
    //     7 => [0x00, 0x00, 0xFF, 0xFF], // Blue
    //     8 => [0xFF, 0x00, 0xFF, 0xFF], // Purple
    //     _ => [0xFF, 0xFF, 0xFF, 0xFF], // White
    // }
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