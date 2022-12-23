use raylib::prelude::*;
use raylib::core::color::Color;
use crate::calc::*;
pub fn plot(f : &Grid, screen :&mut RaylibDrawHandle) {
    let height = 32i32;let width = 32i32;
    screen.clear_background(Color {r : 0, g : 0, b: 0, a: 255});
    for r in 0i32..32 { for c in 0i32..32 {
        let col = if f[r as usize][c as usize] == 0 {Color {r : 0, g : 0, b: 0, a: 255}} else {Color {r : 255, g : 255, b: 255, a: 255} };
        screen.draw_rectangle(c*width,r*height,width,height,col);
    }}
}



