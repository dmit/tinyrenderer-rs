extern crate image;

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
    let mut img = ImageBuffer::from_pixel(1000, 1000, Rgba(BLACK));

    line(130, 200, 800, 400, &mut img, Rgba(WHITE));
    line(200, 130, 400, 800, &mut img, Rgba(RED));
    line(800, 400, 130, 200, &mut img, Rgba(RED));

    let ref mut file = File::create(&Path::new("out/test.png"))
                           .expect("Failed to create image file");
    image::ImageRgba8(img).flipv().save(file, image::PNG).expect("Failed to write image data");
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

    let dx = x1 as i64 - x0 as i64;
    let dy = y1 as i64 - y0 as i64;
    let derror2 = dy.abs() * 2;

    let mut error2 = 0i64;
    let mut y = y0;

    for x in x0..x1 {
        if steep {
            img.put_pixel(y, x, color);
        } else {
            img.put_pixel(x, y, color);
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
