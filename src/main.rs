use std::fs::File;
use std::io::Write;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const COLS: usize = 8;
const ROWS: usize = 6;
const CELL_WIDTH: usize = WIDTH / COLS;
const CELL_HEIGHT: usize = HEIGHT / ROWS;

fn tony_fill(pixels: &mut Vec<usize>, width: usize, height: usize, color: usize) {
    for i in 0..width*height {
        pixels[i] = color;
    }
}

fn tony_fill_rect(pixels: &mut Vec<usize>, width: usize, height: usize, x: usize, y: usize, w: usize, h: usize, color: usize) {
    for dy in 0..h {
        let y_0 = y + dy;
        if y_0 >= height { continue; }

        for dx in 0..w {
            let x_0 = x + dx;
            if x_0 >= width { continue; }

            pixels[y_0 * width + x_0] = color;
        }
    }
}

fn tony_fill_circle(pixels: &mut Vec<usize>, width: usize, height: usize, x: usize, y: usize, r: usize, color: usize) {
    let x1 = x - r;
    let y1 = y - r;
    let x2 = x + r;
    let y2 = y + r;
    
    let _cx = x1;
    let _cy = y1;
    for cy in 0..=y2 {
        if cy < height {
            for cx in 0..=x2 {
                if x < width {
                    let dx = cx - x;
                    let dy = cy - y;
                    if dx*dx + dy*dy <= r*r {
                        pixels[cy * width + cx] = color;
                    }
                }
            }
        }
    }
}

fn tony_draw_line(
    pixels: &mut Vec<usize>,
    width: usize,
    height: usize,
    mut x0: isize,
    mut y0: isize,
    x1: isize,
    y1: isize,
    color: usize,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < width && (y0 as usize) < height {
            pixels[y0 as usize * width + x0 as usize] = color;
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn tony_checker() {
    let mut pixels: Vec<usize> = vec![0; WIDTH*HEIGHT];
    let mut color = 1;
    tony_fill(&mut pixels, WIDTH, HEIGHT, 0xFF202020);
    for y in 0..ROWS {
        for x in 0..COLS {
            if (x + y) % 2 == 0 {
                color = 0xFF0000FF;
            } else {
                color = 0xFF2020FF;
            }
            tony_fill_rect(&mut pixels, WIDTH, HEIGHT, x*CELL_WIDTH, y*CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT, color);
        }
    }
    tony_save_to_ppm(&pixels, WIDTH, HEIGHT, "tony_checker.ppm");
}

fn tony_japan() {
    let mut pixels: Vec<usize> = vec![0; WIDTH*HEIGHT];
    tony_fill(&mut pixels, WIDTH, HEIGHT, 0xFFFFFFFF);
    tony_fill_circle(&mut pixels, WIDTH, HEIGHT, WIDTH/2, HEIGHT/2, 150, 0xFF0000FF);
    tony_save_to_ppm(&pixels, WIDTH, HEIGHT, "tony_japan.ppm");
}

fn tony_lines() {
    let mut pixels: Vec<usize> = vec![0; WIDTH * HEIGHT];
    tony_fill(&mut pixels, WIDTH, HEIGHT, 0xFFFFFFFF);

    tony_draw_line(
        &mut pixels,
        WIDTH,
        HEIGHT,
        0,
        0,
        WIDTH as isize - 1,
        HEIGHT as isize - 1,
        0xFF0000FF,
    );

    tony_draw_line(
        &mut pixels,
        WIDTH,
        HEIGHT,
        0,
        HEIGHT as isize - 1,
        WIDTH as isize - 1,
        0,
        0xFF0000FF,
    );

    tony_save_to_ppm(&pixels, WIDTH, HEIGHT, "tony_lines.ppm");
}

fn tony_save_to_ppm(pixels: &Vec<usize>, width: usize, height: usize, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write_all(header.as_bytes()).unwrap();

    let mut buffer = Vec::with_capacity(width * height * 3);

    for &pixel in pixels {
        let r = ((pixel >> 8*0) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel >> 8*2) as u8;
        buffer.extend_from_slice(&[r, g, b]);
    }

    file.write_all(&buffer).unwrap();
}

fn main() {
    tony_japan();
    tony_checker();
    tony_lines();
}
