use std::{error::Error, mem, path::PathBuf};

use argh::FromArgs;
use image::{ImageBuffer, Rgba, imageops::flip_vertical_in_place};
use tiny::wavefront::{Obj, Tri};

const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xff];

/// Render .obj file to image.
#[derive(Debug, FromArgs)]
struct Opts {
    /// path to the model .obj file
    #[argh(positional)]
    model_path: PathBuf,
    /// image width
    #[argh(option, short = 'w', default = "1000")]
    width: u32,
    /// image height
    #[argh(option, short = 'h', default = "1000")]
    height: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = argh::from_env();

    let model = Obj::from_file(&opts.model_path)?;
    println!("Loaded model: {} vertices, {} faces", model.vertices.len(), model.faces.len());

    let mut img = ImageBuffer::from_pixel(opts.width, opts.height, Rgba(BLACK));

    for f in model.faces {
        let tri = Tri(
            model.vertices[f.vertices.0],
            model.vertices[f.vertices.1],
            model.vertices[f.vertices.2],
        );

        for l in tri.lines() {
            let x0 = (l.0.x + 1.0) * opts.width as f32 / 2.0;
            let y0 = (l.0.y + 1.0) * opts.height as f32 / 2.0;
            let x1 = (l.1.x + 1.0) * opts.width as f32 / 2.0;
            let y1 = (l.1.y + 1.0) * opts.height as f32 / 2.0;
            draw_line(x0 as u32, y0 as u32, x1 as u32, y1 as u32, &mut img, Rgba(WHITE));
        }
    }

    flip_vertical_in_place(&mut img);
    img.save("out/lesson01.png")?;

    Ok(())
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
