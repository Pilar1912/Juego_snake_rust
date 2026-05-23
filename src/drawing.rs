
use piston_window::Context;
use piston_window::G2d;
use piston_window::{rectangle, ellipse, line};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;
const SNAKE_COLOR: Color = [0.2, 0.7, 0.3, 1.0]; // Verde más realista
const APPLE_COLOR: Color = [0.9, 0.2, 0.1, 1.0]; // Rojo manzana
const APPLE_SHINE: Color = [1.0, 0.4, 0.2, 0.8]; // Brillo de la manzana
const LEAF_COLOR: Color = [0.2, 0.6, 0.2, 1.0]; // Verde oscuro para la hoja

pub fn to_gui_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_gui_coord_u32(game_coord: i32) -> u32 {
    to_gui_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);

    rectangle(color, [gui_x, gui_y,
            BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_snake_segment(x: i32, y: i32, is_head: bool, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);
    let padding = 2.0;
    
    // Cuerpo principal de la serpiente
    rectangle(
        SNAKE_COLOR,
        [gui_x + padding, gui_y + padding, BLOCK_SIZE - 2.0 * padding, BLOCK_SIZE - 2.0 * padding],
        con.transform,
        g,
    );
    
    // Borde oscuro para más profundidad
    let border_color = [0.1, 0.5, 0.15, 1.0];
    rectangle(
        border_color,
        [gui_x + padding, gui_y + padding, BLOCK_SIZE - 2.0 * padding, 1.5],
        con.transform,
        g,
    );
    rectangle(
        border_color,
        [gui_x + padding, gui_y + padding, 1.5, BLOCK_SIZE - 2.0 * padding],
        con.transform,
        g,
    );
    
    // Brillo para efecto 3D
    let shine_color = [0.4, 0.85, 0.5, 0.6];
    rectangle(
        shine_color,
        [gui_x + padding + 2.0, gui_y + padding + 2.0, BLOCK_SIZE - 4.0 * padding, 1.0],
        con.transform,
        g,
    );
    
    // Si es la cabeza, dibuja los ojos
    if is_head {
        draw_snake_eyes(gui_x, gui_y, con, g);
    }
}

pub fn draw_snake_eyes(gui_x: f64, gui_y: f64, con: &Context, g: &mut G2d) {
    let eye_radius = 2.0;
    let pupil_radius = 1.0;
    let eye_color = [1.0, 1.0, 1.0, 1.0]; // Blanco
    let pupil_color = [0.0, 0.0, 0.0, 1.0]; // Negro
    
    // Ojo izquierdo
    let left_eye_x = gui_x + 8.0;
    let left_eye_y = gui_y + 7.0;
    ellipse(eye_color, [left_eye_x - eye_radius, left_eye_y - eye_radius, eye_radius * 2.0, eye_radius * 2.0], con.transform, g);
    ellipse(pupil_color, [left_eye_x - pupil_radius, left_eye_y - pupil_radius, pupil_radius * 2.0, pupil_radius * 2.0], con.transform, g);
    
    // Ojo derecho
    let right_eye_x = gui_x + 17.0;
    let right_eye_y = gui_y + 7.0;
    ellipse(eye_color, [right_eye_x - eye_radius, right_eye_y - eye_radius, eye_radius * 2.0, eye_radius * 2.0], con.transform, g);
    ellipse(pupil_color, [right_eye_x - pupil_radius, right_eye_y - pupil_radius, pupil_radius * 2.0, pupil_radius * 2.0], con.transform, g);
}

pub fn draw_apple(x: i32, y: i32, con: &Context, g: &mut G2d) {
    draw_apple_with_color(x, y, APPLE_COLOR, APPLE_SHINE, con, g);
}

pub fn draw_apple_with_color(x: i32, y: i32, body_color: Color, shine_color: Color, con: &Context, g: &mut G2d) {
    let gui_x = to_gui_coord(x);
    let gui_y = to_gui_coord(y);
    let padding = 3.0;
    let apple_radius = (BLOCK_SIZE - 2.0 * padding) / 2.0;
    let center_x = gui_x + BLOCK_SIZE / 2.0;
    let center_y = gui_y + BLOCK_SIZE / 2.0;
    
    // Cuerpo de la manzana (círculo)
    ellipse(
        body_color,
        [center_x - apple_radius, center_y - apple_radius, apple_radius * 2.0, apple_radius * 2.0],
        con.transform,
        g,
    );
    
    // Brillo de la manzana (círculo en la esquina superior izquierda)
    let shine_radius = apple_radius * 0.35;
    ellipse(
        shine_color,
        [center_x - apple_radius + 3.0 - shine_radius, center_y - apple_radius + 3.0 - shine_radius, shine_radius * 2.0, shine_radius * 2.0],
        con.transform,
        g,
    );
    
    // Tallo (línea vertical marrón)
    let stem_color = [0.5, 0.3, 0.1, 1.0];
    line(stem_color, 1.0, [center_x, center_y - apple_radius, center_x, center_y - apple_radius - 4.0], con.transform, g);
    
    // Hoja verde
    draw_apple_leaf(center_x + 2.0, center_y - apple_radius - 2.0, con, g);
}

pub fn draw_apple_leaf(stem_x: f64, stem_y: f64, con: &Context, g: &mut G2d) {
    let leaf_width = 6.0;
    let leaf_height = 5.0;
    
    // Forma de hoja aproximada usando un círculo ligeramente desplazado
    ellipse(
        LEAF_COLOR,
        [stem_x - leaf_width / 2.0, stem_y - leaf_height, leaf_width, leaf_height],
        con.transform,
        g,
    );
    
    // Nervio de la hoja
    let vein_color = [0.1, 0.4, 0.1, 0.8];
    line(vein_color, 0.5, [stem_x, stem_y - leaf_height, stem_x, stem_y], con.transform, g);
}

pub fn draw_rectange(color: Color, start_x: i32, start_y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let gui_start_x = to_gui_coord(start_x);
    let gui_start_y = to_gui_coord(start_y);

    rectangle(color, [gui_start_x, gui_start_y,
            BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g);
}
