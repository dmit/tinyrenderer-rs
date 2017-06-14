extern crate image;

extern crate tiny;

use tiny::wavefront::{Obj, Tri};

use std::env;
use std::fs::File;
use std::mem;
use std::path::Path;

use image::{ImageBuffer, Rgba};

const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xff];

fn main() {
    let model = Obj::from_file(Path::new(
        &env::args().nth(1).expect("Specify path to model"),
    )).expect("Failed to load model");
    println!(
        "Loaded model, vertices: {}, faces: {}",
        model.vertices.len(),
        model.faces.len()
    );

    let width = env::args().nth(2).map_or(1000, |s| {
        s.parse::<u32>().expect(&format!("Invalid width: {}", s))
    });
    let height = env::args().nth(3).map_or(width, |s| {
        s.parse::<u32>().expect(&format!("Invalid height: {}", s))
    });

    let mut img = ImageBuffer::from_pixel(width, height, Rgba(BLACK));

    for f in model.faces {
        let tri = Tri(
            model.vertices[f.vertices.0],
            model.vertices[f.vertices.1],
            model.vertices[f.vertices.2],
        );

        for l in tri.lines() {
            let x0 = (l.0.x + 1.0) * width as f32 / 2.0;
            let y0 = (l.0.y + 1.0) * height as f32 / 2.0;
            let x1 = (l.1.x + 1.0) * width as f32 / 2.0;
            let y1 = (l.1.y + 1.0) * height as f32 / 2.0;
            draw_line(
                x0 as u32,
                y0 as u32,
                x1 as u32,
                y1 as u32,
                &mut img,
                Rgba(WHITE),
            );
        }
    }

    let mut file =
        File::create(&Path::new("out/lesson01.png")).expect("Failed to create image file");
    image::ImageRgba8(img)
        .flipv()
        .save(&mut file, image::PNG)
        .expect("Failed to write image data");
}

fn draw_line<T>(
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    img: &mut ImageBuffer<T, Vec<u8>>,
    color: T,
) where
    T: image::Pixel<Subpixel = u8> + 'static,
{
    let steep = (x0 as i64 - x1 as i64).abs() < (y0 as i64 - y1 as i64).abs();
    if steep {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 as i64 - x0 as i64;
    let dy = y1 as i64 - y0 as i64;
    let derror2 = dy.abs() * 2;

    let mut error2 = 0i64;
    let mut y = y0;

    for x in x0..x1 {
        if x < img.width() && y < img.height() {
            if steep {
                img.put_pixel(y, x, color);
            } else {
                img.put_pixel(x, y, color);
            }
        }
        error2 += derror2;

        if error2 > dx {
            if y1 > y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error2 -= dx * 2;
        }
    }
}
