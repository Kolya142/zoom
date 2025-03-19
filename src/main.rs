use macroquad::prelude::*;
use scrap::{Capturer, Display, Frame};

#[macroquad::main("Zoom")]
async fn main() {
    let mut displays = Display::all().expect("Cannot open display");
    let mut frames: Vec<Vec<u8>> = vec![];
    let mut width: usize = 0;
    let mut height: usize = 0;
    for i in 0..displays.len() {
        let mut capturer: Capturer = Capturer::new(displays.remove(0)).expect("Cannot create capturer");
        
        width = capturer.width();
        height = capturer.height();
        let frame: Vec<u8> = capturer.frame().expect("Cannot screenshot").to_vec();
        frames.push(frame);
    }

    let (mut cx, mut cy, mut cw, mut ch) = (0 as i32, 0 as i32, width as i32, height as i32);
    let mut framei: usize = frames.len() - 1;
    let mut changed: bool = true;
    loop {
        let frame_s = frames[framei].len();
        let (mx, my) = mouse_position();
        if cx != mx as i32 * 3 - (width as i32) {
            cx = mx as i32 * 3 - (width as i32);
            changed = true;
        }
        if cy != my as i32 * 3 - (height as i32) {
            cy = my as i32 * 3 - (height as i32);
            changed = true;
        }
        let (wwidth, wheight) = (screen_width(), screen_height());
        let xcof = (cw as f64) / (wwidth as f64);
        let ycof = (ch as f64) / (wheight as f64);
        for j in 0..(wheight as u64 / 4) { // TODO: redraw only if changed
            for i in 0..(wwidth as u64 / 4) {
                let (x, y) = ((cx as f64) + (i as f64) * xcof, (cy as f64) + (j as f64) * ycof);
                let index = ((y as usize) * width + (x as usize)) * 4; // u32 size
                if index < frame_s {
                    let color: Color = Color {
                        r: (frames[framei][index + 2] as f32) / 256.,
                        g: (frames[framei][index + 1] as f32) / 256.,
                        b: (frames[framei][index + 0] as f32) / 256.,
                        a: 1.
                    };
                    draw_rectangle(i as f32 * 4., j as f32 * 4., 4., 4., color);
                }
            }
        }
        let step = 40;
        if is_key_down(KeyCode::S) {
            cw -= step;
            ch -= step;
            changed = true;
        }
        if is_key_down(KeyCode::D) {
            cw += step;
            ch += step;
            changed = true;
        }
        if is_key_down(KeyCode::E) {
            framei = (framei + 1) % frames.len();
            changed = true;
        }
        if is_key_down(KeyCode::Q) {
            break;
        }
        next_frame().await
    }
}
