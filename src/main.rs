#![feature(test)]

extern crate image;
extern crate test;

use std::fs::File;
use std::mem;
use std::path::Path;

use image::{ImageBuffer, Rgba};

type Color = Rgba<u8>;
type Image = ImageBuffer<Color, Vec<u8>>;

const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xff];
const RED: [u8; 4] = [0xff, 0x00, 0x00, 0xff];

fn main() {
    let mut img = ImageBuffer::new(1000, 1000);

    fill(&mut img, Rgba(BLACK));
    line(130, 200, 800, 400, &mut img, Rgba(WHITE));
    line(200, 130, 400, 800, &mut img, Rgba(RED));
    line(800, 400, 130, 200, &mut img, Rgba(RED));

    let ref mut file = File::create(&Path::new("out/test.png"))
                           .expect("Failed to create image file");
    image::ImageRgba8(img).flipv().save(file, image::PNG).expect("Failed to write image data");
}

fn fill<T>(img: &mut ImageBuffer<T, Vec<u8>>, color: T)
    where T: image::Pixel<Subpixel = u8> + 'static
{
    for p in img.pixels_mut() {
        *p = color;
    }
}

fn line<T>(mut x0: u32,
           mut y0: u32,
           mut x1: u32,
           mut y1: u32,
           img: &mut ImageBuffer<T, Vec<u8>>,
           color: T)
    where T: image::Pixel<Subpixel = u8> + 'static
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

    for x in x0..x1 {
        let t = (x - x0) as f64 / (x1 - x0) as f64;
        let y = (y0 as f64 * (1.0 - t) + y1 as f64 * t) as u32;

        if steep {
            img.put_pixel(y, x, color);
        } else {
            img.put_pixel(x, y, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
}
