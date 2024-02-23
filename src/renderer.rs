use crate::{WIDTH, HEIGHT, WIDTH_USIZE, HEIGHT_USIZE, na, Vector2, util, Game};

use na::vector;
use pixels_primitives;
// use rand::{thread_rng, Rng};

// const GRID_SIZE: u32 = 12;
// const GRID_SIZE_F64: f64 = GRID_SIZE as f64; // TODO: find out if i need this 

pub fn render_view(screen: &mut [u8], game: &mut Game, fov: f64) {
    game.player.cam_plane = Vector2::new(-game.player.dir.y, game.player.dir.x) * fov;

    // floor and ceiling
    let middle = (((HEIGHT/2) as f64 - game.player.pitch) as usize).min(HEIGHT as usize-1);

    // let light_level = game.map.lightmap[game.map.coord_to_index(&(game.player.pos.x as usize), &(game.player.pos.y as usize))];
    let light_level = 15;
    let floor_col = [
        (26 / 16) * (light_level + 1),
        (28 / 16) * (light_level + 1),
        (44 / 16) * (light_level + 1),
    255];
    let ceil_col = [
        (51 / 16) * (light_level + 1),
        (60 / 16) * (light_level + 1),
        (87 / 16) * (light_level + 1),
    255];

    draw_rect(screen, 0, 0,      WIDTH as usize, middle,            &floor_col);
    draw_rect(screen, 0, middle, WIDTH as usize, HEIGHT as usize,   &ceil_col);
    
    // TODO: make it so no-matter the aspect ratio, the map is always cubes
    // for i in 0..100 {
    for w in 0..WIDTH {
        // if w % 3 != 0 {continue;}

        let ray_direction = game.player.dir + (game.player.cam_plane * (w as f64 / WIDTH as f64 * 2.0 - 1.0));
        let raycast_result = util::raycast(game, game.player.pos, ray_direction, 500.0);
        if let Some((cell, distance, texture_along, brightness, side)) = raycast_result {
            // Calculating heights
            let head_height = (game.player.head_bob_amount.sin() / distance) * 10.0;

            // let h = HEIGHT as f64;
            // let lineheight = (h / distance);// * (1.0/ASPECT_RATIO);
            // let mut draw_start = -lineheight / 2.0 + h / 2.0 + head_height - game.player.pitch;
            // if draw_start < 0.0 { draw_start = 0.0 };
            // let mut draw_end = lineheight / 2.0 + h / 2.0 + head_height - game.player.pitch;
            // if draw_end > h { draw_end = h };

            // TODO: Make this better
            let h = HEIGHT as isize;
            let lineheight = (h as f64 / (distance*fov)) as isize;// * (1.0/ASPECT_RATIO);
            let line_start = -lineheight / 2 + h / 2 + (head_height - game.player.pitch) as isize;
            // if draw_start < 0 { draw_start = 0 };
            let line_end   =  lineheight / 2 + h / 2 + (head_height - game.player.pitch) as isize;
            // if draw_end > h { draw_end = h };

            

            // Color stuff
            // let mut color = get_col(game.map[cell]-1);
            let mut color = [255; 4];

            color[0] = (color[0] as f32 * (brightness as f32 / 255.0)) as u8;
            color[1] = (color[1] as f32 * (brightness as f32 / 255.0)) as u8;
            color[2] = (color[2] as f32 * (brightness as f32 / 255.0)) as u8;

            let offset = match game.map.get(cell).kind == 1 {
                true => {
                    if side == 0 {
                        match ray_direction.x.is_sign_positive() {
                            true  => -1,
                            false =>  1,
                        }
                    } else {
                        match ray_direction.y.is_sign_positive() {
                            true  => -(game.map.width as isize),
                            false =>   game.map.width as isize,
                        }
                    }
                }
                _ => 0
            };
            let light_level = game.map.lightmap[cell.saturating_add_signed(offset)];
            color[0] = (color[0] / 16) * (light_level + 1);
            color[1] = (color[1] / 16) * (light_level + 1);
            color[2] = (color[2] / 16) * (light_level + 1);

            // draw_line(screen, Vector2::new(w as f64, draw_start), Vector2::new(w as f64, draw_end), &color);
            draw_slice(screen, game, w as usize, texture_along, line_start, line_end, &color, game.map.cells[cell].texture_index.into());

            if w == WIDTH / 2 { game.player.mid_ray_dist = distance }
        }
    }
    // Draw 'crosshair'
    draw_rect(screen, (WIDTH/2 - 2) as usize, (HEIGHT/2 - 2) as usize, (WIDTH/2 + 2) as usize, (HEIGHT/2 + 2) as usize, &[0xFF, 0xAA, 0x00, 0xFF]);
}

// Draws a slice of a raycast
fn draw_slice(screen: &mut [u8], game: &Game, screen_column: usize, along: f64, line_start: isize, line_end: isize, col: &[u8; 4], texture_index: usize) {
    // TODO: better way to do this
    let draw_start = line_start.clamp(0, HEIGHT as isize) as usize;
    let draw_end   = line_end  .clamp(0, HEIGHT as isize) as usize;

    let tex = &game.textures[texture_index];

    let column: usize = (along.rem_euclid(1.0) * (tex.width) as f64).floor() as usize; // TODO: Don't use 'as' here

    let slice_begin = ( column    * tex.height) * 3;
    let slice_end   = ((column+1) * tex.height) * 3;
    let column_slice = &tex.data[slice_begin..slice_end];
    // Make a vec out of it so we can recolour it
    let mut column_vec = Vec::from(column_slice);
    // Recolour the whole column
    for (i, p) in column_vec.iter_mut().enumerate() {
        *p = ((u16::from(*p) * u16::from(col[i % 3])) / 255) as u8;
    }

    for h in draw_start..draw_end {
        // How far up the column we are
        // Not a fan of all the floats and casting, there's gotta be a way to do this with just ints, but oh well..!
        let ascent = (h as f32 - line_start as f32) / (line_end as f32 - line_start as f32);

        let row = (ascent * tex.height as f32).floor() as usize*3;
        let mut pixel = [0, 0, 0, 255];
        pixel[0..3].copy_from_slice(&column_vec[row..row+3]);
        pixel[3] = col[3];

        let pos = 1*(screen_column)+WIDTH_USIZE * h;
        screen[pos*4..pos*4+4].copy_from_slice(&pixel);
    }
}

// Draws the map on to the screen
pub fn render_map(screen: &mut [u8], game: &Game, cell_size: usize) {
    let render_offset_w = WIDTH_USIZE  / 2 - (game.map.width  * cell_size) / 2;
    let render_offset_h = HEIGHT_USIZE / 2 - (game.map.height * cell_size) / 2;
    let render_offset = Vector2::new(render_offset_w as f64, render_offset_h as f64);
    // let map_size = Vector2::new((game.map.width * cell_size) as f64, (game.map.height * cell_size) as f64);

    for i in 0..game.map.width {
        draw_line(screen,
        Vector2::new((i*cell_size) as f64, 0.0) + render_offset,
        Vector2::new((i*cell_size) as f64, (game.map.height * cell_size) as f64) + render_offset,
        &[0x55, 0x55, 0x55, 0xFF]);
    }
    for i in 0..game.map.height {
        draw_line(screen,
        Vector2::new(0.0, (i*cell_size) as f64) + render_offset,
        Vector2::new((game.map.width * cell_size) as f64, (i*cell_size) as f64) + render_offset,
        &[0x55, 0x55, 0x55, 0xFF]);
    }

    // for (i, &cell) in game.map.cells.clo.enumerate() {
    //     if cell.kind == 0 { continue; }
    //     let x = (i as usize % game.map.width) * cell_size + render_offset_w;
    //     let y = (i as usize / game.map.width) * cell_size + render_offset_h;
    //     draw_rect(screen, x, y, x+cell_size, y+cell_size, &get_col(cell.kind.saturating_sub(1)));
    // }
    

    pixels_primitives::circle_filled(screen, WIDTH as i32,
        game.player.pos.x.clamp(0.0, game.map.width  as f64) * cell_size as f64 + render_offset.x,
        game.player.pos.y.clamp(0.0, game.map.height as f64) * cell_size as f64 + render_offset.y,
        crate::game::player::PLAYER_RADIUS * cell_size as f64, &[0x00, 0xFF, 0x00, 0xFF]);
    draw_line(screen,
         game.player.pos * cell_size as f64 + render_offset,
        (game.player.pos + game.player.dir * game.player.mid_ray_dist) * cell_size as f64 + render_offset,
        &[0xDD, 0xDD, 0xDD, 0xFF]);

    for seg in &game.map.collision {
        draw_line(screen,
            vector![seg[0].x, seg[0].y] * cell_size as f64 + render_offset,
            vector![seg[1].x, seg[1].y] * cell_size as f64 + render_offset,
            &[0xAA, 0xAA, 0xAA, 0xFF]);
        for p in seg {
            pixels_primitives::circle_filled(screen, WIDTH as i32,
                p.x * cell_size as f64 + render_offset.x,
                p.y * cell_size as f64 + render_offset.y,
                cell_size as f64 / 6.0, &[0xFF, 0xFF, 0xFF, 0xFF]);
        }

    }
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